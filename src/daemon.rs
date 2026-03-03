use crate::config::{DaemonConfig, ProcessConfig, load_config};
use crate::error::DaemonError;
use crate::process::{ProcessManager, ProcessState, ProcessStatus, Scheduler};
use crate::pidfile::PidFile;
use crate::signal::{Signal, SignalHandler};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Instant;
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
    schedulers: HashMap<String, Scheduler>,
}

impl Daemon {
    pub fn new(config_path: PathBuf, pid_file_path: &str) -> Result<Self, DaemonError> {
        Self::new_impl(config_path, pid_file_path, true)
    }

    pub fn new_read_only(config_path: PathBuf, pid_file_path: &str) -> Result<Self, DaemonError> {
        Self::new_impl(config_path, pid_file_path, false)
    }

    fn new_impl(config_path: PathBuf, pid_file_path: &str, acquire_lock: bool) -> Result<Self, DaemonError> {
        let config = load_config(&config_path)?;
        let mut process_manager = ProcessManager::new();
        
        let state_file = PathBuf::from(pid_file_path).with_extension("state");
        
        process_manager.load_state(&state_file)?;
        
        let mut pid_file = PidFile::new(pid_file_path);
        
        if acquire_lock {
            #[cfg(unix)]
            pid_file.acquire_lock()?;
            
            #[cfg(not(unix))]
            let _ = pid_file.acquire_lock;
        }

        let global_interval = config.daemon.as_ref()
            .map(|d| d.check_interval)
            .unwrap_or(5);

        let mut schedulers = HashMap::new();
        for proc in &config.processes {
            if let Some(ref schedule) = proc.schedule {
                let scheduler = Scheduler::from_config(schedule, global_interval);
                schedulers.insert(proc.name.clone(), scheduler);
            }
        }
        
        Ok(Self {
            config_path,
            state_file,
            config,
            process_manager,
            pid_file,
            signal_handler: SignalHandler::new(),
            shutdown_tx: None,
            schedulers,
        })
    }
    
    pub async fn run(&mut self) -> Result<(), DaemonError> {
        info!("Starting daemon");
        
        info!("Checking for existing processes...");
        if self.process_manager.process_names().is_empty() {
            info!("No processes found, spawning from config...");
            self.start_processes().await?;
        } else {
            info!("Loaded {} processes from state", self.process_manager.process_names().len());
        }
        
        self.process_manager.save_state(&self.state_file)?;
        
        let (shutdown_tx, mut shutdown_rx) = mpsc::unbounded_channel();
        self.shutdown_tx = Some(shutdown_tx);
        
        let global_interval = self.config.daemon.as_ref()
            .map(|d| d.check_interval)
            .unwrap_or(5);
        
        let has_schedulers = !self.schedulers.is_empty();
        
        if has_schedulers {
            info!("Using per-process scheduling");
        } else {
            info!(global_interval = global_interval, "Daemon started, monitoring processes");
        }
        
        let mut interval = tokio::time::interval(
            tokio::time::Duration::from_secs(1)
        );
        
        let mut last_full_check = Instant::now();
        
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    if has_schedulers {
                        self.monitor_scheduled().await?;
                    } else {
                        if last_full_check.elapsed() >= tokio::time::Duration::from_secs(global_interval) {
                            self.monitor_and_restart().await?;
                            self.process_manager.save_state(&self.state_file)?;
                            last_full_check = Instant::now();
                        }
                    }
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

    async fn monitor_scheduled(&mut self) -> Result<(), DaemonError> {
        let global_interval = self.config.daemon.as_ref()
            .map(|d| d.check_interval)
            .unwrap_or(5);

        static last_full_check: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();
        let last_check = last_full_check.get_or_init(Instant::now);

        // Get scheduler names to check
        let scheduler_names: Vec<String> = self.schedulers.keys().cloned().collect();
        
        for name in scheduler_names {
            if let Some(scheduler) = self.schedulers.get_mut(&name) {
                if scheduler.should_run() {
                    info!(process = name.as_str(), "Running scheduled check");
                    self.monitor_single(&name).await?;
                }
            }
        }

        if last_check.elapsed() >= tokio::time::Duration::from_secs(global_interval) {
            self.monitor_and_restart().await?;
            // Reset the instant
            let _ = last_full_check.set(Instant::now());
        }

        self.process_manager.save_state(&self.state_file)?;
        Ok(())
    }

    async fn monitor_single(&mut self, name: &str) -> Result<(), DaemonError> {
        // Use cleanup_dead and check if our process was affected
        let all_dead = self.process_manager.cleanup_dead();
        
        if all_dead.contains(&name.to_string()) {
            info!(process = name, "Dead processes found");
            if let Some(config) = self.find_config(name) {
                // For cron scheduled processes, always auto_restart
                if self.should_auto_restart(&config) {
                    warn!(process = name, "Auto-restarting dead process");
                    if let Err(e) = self.process_manager.spawn(&config).await {
                        error!(process = name, error = %e, "Failed to restart process");
                    }
                }
            }
        }
        Ok(())
    }

    /// Check if a process should auto restart
    /// For cron scheduled processes, always return true
    fn should_auto_restart(&self, config: &ProcessConfig) -> bool {
        if config.schedule.is_some() {
            // Cron scheduled processes should always auto restart
            return true;
        }
        config.auto_restart
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
        info!("Running monitor check...");
        let dead_names = self.process_manager.cleanup_dead();
        info!("Dead processes found: {:?}", dead_names);
        
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
    
    pub fn get_status(&self, name: &str) -> Result<Vec<ProcessStatus>, DaemonError> {
        self.process_manager.status(name)
    }
    
    pub fn get_all_status(&self) -> Vec<ProcessStatus> {
        self.process_manager.status_all()
    }
}
