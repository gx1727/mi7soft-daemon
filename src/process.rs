use crate::config::ProcessConfig;
use crate::error::DaemonError;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::process::Child;

pub struct ProcessEntry {
    pub name: String,
    pub pid: u32,
    pub start_time: u64,
    pub config: ProcessConfig,
}

impl ProcessEntry {
    pub fn new(name: String, pid: u32, config: ProcessConfig) -> Self {
        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Self { name, pid, start_time, config }
    }
    
    pub fn uptime(&self) -> u64 {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        now.saturating_sub(self.start_time)
    }
}

pub struct ProcessStatus {
    pub name: String,
    pub pid: u32,
    pub state: ProcessState,
    pub uptime: u64,
    pub memory: Option<u64>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProcessState { Running, Stopped, Dead, Unknown }
pub struct ProcessManager {
    registry: HashMap<String, Vec<ProcessEntry>>,
}

impl ProcessManager {
    pub fn new() -> Self {
        Self { registry: HashMap::new() }
    }

    pub async fn spawn(&mut self, config: &ProcessConfig) -> Result<u32, DaemonError> {
        if let Some(max) = config.max_instances {
            if let Some(entries) = self.registry.get(&config.name) {
                if entries.len() >= max {
                    return Err(DaemonError::StartFailed {
                        name: config.name.clone(),
                        reason: format!("Max instances ({}) reached", max),
                    });
                }
            }
        }

        let mut cmd = tokio::process::Command::new(&config.command);
        cmd.args(&config.args);
        if let Some(ref wd) = config.working_directory {
            cmd.current_dir(wd);
        }
        for (k, v) in &config.environment {
            cmd.env(k, v);
        }
        
        let child = cmd.spawn().map_err(|e| DaemonError::StartFailed {
            name: config.name.clone(),
            reason: format!("Failed: {}", e),
        })?;
        
        let pid = child.id().unwrap() as u32;
        self.registry.entry(config.name.clone())
            .or_insert_with(Vec::new)
            .push(ProcessEntry::new(config.name.clone(), pid, config.clone()));
        Ok(pid)
    }
    pub async fn stop(&mut self, name: &str) -> Result<Vec<u32>, DaemonError> {
        if let Some(entries) = self.registry.get(name) {
            if entries.is_empty() {
                return Err(DaemonError::NotRunning { name: name.to_string() });
            }
            let pids: Vec<u32> = entries.iter().map(|e| e.pid).collect();
            let mut stopped = Vec::new();
            for pid in pids {
                if let Ok(p) = self.stop_by_pid(pid).await { stopped.push(p); }
            }
            self.registry.remove(name);
            Ok(stopped)
        } else {
            Err(DaemonError::NotRunning { name: name.to_string() })
        }
    }

    pub async fn stop_by_pid(&self, pid: u32) -> Result<u32, DaemonError> {
        #[cfg(unix)]
        {
            use nix::sys::signal::{self, Signal};
            use nix::unistd::Pid;
            use std::time::Duration;
            
            let _ = signal::kill(Pid::from_raw(pid as i32), Signal::SIGTERM);
            for _ in 0..50 {
                tokio::time::sleep(Duration::from_millis(100)).await;
                if !self.is_process_alive(pid) { return Ok(pid); }
            }
            signal::kill(Pid::from_raw(pid as i32), Signal::SIGKILL)
                .map_err(|e| DaemonError::StopFailed {
                    name: format!("PID {}", pid),
                    reason: format!("SIGKILL failed: {}", e),
                })?;
            tokio::time::sleep(Duration::from_millis(100)).await;
            Ok(pid)
        }
        #[cfg(not(unix))]
        {
            Err(DaemonError::StopFailed { name: format!("PID {}", pid), reason: "Not supported".to_string() })
        }
    }
    pub async fn restart(&mut self, config: &ProcessConfig) -> Result<Vec<u32>, DaemonError> {
        self.stop(&config.name).await.ok();
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        let pid = self.spawn(config).await?;
        Ok(vec![pid])
    }

    pub fn status(&self, name: &str) -> Result<Vec<ProcessStatus>, DaemonError> {
        if let Some(entries) = self.registry.get(name) {
            let mut statuses = Vec::new();
            for entry in entries {
                let state = if self.is_process_alive(entry.pid) { ProcessState::Running } else { ProcessState::Dead };
                statuses.push(ProcessStatus {
                    name: entry.name.clone(),
                    pid: entry.pid,
                    state,
                    uptime: entry.uptime(),
                    memory: self.get_process_memory(entry.pid),
                });
            }
            Ok(statuses)
        } else {
            Err(DaemonError::NotRunning { name: name.to_string() })
        }
    }

    pub fn status_all(&self) -> Vec<ProcessStatus> {
        let mut all = Vec::new();
        for name in self.registry.keys() {
            if let Ok(s) = self.status(name) { all.extend(s); }
        }
        all
    }
    pub fn cleanup_dead(&mut self) -> Vec<String> {
        let mut dead_names = Vec::new();
        let mut to_remove = Vec::new();
        for (name, entries) in &self.registry {
            for entry in entries {
                if !self.is_process_alive(entry.pid) {
                    to_remove.push((name.clone(), entry.pid));
                    dead_names.push(name.clone());
                }
            }
        }
        for (name, pid) in to_remove {
            if let Some(entries) = self.registry.get_mut(&name) {
                entries.retain(|e| e.pid != pid);
                if entries.is_empty() { self.registry.remove(&name); }
            }
        }
        dead_names
    }

    pub fn process_names(&self) -> Vec<String> {
        self.registry.keys().cloned().collect()
    }

    fn is_process_alive(&self, pid: u32) -> bool {
        #[cfg(unix)]
        {
            use nix::sys::signal::{kill, Signal};
            use nix::unistd::Pid;
            kill(Pid::from_raw(pid as i32), None).is_ok()
        }
        #[cfg(not(unix))]
        { false }
    }

    fn get_process_memory(&self, pid: u32) -> Option<u64> {
        #[cfg(unix)]
        {
            let path = format!("/proc/{}/statm", pid);
            if let Ok(content) = std::fs::read_to_string(&path) {
                if let Some(rss_str) = content.split_whitespace().nth(1) {
                    if let Ok(rss) = rss_str.parse::<u64>() { return Some(rss * 4096); }
                }
            }
            None
        }
        #[cfg(not(unix))]
        { None }
    }
}
impl Default for ProcessManager {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_process_manager() {
        let pm = ProcessManager::new();
        assert_eq!(pm.process_names().len(), 0);
    }
    
    #[test]
    fn test_process_entry() {
        let cfg = ProcessConfig {
            name: "test".to_string(),
            command: "/bin/sleep".to_string(),
            args: vec![],
            working_directory: None,
            environment: std::collections::HashMap::new(),
            auto_restart: false,
            log_file: None,
            max_instances: None,
        };
        let e = ProcessEntry::new("test".to_string(), 1234, cfg);
        assert_eq!(e.name, "test");
        assert_eq!(e.pid, 1234);
    }
}
