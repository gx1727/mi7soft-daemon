use crate::config::{ProcessConfig, Schedule as ProcessSchedule, ScheduleType};
use crate::error::DaemonError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::str::FromStr;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::process::Child;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
    pub schedule_type: ScheduleType,
    pub interval: Option<u64>,
    pub expression: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SchedulerType {
    Interval,
    Cron,
}

pub struct Scheduler {
    pub scheduler_type: SchedulerType,
    pub interval: Option<u64>,
    pub cron_expression: Option<String>,
    pub next_run: Option<Instant>,
    compiled_schedule: Option<cron::Schedule>,
}

impl Scheduler {
    pub fn from_config(schedule: &ProcessSchedule, global_interval: u64) -> Self {
        let (scheduler_type, interval, cron_expression, compiled_schedule) = match schedule.schedule_type {
            crate::config::ScheduleType::Interval => {
                let interval = schedule.interval.unwrap_or(global_interval);
                (SchedulerType::Interval, Some(interval), None, None)
            }
            crate::config::ScheduleType::Cron => {
                let expr = schedule.expression.as_deref().unwrap_or("* * * * *");
                tracing::debug!(expr = expr, "Cron: parsing expression");
                let compiled = cron::Schedule::from_str(expr);
                match &compiled {
                    Ok(s) => tracing::debug!(schedule = ?s, "Cron: expression parsed OK"),
                    Err(e) => tracing::warn!(expr = expr, error = ?e, "Cron: failed to parse expression"),
                }
                (SchedulerType::Cron, None, schedule.expression.clone(), compiled.ok())
            }
        };

        let next_run = Self::calculate_next_run(scheduler_type.clone(), interval, cron_expression.as_deref(), compiled_schedule.as_ref());

        Self {
            scheduler_type,
            interval,
            cron_expression,
            next_run,
            compiled_schedule,
        }
    }

    fn calculate_next_run(
        scheduler_type: SchedulerType,
        interval: Option<u64>,
        cron_expression: Option<&str>,
        compiled_schedule: Option<&cron::Schedule>,
    ) -> Option<Instant> {
        match scheduler_type {
            SchedulerType::Interval => {
                interval.map(|i| {
                    let next = Instant::now() + Duration::from_secs(i);
                    tracing::debug!(interval = i, next_ms = next.elapsed().as_millis(), "Interval next_run");
                    next
                })
            }
            SchedulerType::Cron => {
                tracing::debug!(expr = cron_expression, "Cron: trying to calculate next run");
                if let Some(schedule) = compiled_schedule {
                    tracing::debug!(schedule = ?schedule, "Cron: schedule compiled OK");
                    if let Some(dt) = schedule.upcoming(chrono::Utc).next() {
                        let now = chrono::Utc::now();
                        let duration = dt.signed_duration_since(now);
                        let next = Instant::now() + Duration::from_secs(duration.num_seconds() as u64);
                        tracing::debug!(next_run = ?dt, now = ?now, diff_sec = duration.num_seconds(), "Cron next_run calculated");
                        return Some(next);
                    } else {
                        tracing::warn!("Cron: schedule.upcoming().next() returned None");
                    }
                } else {
                    tracing::warn!("Cron: compiled_schedule is None");
                }
                tracing::warn!(scheduler_type = ?scheduler_type, "Cron: no next run calculated");
                None
            }
        }
    }

