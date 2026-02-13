mod cli;
mod config;
mod daemon;
mod error;
mod pidfile;
mod process;
mod signal;

use clap::Parser;
use cli::{Cli, Commands};
use daemon::Daemon;
use error::DaemonError;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    if let Err(e) = run(&cli).await {
        eprintln!("Error: {}", e);
        std::process::exit(e.exit_code());
    }
    
    Ok(())
}

async fn run(cli: &Cli) -> Result<(), DaemonError> {
    let config_path = cli.get_config_path();
    
    #[cfg(unix)]
    let pid_file_path = "/var/run/mi7soft-daemon.pid";
    
    #[cfg(not(unix))]
    let pid_file_path = "mi7soft-daemon.pid";
    
    match &cli.command {
        Commands::Start => {
            run_daemon(config_path, pid_file_path, cli.daemonize).await
        }
        Commands::StartProcess { name } => {
            start_single_process(config_path, name).await
        }
        Commands::Stop { name } => {
            stop_single_process(config_path, pid_file_path, name).await
        }
        Commands::Restart { name } => {
            restart_single_process(config_path, name).await
        }
        Commands::Status { name } => {
            show_status(config_path, pid_file_path, name).await
        }
        Commands::Shutdown => {
            shutdown_daemon(pid_file_path).await
        }
    }
}

#[cfg(unix)]
async fn run_daemon(config_path: std::path::PathBuf, pid_file_path: &str, daemonize: bool) -> Result<(), DaemonError> {
    if daemonize {
        use daemonize::Daemonize;
        let daemon = Daemonize::new()
            .pid_file(pid_file_path)
            .stdout(std::fs::File::create("/var/log/mi7soft-daemon.out").unwrap())
            .stderr(std::fs::File::create("/var/log/mi7soft-daemon.err").unwrap());
        
        daemon.start()?;
    }
    
    let mut daemon = Daemon::new(config_path, pid_file_path)?;
    daemon.run().await
}

#[cfg(not(unix))]
async fn run_daemon(_config_path: std::path::PathBuf, _pid_file_path: &str, _daemonize: bool) -> Result<(), DaemonError> {
    Err(DaemonError::Daemonize("Daemon mode not supported on Windows".to_string()))
}

async fn start_single_process(config_path: std::path::PathBuf, name: &str) -> Result<(), DaemonError> {
    #[cfg(unix)]
    let pid_file_path = "mi7soft-daemon-single.pid";
    
    #[cfg(not(unix))]
    let pid_file_path = "mi7soft-daemon-single.pid";
    
    let mut daemon = Daemon::new(config_path, pid_file_path)?;
    let pid = daemon.start_process(name).await?;
    println!("Started process {} with PID {}", name, pid);
    Ok(())
}

async fn stop_single_process(config_path: std::path::PathBuf, pid_file_path: &str, name: &str) -> Result<(), DaemonError> {
    let mut daemon = Daemon::new(config_path, pid_file_path)?;
    let stopped = daemon.stop_process(name).await?;
    println!("Stopped process {}: {} instance(s)", name, stopped.len());
    for pid in stopped {
        println!("  - PID {}", pid);
    }
    Ok(())
}

async fn restart_single_process(config_path: std::path::PathBuf, name: &str) -> Result<(), DaemonError> {
    #[cfg(unix)]
    let pid_file_path = "mi7soft-daemon-single.pid";
    
    #[cfg(not(unix))]
    let pid_file_path = "mi7soft-daemon-single.pid";
    
    let mut daemon = Daemon::new(config_path, pid_file_path)?;
    let pids = daemon.restart_process(name).await?;
    println!("Restarted process {} with {} instance(s)", name, pids.len());
    for pid in pids {
        println!("  - PID {}", pid);
    }
    Ok(())
}

async fn show_status(config_path: std::path::PathBuf, pid_file_path: &str, name: &Option<String>) -> Result<(), DaemonError> {
    let daemon = Daemon::new(config_path, pid_file_path)?;
    
    if let Some(process_name) = name {
        let statuses = daemon.get_status(process_name)?;
        println!("Status for process {}:", process_name);
        for status in statuses {
            println!("  {}", status);
        }
    } else {
        let all_statuses = daemon.get_all_status();
        println!("Status for all processes:");
        for status in all_statuses {
            println!("  {}", status);
        }
    }
    
    Ok(())
}

async fn shutdown_daemon(_pid_file_path: &str) -> Result<(), DaemonError> {
    // In production, this would send a signal to the running daemon
    // For now, just print a message
    println!("To shutdown the daemon, use: kill $(cat /var/run/mi7soft-daemon.pid)");
    Ok(())
}
