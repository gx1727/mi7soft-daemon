mod cli;
mod config;
mod daemon;
mod error;
mod formatter;
mod logging;
mod pidfile;
mod process;
mod process_output;
mod signal;
mod storage;

use clap::Parser;
use cli::{Cli, Commands};
use daemon::Daemon;
use error::DaemonError;
use formatter::print_status_table;
use std::path::PathBuf;
use tracing::{error, info};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Initialize logging early
    let log_file = std::env::var("MI7SOFT_LOG_FILE").ok();
    if let Err(e) = logging::init_logging(log_file.as_deref(), cli.verbose) {
        eprintln!("Failed to initialize logging: {}", e);
    }

    info!("MI7Soft Daemon starting...");

    if let Err(e) = run(&cli).await {
        error!("Error: {}", e);
        eprintln!("Error: {}", e);
        std::process::exit(e.exit_code());
    }

    Ok(())
}

async fn run(cli: &Cli) -> Result<(), DaemonError> {
    let config_path = cli.get_config_path();

    // Check if we should daemonize (skip if MI7SOFT_NO_DAEMON is set)
    let should_daemonize = match &cli.command {
        Commands::Start => cli.daemonize && std::env::var("MI7SOFT_NO_DAEMON").is_err(),
        _ => false,
    };

    #[cfg(unix)]
    let pid_file_path = "/var/run/mi7soft-daemon.pid";

    #[cfg(not(unix))]
    let pid_file_path = "mi7soft-daemon.pid";

    match &cli.command {
        Commands::Start => {
            run_daemon(config_path, pid_file_path, should_daemonize).await
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
        Commands::Logs { name, lines, follow, since } => {
            show_logs(config_path, name, *lines, *follow, *since).await
        }
        Commands::History { name, number } => {
            show_history(config_path, name, *number).await
        }
    }
}

#[cfg(unix)]
async fn run_daemon(config_path: PathBuf, pid_file_path: &str, daemonize: bool) -> Result<(), DaemonError> {
    if daemonize {
        use daemonize::Daemonize;
        let daemon = Daemonize::new()
            .stdout(std::fs::File::create("/var/log/mi7soft-daemon.out").unwrap())
            .stderr(std::fs::File::create("/var/log/mi7soft-daemon.err").unwrap());

        daemon.start()?;
    }

    info!(pid_file = pid_file_path, "Starting daemon");
    let mut daemon = Daemon::new(config_path, pid_file_path)?;
    daemon.run().await
}

#[cfg(not(unix))]
async fn run_daemon(_config_path: PathBuf, _pid_file_path: &str, _daemonize: bool) -> Result<(), DaemonError> {
    Err(DaemonError::Daemonize(
        "Daemon mode not supported on Windows".to_string(),
    ))
}

async fn start_single_process(config_path: PathBuf, name: &str) -> Result<(), DaemonError> {
    #[cfg(unix)]
    let pid_file_path = "mi7soft-daemon-single.pid";

    #[cfg(not(unix))]
    let pid_file_path = "mi7soft-daemon-single.pid";

    let mut daemon = Daemon::new(config_path, pid_file_path)?;
    let pid = daemon.start_process(name).await?;
    info!(process = name, pid = pid, "Process started");
    println!("Started process {} with PID {}", name, pid);
    Ok(())
}

async fn stop_single_process(
    config_path: PathBuf,
    pid_file_path: &str,
    name: &str,
) -> Result<(), DaemonError> {
    let mut daemon = Daemon::new(config_path, pid_file_path)?;
    let stopped = daemon.stop_process(name).await?;
    info!(process = name, instances = stopped.len(), "Process stopped");
    println!("Stopped process {}: {} instance(s)", name, stopped.len());
    for pid in stopped {
        println!("  - PID {}", pid);
    }
    Ok(())
}

async fn restart_single_process(config_path: PathBuf, name: &str) -> Result<(), DaemonError> {
    #[cfg(unix)]
    let pid_file_path = "mi7soft-daemon-single.pid";

    #[cfg(not(unix))]
    let pid_file_path = "mi7soft-daemon-single.pid";

    let mut daemon = Daemon::new(config_path, pid_file_path)?;
    let pids = daemon.restart_process(name).await?;
    info!(process = name, instances = pids.len(), "Process restarted");
    println!("Restarted process {} with {} instance(s)", name, pids.len());
    for pid in pids {
        println!("  - PID {}", pid);
    }
    Ok(())
}

async fn show_status(
    config_path: PathBuf,
    pid_file_path: &str,
    name: &Option<String>,
) -> Result<(), DaemonError> {
    let daemon = Daemon::new_read_only(config_path, pid_file_path)?;

    if let Some(process_name) = name {
        let statuses = daemon.get_status(process_name)?;
        info!(process = process_name, "Showing status");
        print_status_table(&statuses);
    } else {
        let all_statuses = daemon.get_all_status();
        info!("Showing all process status");
        print_status_table(&all_statuses);
    }

    Ok(())
}

async fn shutdown_daemon(pid_file_path: &str) -> Result<(), DaemonError> {
    use std::fs;
    use std::process::Command;

    info!("Shutdown requested");

    let pid = fs::read_to_string(pid_file_path)
        .map_err(|e| DaemonError::PidFile(format!("Failed to read PID file: {}", e)))?
        .trim()
        .parse::<u32>()
        .map_err(|_| DaemonError::PidFile("Invalid PID in file".to_string()))?;

    println!("Stopping daemon (PID: {})...", pid);

    // Kill the process group (negative PID)
    let _ = Command::new("kill")
        .arg("--")
        .arg(format!("-{}", pid))
        .output();

    // Also kill any child processes by name
    let _ = Command::new("pkill")
        .arg("-f")
        .arg("php cli.php")
        .output();

    // Wait and force kill if still running
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    let _ = Command::new("kill")
        .arg("-9")
        .arg(format!("{}", pid))
        .output();

    let _ = Command::new("pkill")
        .arg("-9")
        .arg("-f")
        .arg("php cli.php")
        .output();

    // Clean up PID file
    let _ = fs::remove_file(pid_file_path);

    // Clean up state file
    let state_file = std::path::PathBuf::from(pid_file_path).with_extension("state");
    let _ = fs::remove_file(state_file);

    println!("Daemon stopped");
    Ok(())
}

async fn show_logs(
    config_path: PathBuf,
    name: &str,
    lines: usize,
    follow: bool,
    since: Option<i64>,
) -> Result<(), DaemonError> {
    use crate::process_output::LogViewer;

    let config = crate::config::load_config(&config_path)?;
    let process_config = config
        .processes
        .iter()
        .find(|p| p.name == name)
        .ok_or_else(|| DaemonError::Config(format!("Process '{}' not found", name)))?;

    let log_file = process_config.log_file.as_ref()
        .ok_or_else(|| DaemonError::Config("No log file configured".to_string()))?;

    let viewer = LogViewer::new(std::path::PathBuf::from(log_file));

    if follow {
        println!("Following logs for {} (Ctrl+C to stop)...", name);
        let mut receiver = viewer.follow().await?;
        while let Some(line) = receiver.recv().await {
            println!("{}", line);
        }
    } else if let Some(secs) = since {
        let logs = viewer.since(secs).await?;
        for line in logs {
            println!("{}", line);
        }
    } else {
        let logs = viewer.tail(lines).await?;
        for line in logs {
            println!("{}", line);
        }
    }

    Ok(())
}

async fn show_history(_config_path: PathBuf, name: &str, number: usize) -> Result<(), DaemonError> {
    use crate::storage::Storage;

    let db_path = dirs::data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("mi7soft-daemon")
        .join("daemon.db");

    let storage = Storage::new(db_path)?;
    let history = storage.get_history(name, number)?;

    if history.is_empty() {
        println!("No history found for process: {}", name);
        return Ok(());
    }

    println!(
        "History for process {} (last {} records):",
        name, number
    );
    println!("{:-<80}", "");

    for record in history {
        let status = match record.exit_code {
            Some(0) => "✓ Success",
            Some(code) => &format!("✗ Failed (code: {})", code),
            None => "Running...",
        };

        let duration = match record.end_time {
            Some(end) => {
                let secs = (end - record.start_time).num_seconds();
                format!("{}s", secs)
            }
            None => "N/A".to_string(),
        };

        println!(
            "  PID {:<6} | {:<19} - {:<19} | {:<12} | {}",
            record.pid,
            record.start_time.format("%Y-%m-%d %H:%M:%S"),
            record.end_time
                .map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string())
                .unwrap_or_else(|| "Still running".to_string()),
            duration,
            status
        );
    }

    Ok(())
}
