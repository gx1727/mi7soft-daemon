use crate::error::DaemonError;
use std::fs::File;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

/// PID file manager with exclusive lock
pub struct PidFile {
    path: String,
    _file: Option<File>, // Keep file handle to maintain lock
    created: AtomicBool,
}

impl PidFile {
    /// Create a new PID file manager
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
            _file: None,
            created: AtomicBool::new(false),
        }
    }

    /// Acquire exclusive lock and write PID
    pub fn acquire_lock(&mut self) -> Result<(), DaemonError> {
        #[cfg(unix)]
        {
            self.acquire_lock_unix()
        }

        #[cfg(not(unix))]
        {
            self.acquire_lock_windows()
        }
    }

    /// Release lock and remove PID file
    pub fn release_lock(&mut self) -> Result<(), DaemonError> {
        if !self.created.load(Ordering::Relaxed) {
            return Ok(());
        }

        // Close the file handle first to release the lock
        self._file = None;

        std::fs::remove_file(&self.path)
            .map_err(|e| DaemonError::PidFile(format!("Failed to remove PID file: {}", e)))?;

        self.created.store(false, Ordering::Relaxed);
        Ok(())
    }

    /// Read PID from file (for external checking)
    pub fn read_pid(&self) -> Result<Option<u32>, DaemonError> {
        if !Path::new(&self.path).exists() {
            return Ok(None);
        }

        let content = std::fs::read_to_string(&self.path)
            .map_err(|e| DaemonError::PidFile(format!("Failed to read PID file: {}", e)))?;

        let pid_str = content.trim();
        if pid_str.is_empty() {
            return Ok(None);
        }

        let pid: u32 = pid_str
            .parse()
            .map_err(|_| DaemonError::PidFile("Invalid PID in file".to_string()))?;

        Ok(Some(pid))
    }

    /// Write current PID to file
    fn write_pid(&mut self) -> Result<(), DaemonError> {
        let pid = std::process::id();

        let content = format!("{}\n", pid);

        // Create parent directories if they don't exist
        if let Some(parent) = Path::new(&self.path).parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                DaemonError::PidFile(format!("Failed to create PID directory: {}", e))
            })?;
        }

        let mut file = File::create(&self.path)
            .map_err(|e| DaemonError::PidFile(format!("Failed to create PID file: {}", e)))?;

        use std::io::Write;
        file.write_all(content.as_bytes())
            .map_err(|e| DaemonError::PidFile(format!("Failed to write PID: {}", e)))?;

        file.sync_all()
            .map_err(|e| DaemonError::PidFile(format!("Failed to sync PID file: {}", e)))?;

        Ok(())
    }

    #[cfg(unix)]
    fn acquire_lock_unix(&mut self) -> Result<(), DaemonError> {
        use std::fs::OpenOptions;
        use std::os::unix::fs::OpenOptionsExt;

        // Check if process already running
        if let Some(pid) = self.read_pid()? {
            if self.is_process_alive(pid) {
                return Err(DaemonError::AlreadyRunning {
                    name: "daemon".to_string(),
                    pid,
                });
            }
            // Process died, clean up stale lock
            let _ = std::fs::remove_file(&self.path);
        }

        // Create file with exclusive lock (O_CREAT | O_EXCL)
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o644) // rw-r--r--
            .open(&self.path)
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::AlreadyExists {
                    DaemonError::LockFile("Another instance is already running".to_string())
                } else {
                    DaemonError::PidFile(format!("Failed to create PID file: {}", e))
                }
            })?;

        // Write our PID
        use std::io::Write;
        let pid = std::process::id();
        writeln!(file, "{}", pid)
            .map_err(|e| DaemonError::PidFile(format!("Failed to write PID: {}", e)))?;

        self._file = Some(file);
        self.created.store(true, Ordering::Relaxed);
        Ok(())
    }

    #[cfg(not(unix))]
    fn acquire_lock_windows(&mut self) -> Result<(), DaemonError> {
        // Windows: use file-based locking
        use std::fs::OpenOptions;

        // Check if process already running
        if let Some(pid) = self.read_pid()? {
            if self.is_process_alive(pid) {
                return Err(DaemonError::AlreadyRunning {
                    name: "daemon".to_string(),
                    pid,
                });
            }
        }

        // Try to create or open file
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.path)
            .map_err(|e| DaemonError::PidFile(format!("Failed to open PID file: {}", e)))?;

        // On Windows, we rely on the file handle to prevent multiple instances
        // A better approach would be to use Windows named mutex, but file handle is simpler
        self.write_pid()?;

        self._file = Some(file);
        self.created.store(true, Ordering::Relaxed);
        Ok(())
    }

    /// Check if process is still running
    #[cfg(unix)]
    fn is_process_alive(&self, pid: u32) -> bool {
        use nix::sys::signal::{kill, Signal};
        use nix::unistd::Pid;

        // Send signal 0 - no signal is actually sent, just check if process exists
        kill(Pid::from_raw(pid as i32), None).is_ok()
    }

    #[cfg(not(unix))]
    fn is_process_alive(&self, _pid: u32) -> bool {
        // Windows: Not implemented (Windows-only builds are for dev, not for running daemon)
        false
    }
}

impl Drop for PidFile {
    fn drop(&mut self) {
        // Best effort cleanup - ignore errors
        let _ = self.release_lock();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_pidfile_creation() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap().to_string();

        let mut pidfile = PidFile::new(&path);
        pidfile.acquire_lock().unwrap();

        let pid = pidfile.read_pid().unwrap();
        assert!(pid.is_some());
        assert_eq!(pid.unwrap(), std::process::id());
    }

    #[test]
    fn test_pidfile_duplicate() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap().to_string();

        let mut pidfile1 = PidFile::new(&path);
        pidfile1.acquire_lock().unwrap();

        let mut pidfile2 = PidFile::new(&path);
        let result = pidfile2.acquire_lock();
        assert!(result.is_err());
    }

    #[test]
    fn test_pidfile_release() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap().to_string();

        let mut pidfile = PidFile::new(&path);
        pidfile.acquire_lock().unwrap();
        pidfile.release_lock().unwrap();

        assert!(!Path::new(&path).exists());
    }
}
