use crate::config::{DaemonConfig, ProcessConfig, load_config};
use crate::error::DaemonError;
use crate::process::{ProcessManager, ProcessState};
use crate::pidfile::PidFile;
use crate::signal::{Signal, SignalHandler};
use std::path::PathBuf;
use tokio::sync::mpsc;
use tracing::{error, info, warn};

pub struct Daemon {
    config_path: PathBuf,
    state_file: PathBuf,
    config: DaemonConfig,
    process_manager: ProcessManager,
    pid_file: PidFile,
    signal_handler: SignalHandler,
    shutdown_tx: Option<mpsc::UnboundedSender<bool>>,
}

impl Daemon {
    pub fn new(config_path: PathBuf, pid_file_path: &str) -> Result<Self, DaemonError> {
        let config = load_config(&config_path)?;
        let mut process_manager = ProcessManager::new();
        
        // Derive state file path from pid_file_path
        let state_file = PathBuf::from(pid_file_path).with_extension("state");
        
        // Load existing state and verify processes
        process_manager.load_state(&state_file)?;
        
        let mut pid_file = PidFile::new(pid_file_path);
        
        #[cfg(unix)]
        pid_file.acquire_lock()?;
        
        #[cfg(not(unix))]
        let _ = pid_file.acquire_lock;
        
        Ok(Self {
            config_path,
            state_file,
            config,
            process_manager,
            pid_file,
            signal_handler: SignalHandler::new(),
            shutdown_tx: None,
        })
    }
    
    pub async fn run(&mut self) -> Result<(), DaemonError> {
        info!("Starting daemon");
        
        // If no processes loaded from state, spawn configured ones
        if self.process_manager.process_names().is_empty() {
            self.start_processes().await?;
        }
        
        // Save initial state
        self.process_manager.save_state(&self.state_file)?;
        
        let (shutdown_tx, mut shutdown_rx) = mpsc::unbounded_channel();
        self.shutdown_tx = Some(shutdown_tx);
        
        let check_interval = self.config.daemon.as_ref()
            .map(|d| d.check_interval)
            .unwrap_or(5);
        
        info!(check_interval = check_interval, "Daemon started, monitoring processes");
        
        let mut interval = tokio::time::interval(
            tokio::time::Duration::from_secs(check_interval)
        );
        
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    self.monitor_and_restart().await?;
                    self.process_manager.save_state(&self.state_file)?;
                }
                Some(signal) = self.signal_handler.recv() => {
                    match signal {
                        Signal::Shutdown => {
                            info!("Received shutdown signal");
                            self.shutdown().await?;
                            return Ok(());
                        }
                        Signal::ReloadConfig => {
                            info!("Received reload config signal");
                            self.reload_config()?;
                        }
                        _ => {}
                    }
                }
                _ = shutdown_rx.recv() => {
                    info!("Received shutdown request");
                    self.shutdown().await?;
                    return Ok(());
                }
            }
        }
    }
    
    async fn start_processes(&mut self) -> Result<(), DaemonError> {
        for process_config in &self.config.processes {
            if let Err(e) = self.process_manager.spawn(process_config).await {
                error!(
                    process = process_config.name.as_str(),
                    error = %e,
                    "Failed to start process"
                );
            }
        }
        Ok(())
    }
    
    async fn monitor_and_restart(&mut self) -> Result<(), DaemonError> {
        let dead_names = self.process_manager.cleanup_dead();
        
        for name in dead_names {
            if let Some(config) = self.find_config(&name) {
                if config.auto_restart {
                    warn!(process = name.as_str(), "Auto-restarting dead process");
                    if let Err(e) = self.process_manager.spawn(&config).await {
                        error!(
                            process = name.as_str(),
                            error = %e,
                            "Failed to restart process"
                        );
                    }
                }
            }
        }
        Ok(())
    }
    
    async fn shutdown(&mut self) -> Result<(), DaemonError> {
        info!("Shutting down daemon...");
        
        let names = self.process_manager.process_names();
        for name in names {
            info!(process = name.as_str(), "Stopping process");
            if let Err(e) = self.process_manager.stop(&name).await {
                error!(
                    process = name.as_str(),
                    error = %e,
                    "Failed to stop process"
                );
            }
        }
        
        // Save final state
        self.process_manager.save_state(&self.state_file)?;
        
        self.pid_file.release_lock()?;
        
        info!("Daemon shutdown complete");
        Ok(())
    }
    
    fn reload_config(&mut self) -> Result<(), DaemonError> {
        info!("Reloading configuration...");
        
        let new_config = load_config(&self.config_path)?;
        
        for new_proc in &new_config.processes {
            if !self.config.processes.iter().any(|p| p.name == new_proc.name) {
                info!(process = new_proc.name.as_str(), "Adding new process");
            }
        }
        
        for old_proc in &self.config.processes {
            if !new_config.processes.iter().any(|p| p.name == old_proc.name) {
                info!(process = old_proc.name.as_str(), "Removing process");
            }
        }
        
        self.config = new_config;
        info!("Configuration reloaded");
        Ok(())
    }
    
    fn find_config(&self, name: &str) -> Option<ProcessConfig> {
        self.config.processes.iter().find(|p| &p.name == name).cloned()
    }
    
    pub fn trigger_shutdown(&self) {
        if let Some(ref tx) = self.shutdown_tx {
            let _ = tx.send(true);
        }
    }
    
    pub async fn start_process(&mut self, name: &str) -> Result<u32, DaemonError> {
        if let Some(config) = self.find_config(name) {
            let pid = self.process_manager.spawn(&config).await?;
            self.process_manager.save_state(&self.state_file)?;
            Ok(pid)
        } else {
            Err(DaemonError::Config(format!("Process '{}' not found in config", name)))
        }
    }
    
    pub async fn stop_process(&mut self, name: &str) -> Result<Vec<u32>, DaemonError> {
        let pids = self.process_manager.stop(name).await?;
        self.process_manager.save_state(&self.state_file)?;
        Ok(pids)
    }
    
    pub async fn restart_process(&mut self, name: &str) -> Result<Vec<u32>, DaemonError> {
        if let Some(config) = self.find_config(name) {
            let pids = self.process_manager.restart(&config).await?;
            self.process_manager.save_state(&self.state_file)?;
            Ok(pids)
        } else {
            Err(DaemonError::Config(format!("Process '{}' not found in config", name)))
        }
    }
    
    pub fn get_status(&self, name: &str) -> Result<Vec<String>, DaemonError> {
        let statuses = self.process_manager.status(name)?;
        Ok(statuses.into_iter().map(|s| format!(
            "{} (PID: {}, State: {:?}, Uptime: {}s)",
            s.name, s.pid, s.state, s.uptime
        )).collect())
    }
    
    pub fn get_all_status(&self) -> Vec<String> {
        self.process_manager.status_all().into_iter().map(|s| format!(
            "{} (PID: {}, State: {:?}, Uptime: {}s)",
            s.name, s.pid, s.state, s.uptime
        )).collect()
    }
}
