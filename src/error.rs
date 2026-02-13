use std::fmt;
use thiserror::Error;

/// Main error type for the daemon process manager
#[derive(Debug, Error)]
pub enum DaemonError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Process '{name}' is already running (PID: {pid})")]
    AlreadyRunning { name: String, pid: u32 },

    #[error("Process '{name}' is not running")]
    NotRunning { name: String },

    #[error("Failed to start process '{name}': {reason}")]
    StartFailed { name: String, reason: String },

    #[error("Failed to stop process '{name}': {reason}")]
    StopFailed { name: String, reason: String },

    #[error("Failed to restart process '{name}': {reason}")]
    RestartFailed { name: String, reason: String },

    #[error("Lock file error: {0}")]
    LockFile(String),

    #[error("PID file error: {0}")]
    PidFile(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML parse error: {0}")]
    TomlParse(String),

    #[error("Daemonization error: {0}")]
    Daemonize(String),

    #[error("Signal error: {0}")]
    Signal(String),

    #[error("Process not found")]
    ProcessNotFound,
}

impl DaemonError {
    /// Get appropriate exit code following sysexits(3)
    pub fn exit_code(&self) -> i32 {
        match self {
            DaemonError::Config(_) => 78,            // EX_CONFIG
            DaemonError::AlreadyRunning { .. } => 1, // Generic error
            DaemonError::NotRunning { .. } => 1,
            DaemonError::StartFailed { .. } => 71, // EX_SOFTWARE
            DaemonError::StopFailed { .. } => 71,
            DaemonError::RestartFailed { .. } => 71,
            DaemonError::LockFile(_) => 75, // EX_TEMPFAIL
            DaemonError::PidFile(_) => 75,
            DaemonError::Io(_) => 74,        // EX_IOERR
            DaemonError::TomlParse(_) => 65, // EX_DATAERR
            DaemonError::Daemonize(_) => 75,
            DaemonError::Signal(_) => 70, // EX_SOFTWARE
            DaemonError::ProcessNotFound => 1,
        }
    }
}

#[cfg(unix)]
impl From<daemonize::Error> for DaemonError {
    fn from(err: daemonize::Error) -> Self {
        DaemonError::Daemonize(format!("{}", err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_exit_codes() {
        assert_eq!(DaemonError::Config("test".to_string()).exit_code(), 78);
        assert_eq!(
            DaemonError::AlreadyRunning {
                name: "test".to_string(),
                pid: 1234
            }
            .exit_code(),
            1
        );
        assert_eq!(
            DaemonError::NotRunning {
                name: "test".to_string()
            }
            .exit_code(),
            1
        );
        assert_eq!(
            DaemonError::StartFailed {
                name: "test".to_string(),
                reason: "test".to_string()
            }
            .exit_code(),
            71
        );
        assert_eq!(
            DaemonError::StopFailed {
                name: "test".to_string(),
                reason: "test".to_string()
            }
            .exit_code(),
            71
        );
        assert_eq!(
            DaemonError::RestartFailed {
                name: "test".to_string(),
                reason: "test".to_string()
            }
            .exit_code(),
            71
        );
        assert_eq!(DaemonError::LockFile("test".to_string()).exit_code(), 75);
        assert_eq!(DaemonError::PidFile("test".to_string()).exit_code(), 75);
        assert_eq!(DaemonError::TomlParse("test".to_string()).exit_code(), 65);
        assert_eq!(DaemonError::Daemonize("test".to_string()).exit_code(), 75);
        assert_eq!(DaemonError::Signal("test".to_string()).exit_code(), 70);
        assert_eq!(DaemonError::ProcessNotFound.exit_code(), 1);
    }
}
