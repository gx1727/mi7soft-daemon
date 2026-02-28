//! 进程输出捕获模块
//! 
//! 负责捕获进程的 stdout/stderr 并存储到文件

use std::path::PathBuf;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::fs::{File, OpenOptions};
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

/// 日志行
#[derive(Debug, Clone)]
pub struct LogLine {
    pub timestamp: i64,
    pub stream: LogStream,
    pub content: String,
}

/// 日志流类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogStream {
    Stdout,
    Stderr,
}

/// 进程输出捕获器
pub struct OutputCapture {
    process_name: String,
    log_file: PathBuf,
    max_size: Option<u64>,
    sender: mpsc::UnboundedSender<LogLine>,
}

impl OutputCapture {
    /// 创建新的输出捕获器
    pub fn new(
        process_name: String,
        log_file: PathBuf,
        max_size: Option<u64>,
    ) -> (Self, mpsc::UnboundedReceiver<LogLine>) {
        let (sender, receiver) = mpsc::unbounded_channel();
        
        (
            Self {
                process_name,
                log_file,
                max_size,
                sender,
            },
            receiver,
        )
    }
    
    /// 捕获 stdout
    pub fn capture_stdout(&self, stdout: tokio::process::ChildStdout) {
        let sender = self.sender.clone();
        let process_name = self.process_name.clone();
        
        tokio::spawn(async move {
            let reader = BufReader::new(stdout).lines();
            let mut lines = reader;
            
            while let Ok(Some(line)) = lines.next_line().await {
                let log_line = LogLine {
                    timestamp: chrono::Utc::now().timestamp(),
                    stream: LogStream::Stdout,
                    content: line,
                };
                
                if sender.send(log_line).is_err() {
                    debug!(process = process_name.as_str(), "stdout channel closed");
                    break;
                }
            }
        });
    }
    
    /// 捕获 stderr
    pub fn capture_stderr(&self, stderr: tokio::process::ChildStderr) {
        let sender = self.sender.clone();
        let process_name = self.process_name.clone();
        
        tokio::spawn(async move {
            let reader = BufReader::new(stderr).lines();
            let mut lines = reader;
            
            while let Ok(Some(line)) = lines.next_line().await {
                let log_line = LogLine {
                    timestamp: chrono::Utc::now().timestamp(),
                    stream: LogStream::Stderr,
                    content: line,
                };
                
                if sender.send(log_line).is_err() {
                    debug!(process = process_name.as_str(), "stderr channel closed");
                    break;
                }
            }
        });
    }
    
    /// 启动日志写入器
    pub fn start_writer(&self, mut receiver: mpsc::UnboundedReceiver<LogLine>) {
        let log_file = self.log_file.clone();
        let max_size = self.max_size;
        let process_name = self.process_name.clone();
        
        tokio::spawn(async move {
            // 打开日志文件
            let mut file = match OpenOptions::new()
                .create(true)
                .append(true)
                .open(&log_file)
                .await
            {
                Ok(f) => f,
                Err(e) => {
                    error!(
                        process = process_name.as_str(),
                        path = ?log_file,
                        error = %e,
                        "Failed to open log file"
                    );
                    return;
                }
            };
            
            info!(
                process = process_name.as_str(),
                path = ?log_file,
                "Log writer started"
            );
            
            while let Some(log_line) = receiver.recv().await {
                // 格式化日志行
                let formatted = format!(
                    "[{}] [{}] {}\n",
                    chrono::DateTime::from_timestamp(log_line.timestamp, 0)
                        .unwrap()
                        .format("%Y-%m-%d %H:%M:%S"),
                    if log_line.stream == LogStream::Stdout { "OUT" } else { "ERR" },
                    log_line.content
                );
                
                // 写入文件
                use tokio::io::AsyncWriteExt;
                if let Err(e) = file.write_all(formatted.as_bytes()).await {
                    error!(
                        process = process_name.as_str(),
                        error = %e,
                        "Failed to write log"
                    );
                }
                
                // 检查文件大小（简化版本，实际应该使用日志轮转）
                if let Some(max) = max_size {
                    if let Ok(metadata) = file.metadata().await {
                        if metadata.len() > max {
                            warn!(
                                process = process_name.as_str(),
                                size = metadata.len(),
                                max = max,
                                "Log file size exceeded"
                            );
                            // TODO: 实现日志轮转
                        }
                    }
                }
            }
            
            info!(process = process_name.as_str(), "Log writer stopped");
        });
    }
}

/// 日志查看器
pub struct LogViewer {
    log_file: PathBuf,
}

impl LogViewer {
    pub fn new(log_file: PathBuf) -> Self {
        Self { log_file }
    }
    
    /// 读取最近的日志
    pub async fn tail(&self, lines: usize) -> Result<Vec<String>, std::io::Error> {
        use tokio::io::AsyncBufReadExt;
        
        let file = File::open(&self.log_file).await?;
        let reader = BufReader::new(file).lines();
        let mut all_lines = Vec::new();
        
        let mut lines_iter = reader;
        while let Ok(Some(line)) = lines_iter.next_line().await {
            all_lines.push(line);
        }
        
        // 返回最后 N 行
        let start = if all_lines.len() > lines {
            all_lines.len() - lines
        } else {
            0
        };
        
        Ok(all_lines[start..].to_vec())
    }
    
    /// 实时跟踪日志（返回接收器）
    pub async fn follow(&self) -> Result<mpsc::Receiver<String>, std::io::Error> {
        let (sender, receiver) = mpsc::channel(100);
        let log_file = self.log_file.clone();
        
        tokio::spawn(async move {
            use tokio::io::AsyncBufReadExt;
            
            // 等待文件存在
            while !log_file.exists() {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
            
            let file = match File::open(&log_file).await {
                Ok(f) => f,
                Err(_) => return,
            };
            
            let reader = BufReader::new(file).lines();
            let mut lines = reader;
            
            while let Ok(Some(line)) = lines.next_line().await {
                if sender.send(line).await.is_err() {
                    break;
                }
            }
        });
        
        Ok(receiver)
    }
    
    /// 按时间过滤日志
    pub async fn since(&self, since_seconds: i64) -> Result<Vec<String>, std::io::Error> {
        let cutoff = chrono::Utc::now().timestamp() - since_seconds;
        let all_lines = self.tail(10000).await?; // 读取最近的 10000 行
        
        let filtered: Vec<String> = all_lines
            .into_iter()
            .filter(|line| {
                // 简单的时间过滤（假设格式正确）
                if let Some(timestamp_str) = line.split('[').nth(1) {
                    if let Some(time_str) = timestamp_str.split(']').next() {
                        // 解析时间戳（简化版本）
                        // 实际应该更严格地解析
                        return true; // 暂时返回所有行
                    }
                }
                false
            })
            .collect();
        
        Ok(filtered)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    
    #[tokio::test]
    async fn test_output_capture() {
        let temp_file = NamedTempFile::new().unwrap();
        let log_path = temp_file.path().to_path_buf();
        
        let (capture, receiver) = OutputCapture::new(
            "test-process".to_string(),
            log_path,
            None,
        );
        
        // 测试捕获器创建
        assert_eq!(capture.process_name, "test-process");
    }
}