    pub fn should_run(&mut self) -> bool {
        let now = Instant::now();
        if let Some(next) = self.next_run {
            tracing::debug!(scheduler_type = ?self.scheduler_type, now_ms = now.elapsed().as_millis(), next_ms = ?next.elapsed().as_millis(), "should_run check");
            if now >= next {
                self.next_run = Self::calculate_next_run(
                    self.scheduler_type.clone(),
                    self.interval,
                    self.cron_expression.as_deref(),
                    self.compiled_schedule.as_ref(),
                );
                tracing::info!(scheduler_type = ?self.scheduler_type, "should_run: TRUE");
                return true;
            }
        }
        tracing::debug!(scheduler_type = ?self.scheduler_type, "should_run: false");
        false
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProcessState { Running, Stopped, Dead, Unknown }
pub struct ProcessManager {
    registry: HashMap<String, Vec<ProcessEntry>>,
}

impl ProcessManager {
    pub fn new() -> Self {
        Self { registry: HashMap::new() }
    }

    /// Load state from file and verify processes are still alive
    pub fn load_state(&mut self, state_file: &Path) -> Result<(), DaemonError> {
        if !state_file.exists() {
            return Ok(());
        }

        let content = std::fs::read_to_string(state_file)
            .map_err(|e| DaemonError::Config(format!("Failed to read state file: {}", e)))?;

        let data: HashMap<String, Vec<ProcessEntry>> = serde_json::from_str(&content)
            .map_err(|e| DaemonError::Config(format!("Failed to parse state file: {}", e)))?;

        // Verify each process is still alive, keep only live ones
        let mut loaded_count = 0;
        for (name, entries) in data {
            let live_entries: Vec<ProcessEntry> = entries
                .into_iter()
                .filter(|e| self.is_process_alive(e.pid))
                .collect();
            
            if !live_entries.is_empty() {
                self.registry.insert(name, live_entries);
                loaded_count += 1;
            }
        }

        tracing::info!("Loaded {} processes from state file", loaded_count);
        Ok(())
    }

    /// Save state to file
    pub fn save_state(&self, state_file: &Path) -> Result<(), DaemonError> {
        // Convert registry to serializable format
        let data: HashMap<String, Vec<ProcessEntry>> = self.registry.clone();
        
        let content = serde_json::to_string_pretty(&data)
            .map_err(|e| DaemonError::Config(format!("Failed to serialize state: {}", e)))?;

        // Create parent directory if needed
        if let Some(parent) = state_file.parent() {
            std::fs::create_dir_all(parent).ok();
        }

        std::fs::write(state_file, content)
            .map_err(|e| DaemonError::Config(format!("Failed to write state file: {}", e)))?;

        Ok(())
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
        
        // 🔧 设置进程组：让子进程成为新进程组的 leader (PGID = PID)
        // 这样 kill(-(pid as i32)) 就能杀死整个进程组（包括所有 Swoole 子进程）
        cmd.process_group(0);
        
        // 🔧 捕获进程输出
        if config.capture_output {
            cmd.stdout(std::process::Stdio::piped());
            cmd.stderr(std::process::Stdio::piped());
        }
        
        let mut child = cmd.spawn().map_err(|e| DaemonError::StartFailed {
            name: config.name.clone(),
            reason: format!("Failed: {}", e),
        })?;
        
        // 🔧 启动输出捕获任务
        if config.capture_output {
            if let (Some(stdout), Some(stderr)) = (child.stdout.take(), child.stderr.take()) {
                let name = config.name.clone();
                let log_file = config.log_file.clone().unwrap_or_else(|| {
                    format!("/var/log/mi7soft-{}.log", config.name)
                });
                let max_size = config.max_log_size;
                
                tokio::spawn(async move {
                    use crate::process_output::OutputCapture;
                    let (mut capture, mut receiver) = OutputCapture::new(
                        name,
                        std::path::PathBuf::from(&log_file),
                        max_size,
                    );
                    capture.capture_stdout(stdout);
                    capture.capture_stderr(stderr);
                    capture.start_writer(receiver);
                });
            }
        }
        
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
            
            // 🔧 杀死整个进程组：使用负数 PID 表示 PGID
            // 这样会同时杀死 Master + Worker + TaskWorker + Manager 所有进程
            let pgid = Pid::from_raw(-(pid as i32));
            let _ = signal::kill(pgid, Signal::SIGTERM);
            
            for _ in 0..50 {
                tokio::time::sleep(Duration::from_millis(100)).await;
                if !self.is_process_alive(pid) { return Ok(pid); }
            }
            
            // 强制杀死整个进程组
            signal::kill(pgid, Signal::SIGKILL)
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

    pub fn is_process_alive(&self, pid: u32) -> bool {
        #[cfg(unix)]
        {
            // Check if process exists using /proc/{pid}/stat
            let stat_path = format!("/proc/{}/stat", pid);
            if let Ok(content) = std::fs::read_to_string(&stat_path) {
                // Format: pid (name) state ...
                // State is the 3rd field, e.g., "R", "S", "Z", "X", etc.
                if let Some(state_start) = content.find('(') {
                    if let Some(state_end) = content.find(')') {
                        if state_end + 2 < content.len() {
                            let state = content.chars().nth(state_end + 2).unwrap_or('X');
                            tracing::info!("PID {} state: '{}'", pid, state);
                            // 'Z' = zombie, 'X' = dead
                            if state == 'Z' || state == 'X' {
                                return false;
                            }
                        }
                    }
                }
            }
            
            // Fallback: use kill(pid, 0)
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
