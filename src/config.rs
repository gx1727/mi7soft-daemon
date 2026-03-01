use serde::{Deserialize, Serialize};
use tracing::warn;
use std::collections::HashMap;
use std::path::Path;
use crate::error::DaemonError;

/// Daemon configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DaemonConfig {
    pub daemon: Option<DaemonSettings>,
    pub processes: Vec<ProcessConfig>,
}

/// Daemon-specific settings
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DaemonSettings {
    #[serde(default = "default_pid_file")]
    pub pid_file: String,
    
    #[serde(default = "default_log_file")]
    pub log_file: String,
    
    #[serde(default = "default_check_interval")]
    pub check_interval: u64,
}

fn default_pid_file() -> String {
    "/var/run/mi7soft-daemon.pid".to_string()
}

fn default_log_file() -> String {
    "/var/log/mi7soft-daemon.log".to_string()
}

fn default_check_interval() -> u64 {
    5
}

/// Process configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProcessConfig {
    pub name: String,
    
    pub command: String,
    
    #[serde(default)]
    pub args: Vec<String>,
    
    #[serde(default)]
    pub working_directory: Option<String>,
    
    #[serde(default)]
    pub environment: HashMap<String, String>,
    
    #[serde(default)]
    pub auto_restart: bool,
    
    #[serde(default)]
    pub log_file: Option<String>,
    
    #[serde(default)]
    pub max_instances: Option<usize>,
    
    /// 是否捕获进程输出（新增）
    #[serde(default = "default_capture_output")]
    pub capture_output: bool,
    
    /// 最大日志文件大小（字节）（新增）
    #[serde(default)]
    pub max_log_size: Option<u64>,
}

fn default_capture_output() -> bool {
    true  // 默认捕获输出
}

/// Load configuration from TOML file
pub fn load_config(path: &Path) -> Result<DaemonConfig, DaemonError> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| DaemonError::Config(format!("Failed to read config file: {}", e)))?;
    
    let config: DaemonConfig = toml::from_str(&content)
        .map_err(|e| DaemonError::TomlParse(format!("Failed to parse TOML: {}", e)))?;
    
    // Validate configuration
    validate_config(&config)?;
    
    Ok(config)
}

/// Validate configuration
fn validate_config(config: &DaemonConfig) -> Result<(), DaemonError> {
    if config.processes.is_empty() {
        return Err(DaemonError::Config("No processes defined".to_string()));
    }
    
    // Check for duplicate process names
    let mut names = std::collections::HashSet::new();
    for proc in &config.processes {
        if !names.insert(&proc.name) {
            return Err(DaemonError::Config(format!(
                "Duplicate process name: {}", proc.name
            )));
        }
        
        // 验证日志文件路径
        if proc.capture_output && proc.log_file.is_none() {
            // 如果捕获输出但没有指定日志文件，使用默认路径
            warn!(
                process = proc.name.as_str(),
                "No log file specified, using default path"
            );
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn test_load_config() {
        let config_content = r#"
[daemon]
pid_file = "/var/run/test.pid"
log_file = "/var/log/test.log"
check_interval = 10

[[processes]]
name = "test-process"
command = "/bin/sleep"
args = ["100"]
working_directory = "/tmp"
auto_restart = true
capture_output = true

[[processes]]
name = "another-process"
command = "/bin/echo"
args = ["hello"]
"#;
        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "{}", config_content).unwrap();
        
        let config = load_config(temp_file.path()).unwrap();
        assert_eq!(config.processes.len(), 2);
        assert_eq!(config.processes[0].name, "test-process");
        assert_eq!(config.processes[1].name, "another-process");
        assert!(config.daemon.is_some());
        assert_eq!(config.daemon.unwrap().check_interval, 10);
        assert!(config.processes[0].capture_output);
    }

    #[test]
    fn test_invalid_toml() {
        let invalid_content = r#"
[daemon
pid_file = "/var/run/test.pid"
"#;
        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "{}", invalid_content).unwrap();
        
        let result = load_config(temp_file.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_duplicate_process_names() {
        let duplicate_content = r#"
[[processes]]
name = "test"
command = "/bin/sleep"

[[processes]]
name = "test"
command = "/bin/echo"
"#;
        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "{}", duplicate_content).unwrap();
        
        let result = load_config(temp_file.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_processes() {
        let empty_content = r#"
[daemon]
pid_file = "/var/run/test.pid"
"#;
        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "{}", empty_content).unwrap();
        
        let result = load_config(temp_file.path());
        assert!(result.is_err());
    }
}
