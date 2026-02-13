use crate::config::{DaemonConfig, ProcessConfig, load_config};
use crate::error::DaemonError;
use crate::process::{ProcessManager, ProcessState};
use crate::pidfile::PidFile;
use crate::signal::{Signal, SignalHandler};
use std::path::PathBuf;
use tokio::sync::mpsc;

pub struct Daemon {
    config_path: PathBuf,
    config: DaemonConfig,
    process_manager: ProcessManager,
    pid_file: PidFile,
    signal_handler: SignalHandler,
    shutdown_tx: Option<mpsc::UnboundedSender<bool>>,
}

impl Daemon {
    pub fn new(config_path: PathBuf, pid_file_path: &str) -> Result<Self, DaemonError> {
        let config = load_config(&config_path)?;
        let process_manager = ProcessManager::new();
        let mut pid_file = PidFile::new(pid_file_path);
        
        #[cfg(unix)]
        pid_file.acquire_lock()?;
        
        #[cfg(not(unix))]
        let _ = pid_file.acquire_lock();
        
        Ok(Self {
            config_path,
            config,
            process_manager,
            pid_file,
            signal_handler: SignalHandler::new(),
            shutdown_tx: None,
        })
    }
    pub async fn run(&mut self) -> Result<(), DaemonError> {
        self.start_processes().await?;
        
        let (shutdown_tx, mut shutdown_rx) = mpsc::unbounded_channel();
        self.shutdown_tx = Some(shutdown_tx);
        
        let check_interval = self.config.daemon.as_ref()
            .map(|d| d.check_interval)
            .unwrap_or(5);
        
        let mut interval = tokio::time::interval(
            tokio::time::Duration::from_secs(check_interval)
        );
        
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    self.monitor_and_restart().await?;
                }
                Some(signal) = self.signal_handler.recv() => {
                    match signal {
                        Signal::Shutdown => {
                            self.shutdown().await?;
                            return Ok(());
                        }
                        Signal::ReloadConfig => {
                            self.reload_config()?;
                        }
                        _ => {}
                    }
                }
                _ = shutdown_rx.recv() => {
                    self.shutdown().await?;
                    return Ok(());
                }
            }
        }
    }
    
    async fn start_processes(&mut self) -> Result<(), DaemonError> {
        for process_config in &self.config.processes {
            let _ = self.process_manager.spawn(process_config).await;
        }
        Ok(())
    }
    
    async fn monitor_and_restart(&mut self) -> Result<(), DaemonError> {
        let dead_names = self.process_manager.cleanup_dead();
        
        for name in dead_names {
            if let Some(config) = self.find_config(&name) {
                if config.auto_restart {
                    eprintln!("Auto-restarting process: {}", name);
                    let _ = self.process_manager.spawn(&config).await;
                }
            }
        }
        Ok(())
    }
    async fn shutdown(&mut self) -> Result<(), DaemonError> {
        eprintln!("Shutting down daemon...");
        
        let names = self.process_manager.process_names();
        for name in names {
            eprintln!("Stopping process: {}", name);
            let _ = self.process_manager.stop(&name).await;
        }
        
        self.pid_file.release_lock()?;
        
        Ok(())
    }
    
    fn reload_config(&mut self) -> Result<(), DaemonError> {
        eprintln!("Reloading configuration...");
        
        let new_config = load_config(&self.config_path)?;
        
        for new_proc in &new_config.processes {
            if !self.config.processes.iter().any(|p| p.name == new_proc.name) {
                eprintln!("Adding new process: {}", new_proc.name);
            }
        }
        
        for old_proc in &self.config.processes {
            if !new_config.processes.iter().any(|p| p.name == old_proc.name) {
                eprintln!("Removing process: {}", old_proc.name);
            }
        }
        
        self.config = new_config;
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
            self.process_manager.spawn(&config).await
        } else {
            Err(DaemonError::Config(format!("Process '{}' not found in config", name)))
        }
    }
    
    pub async fn stop_process(&mut self, name: &str) -> Result<Vec<u32>, DaemonError> {
        self.process_manager.stop(name).await
    }
    
    pub async fn restart_process(&mut self, name: &str) -> Result<Vec<u32>, DaemonError> {
        if let Some(config) = self.find_config(name) {
            self.process_manager.restart(&config).await
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
