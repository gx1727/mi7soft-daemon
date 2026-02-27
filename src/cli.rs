use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Process manager daemon for Linux
#[derive(Debug, Parser)]
#[command(name = "mi7soft-daemon")]
#[command(author = "Your Name")]
#[command(version = "0.1.0")]
#[command(about = "A daemon process manager that keeps your services running", long_about = None)]
pub struct Cli {
    /// Configuration file path
    #[arg(short, long, value_name = "FILE", global = true)]
    pub config: Option<PathBuf>,
    
    /// Increase verbosity (can be used multiple times)
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    pub verbose: u8,
    
    /// Run as daemon (detach from terminal)
    #[arg(short, long, global = true)]
    pub daemonize: bool,
    
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Start the daemon (manages all configured processes)
    Start,
    
    /// Start a specific process
    StartProcess {
        /// Process name
        #[arg(value_name = "NAME")]
        name: String,
    },
    
    /// Stop a specific process (all instances)
    Stop {
        /// Process name
        #[arg(value_name = "NAME")]
        name: String,
    },
    
    /// Restart a specific process (all instances)
    Restart {
        /// Process name
        #[arg(value_name = "NAME")]
        name: String,
    },
    
    /// Show process status
    Status {
        /// Process name (optional, shows all if not specified)
        #[arg(value_name = "NAME")]
        name: Option<String>,
    },
    
    /// Shutdown the daemon (stops all processes)
    Shutdown,
}

impl Cli {
    pub fn get_config_path(&self) -> PathBuf {
        // First: check if -c option is provided
        if let Some(ref config) = self.config {
            return config.clone();
        }
        
        // Second: check current directory for daemon.toml
        let current_dir = std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."));
        let local_config = current_dir.join("daemon.toml");
        
        if local_config.exists() {
            return local_config;
        }
        
        // Third: create default config in current directory if not exists
        let default_config = r#"# mi7soft-daemon configuration
# Edit this file to manage your processes

[daemon]
check_interval = 5

# Example process - uncomment and modify as needed
[[processes]]
name = "example"
command = "echo"
args = ["hello"]
"#;
        if std::fs::write(&local_config, default_config).is_ok() {
            eprintln!("Created default config: {}", local_config.display());
        }
        
        local_config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing() {
        use std::ffi::OsString;
        
        let cli = Cli::try_parse_from([
            "mi7soft-daemon",
            "--verbose",
            "status",
        ]).unwrap();
        
        assert_eq!(cli.verbose, 1);
        assert!(matches!(cli.command, Commands::Status { .. }));
        
        if let Commands::Status { name } = cli.command {
            assert!(name.is_none());
        } else {
            panic!("Expected Status command");
        }
    }

    #[test]
    fn test_cli_with_process_name() {
        let cli = Cli::try_parse_from([
            "mi7soft-daemon",
            "status",
            "test-process",
        ]).unwrap();
        
        if let Commands::Status { name } = cli.command {
            assert_eq!(name, Some("test-process".to_string()));
        } else {
            panic!("Expected Status command with name");
        }
    }

    #[test]
    fn test_cli_start_process() {
        let cli = Cli::try_parse_from([
            "mi7soft-daemon",
            "start-process",
            "my-service",
        ]).unwrap();
        
        if let Commands::StartProcess { name } = cli.command {
            assert_eq!(name, "my-service");
        } else {
            panic!("Expected StartProcess command");
        }
    }

    #[test]
    fn test_cli_stop() {
        let cli = Cli::try_parse_from([
            "mi7soft-daemon",
            "stop",
            "test-process",
        ]).unwrap();
        
        if let Commands::Stop { name } = cli.command {
            assert_eq!(name, "test-process");
        } else {
            panic!("Expected Stop command");
        }
    }

    #[test]
    fn test_cli_restart() {
        let cli = Cli::try_parse_from([
            "mi7soft-daemon",
            "restart",
            "test-process",
        ]).unwrap();
        
        if let Commands::Restart { name } = cli.command {
            assert_eq!(name, "test-process");
        } else {
            panic!("Expected Restart command");
        }
    }

    #[test]
    fn test_cli_start_daemon() {
        let cli = Cli::try_parse_from([
            "mi7soft-daemon",
            "--daemonize",
            "start",
        ]).unwrap();
        
        assert!(cli.daemonize);
        assert!(matches!(cli.command, Commands::Start));
    }

    #[test]
    fn test_cli_shutdown() {
        let cli = Cli::try_parse_from([
            "mi7soft-daemon",
            "shutdown",
        ]).unwrap();
        
        assert!(matches!(cli.command, Commands::Shutdown));
    }
}
