use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "m7d")]
#[command(bin_name = "m7d")]
#[command(about = "A daemon process manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    
    /// Path to configuration file
    #[arg(short, long, global = true)]
    pub config: Option<PathBuf>,
    
    /// Run as daemon (background)
    #[arg(short = 'd', long, global = true)]
    pub daemonize: bool,
    
    /// Verbose output (can be used multiple times)
    #[arg(short, long, global = true, action = clap::ArgAction::Count)]
    pub verbose: u8,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start the daemon
    Start,
    
    /// Start a specific process
    StartProcess {
        /// Process name
        name: String,
    },
    
    /// Stop a process
    Stop {
        /// Process name
        name: String,
    },
    
    /// Restart a process
    Restart {
        /// Process name
        name: String,
    },
    
    /// Show process status
    Status {
        /// Process name (optional, shows all if not specified)
        name: Option<String>,
    },
    
    /// Shutdown the daemon
    Shutdown,
    
    /// View process logs
    Logs {
        /// Process name
        name: String,
        
        /// Number of lines to show
        #[arg(short = 'n', long, default_value = "100")]
        lines: usize,
        
        /// Follow log output (like tail -f)
        #[arg(short, long)]
        follow: bool,
        
        /// Show logs since N seconds ago
        #[arg(long)]
        since: Option<i64>,
    },
    
    /// View process history
    History {
        /// Process name
        name: String,
        
        /// Number of records to show
        #[arg(short = 'n', long, default_value = "10")]
        number: usize,
    },
}

impl Cli {
    pub fn get_config_path(&self) -> PathBuf {
        self.config.clone().unwrap_or_else(|| {
            dirs::config_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("mi7soft-daemon")
                .join("daemon.toml")
        })
    }
}
