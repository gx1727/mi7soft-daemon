//! Output formatting utilities

use crate::process::{ProcessState, ProcessStatus};

/// Print status table (like pm2)
pub fn print_status_table(statuses: &[ProcessStatus]) {
    if statuses.is_empty() {
        println!("No processes running");
        return;
    }

    let name_col = "Name";
    let pid_col = "PID";
    let status_col = "Status";
    let uptime_col = "Uptime";
    let cpu_col = "CPU";
    let memory_col = "Memory";

    let min_name_w = 15;
    let min_pid_w = 6;
    let min_status_w = 8;
    let min_uptime_w = 8;
    let min_cpu_w = 5;
    let min_memory_w = 10;

    let name_w = std::cmp::max(
        min_name_w,
        statuses.iter().map(|s| s.name.len()).max().unwrap_or(0),
    );
    let pid_w = std::cmp::max(
        min_pid_w,
        statuses.iter().map(|s| s.pid.to_string().len()).max().unwrap_or(0),
    );
    let status_w = std::cmp::max(
        min_status_w,
        statuses
            .iter()
            .map(|s| format_state(&s.state).len())
            .max()
            .unwrap_or(0),
    );
    let uptime_w = std::cmp::max(
        min_uptime_w,
        statuses
            .iter()
            .map(|s| format_uptime(s.uptime).len())
            .max()
            .unwrap_or(0),
    );
    let cpu_w = min_cpu_w;
    let memory_w = std::cmp::max(
        min_memory_w,
        statuses
            .iter()
            .map(|s| format_memory(s.memory).len())
            .max()
            .unwrap_or(0),
    );

    let border = format!(
        "┌{}┬{}┬{}┬{}┬{}┬{}┐",
        "─".repeat(name_w),
        "─".repeat(pid_w),
        "─".repeat(status_w),
        "─".repeat(uptime_w),
        "─".repeat(cpu_w),
        "─".repeat(memory_w)
    );

    println!("{}", border);
    println!(
        "│ {:^name_w$} │ {:^pid_w$} │ {:^status_w$} │ {:^uptime_w$} │ {:^cpu_w$} │ {:^memory_w$} │",
        name_col,
        pid_col,
        status_col,
        uptime_col,
        cpu_col,
        memory_col,
        name_w = name_w,
        pid_w = pid_w,
        status_w = status_w,
        uptime_w = uptime_w,
        cpu_w = cpu_w,
        memory_w = memory_w
    );

    let sep = format!(
        "├{}┼{}┼{}┼{}┼{}┼{}┤",
        "─".repeat(name_w),
        "─".repeat(pid_w),
        "─".repeat(status_w),
        "─".repeat(uptime_w),
        "─".repeat(cpu_w),
        "─".repeat(memory_w)
    );
    println!("{}", sep);

    for s in statuses {
        let status_str = format_state(&s.state);
        println!(
            "│ {:name_w$} │ {:^pid_w$} │ {:^status_w$} │ {:^uptime_w$} │ {:^cpu_w$} │ {:^memory_w$} │",
            s.name,
            s.pid,
            status_str,
            format_uptime(s.uptime),
            "N/A",
            format_memory(s.memory),
            name_w = name_w,
            pid_w = pid_w,
            status_w = status_w,
            uptime_w = uptime_w,
            cpu_w = cpu_w,
            memory_w = memory_w
        );
    }

    let footer = format!(
        "└{}┴{}┴{}┴{}┴{}┴{}┘",
        "─".repeat(name_w),
        "─".repeat(pid_w),
        "─".repeat(status_w),
        "─".repeat(uptime_w),
        "─".repeat(cpu_w),
        "─".repeat(memory_w)
    );
    println!("{}", footer);
}

/// Format process state to string
pub fn format_state(state: &ProcessState) -> String {
    match state {
        ProcessState::Running => "online".to_string(),
        ProcessState::Stopped => "stopped".to_string(),
        ProcessState::Dead => "dead".to_string(),
        ProcessState::Unknown => "unknown".to_string(),
    }
}

/// Format uptime in human-readable format
pub fn format_uptime(seconds: u64) -> String {
    if seconds < 60 {
        format!("{}s", seconds)
    } else if seconds < 3600 {
        format!("{}m", seconds / 60)
    } else if seconds < 86400 {
        format!("{}h", seconds / 3600)
    } else {
        format!("{}d", seconds / 86400)
    }
}

/// Format memory in human-readable format
pub fn format_memory(bytes: Option<u64>) -> String {
    match bytes {
        Some(b) if b >= 1024 * 1024 * 1024 => {
            format!("{:.1} GB", b as f64 / (1024.0 * 1024.0 * 1024.0))
        }
        Some(b) if b >= 1024 * 1024 => format!("{:.1} MB", b as f64 / (1024.0 * 1024.0)),
        Some(b) if b >= 1024 => format!("{:.1} KB", b as f64 / 1024.0),
        Some(b) => format!("{} B", b),
        None => "N/A".to_string(),
    }
}
