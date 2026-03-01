//! 持久化存储模块
//! 
//! 使用 SQLite 存储进程历史记录和统计信息

use rusqlite::{Connection, Result as SqliteResult};
use rusqlite::OptionalExtension;
use std::path::PathBuf;
use chrono::{DateTime, Utc};
use tracing::{debug, error, info};

/// 进程历史记录
#[derive(Debug, Clone)]
pub struct ProcessHistory {
    pub id: i64,
    pub name: String,
    pub pid: u32,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub exit_code: Option<i32>,
    pub restart_count: u32,
    pub auto_restart: bool,
}

/// 进程统计信息
#[derive(Debug, Clone)]
pub struct ProcessStats {
    pub name: String,
    pub total_starts: u32,
    pub total_restarts: u32,
    pub total_failures: u32,
    pub avg_uptime_seconds: f64,
    pub last_start_time: Option<DateTime<Utc>>,
    pub last_exit_code: Option<i32>,
}

/// 存储管理器
pub struct Storage {
    conn: Connection,
}

impl Storage {
    /// 创建新的存储管理器
    pub fn new(db_path: PathBuf) -> SqliteResult<Self> {
        // 确保目录存在
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        
        let conn = Connection::open(&db_path)?;
        
        let storage = Self { conn };
        storage.initialize_tables()?;
        
        info!(path = ?db_path, "Storage initialized");
        Ok(storage)
    }
    
    /// 初始化数据库表
    fn initialize_tables(&self) -> SqliteResult<()> {
        self.conn.execute_batch(
            r#"
            -- 进程历史表
            CREATE TABLE IF NOT EXISTS process_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                pid INTEGER NOT NULL,
                start_time TEXT NOT NULL,
                end_time TEXT,
                exit_code INTEGER,
                restart_count INTEGER DEFAULT 0,
                auto_restart BOOLEAN DEFAULT 0
            );
            
            -- 创建索引
            CREATE INDEX IF NOT EXISTS idx_process_name ON process_history(name);
            CREATE INDEX IF NOT EXISTS idx_start_time ON process_history(start_time);
            
            -- 进程统计表
            CREATE TABLE IF NOT EXISTS process_stats (
                name TEXT PRIMARY KEY,
                total_starts INTEGER DEFAULT 0,
                total_restarts INTEGER DEFAULT 0,
                total_failures INTEGER DEFAULT 0,
                total_uptime_seconds INTEGER DEFAULT 0,
                last_start_time TEXT,
                last_exit_code INTEGER
            );
            "#,
        )?;
        
        debug!("Database tables initialized");
        Ok(())
    }
    
    /// 记录进程启动
    pub fn record_start(
        &self,
        name: &str,
        pid: u32,
        auto_restart: bool,
    ) -> SqliteResult<i64> {
        let now = Utc::now().to_rfc3339();
        
        self.conn.execute(
            "INSERT INTO process_history (name, pid, start_time, auto_restart)
             VALUES (?1, ?2, ?3, ?4)",
            (name, pid as i64, now.clone(), auto_restart),
        )?;
        
        let id = self.conn.last_insert_rowid();
        
        // 更新统计
        self.conn.execute(
            "INSERT INTO process_stats (name, total_starts, last_start_time)
             VALUES (?1, 1, ?2)
             ON CONFLICT(name) DO UPDATE SET
                 total_starts = total_starts + 1,
                 last_start_time = ?2",
            (name, now),
        )?;
        
        debug!(process = name, pid = pid, record_id = id, "Process start recorded");
        Ok(id)
    }
    
    /// 记录进程结束
    pub fn record_end(
        &self,
        name: &str,
        pid: u32,
        exit_code: Option<i32>,
    ) -> SqliteResult<()> {
        let now = Utc::now().to_rfc3339();
        
        // 更新历史记录
        let rows_affected = self.conn.execute(
            "UPDATE process_history
             SET end_time = ?1, exit_code = ?2
             WHERE name = ?3 AND pid = ?4 AND end_time IS NULL",
            (now, exit_code, name, pid as i64),
        )?;
        
        // 更新统计
        if exit_code.unwrap_or(0) != 0 {
            self.conn.execute(
                "UPDATE process_stats
                 SET total_failures = total_failures + 1, last_exit_code = ?1
                 WHERE name = ?2",
                (exit_code, name),
            )?;
        }
        
        debug!(
            process = name,
            pid = pid,
            exit_code = exit_code,
            rows = rows_affected,
            "Process end recorded"
        );
        
        Ok(())
    }
    
    /// 记录进程重启
    pub fn record_restart(&self, name: &str) -> SqliteResult<()> {
        self.conn.execute(
            "UPDATE process_stats
             SET total_restarts = total_restarts + 1
             WHERE name = ?1",
            (name,),
        )?;
        
        debug!(process = name, "Process restart recorded");
        Ok(())
    }
    
    /// 更新运行时间
    pub fn update_uptime(&self, name: &str, uptime_seconds: i64) -> SqliteResult<()> {
        self.conn.execute(
            "UPDATE process_stats
             SET total_uptime_seconds = total_uptime_seconds + ?1
             WHERE name = ?2",
            (uptime_seconds, name),
        )?;
        
        Ok(())
    }
    
    /// 获取进程历史
    pub fn get_history(&self, name: &str, limit: usize) -> SqliteResult<Vec<ProcessHistory>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, pid, start_time, end_time, exit_code, restart_count, auto_restart
             FROM process_history
             WHERE name = ?1
             ORDER BY start_time DESC
             LIMIT ?2"
        )?;
        
        let history = stmt.query_map((name, limit as i64), |row| {
            Ok(ProcessHistory {
                id: row.get(0)?,
                name: row.get(1)?,
                pid: row.get::<_, i64>(2)? as u32,
                start_time: DateTime::parse_from_rfc3339(&row.get::<_, String>(3)?)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                end_time: row.get::<_, Option<String>>(4)?
                    .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                    .map(|dt| dt.with_timezone(&Utc)),
                exit_code: row.get(5)?,
                restart_count: row.get::<_, i64>(6)? as u32,
                auto_restart: row.get(7)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;
        
        Ok(history)
    }
    
    /// 获取进程统计
    pub fn get_stats(&self, name: &str) -> SqliteResult<Option<ProcessStats>> {
        let mut stmt = self.conn.prepare(
            "SELECT name, total_starts, total_restarts, total_failures,
                    total_uptime_seconds, last_start_time, last_exit_code
             FROM process_stats
             WHERE name = ?1"
        )?;
        
        let result = stmt.query_row((name,), |row| {
            let total_starts: i64 = row.get(1)?;
            let total_uptime: i64 = row.get(4)?;
            
            Ok(ProcessStats {
                name: row.get(0)?,
                total_starts: total_starts as u32,
                total_restarts: row.get::<_, i64>(2)? as u32,
                total_failures: row.get::<_, i64>(3)? as u32,
                avg_uptime_seconds: if total_starts > 0 {
                    total_uptime as f64 / total_starts as f64
                } else {
                    0.0
                },
                last_start_time: row.get::<_, Option<String>>(5)?
                    .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                    .map(|dt| dt.with_timezone(&Utc)),
                last_exit_code: row.get(6)?,
            })
        }).optional()?;
        
        Ok(result)
    }
    
    /// 获取所有进程统计
    pub fn get_all_stats(&self) -> SqliteResult<Vec<ProcessStats>> {
        let mut stmt = self.conn.prepare(
            "SELECT name, total_starts, total_restarts, total_failures,
                    total_uptime_seconds, last_start_time, last_exit_code
             FROM process_stats
             ORDER BY name"
        )?;
        
        let stats = stmt.query_map([], |row| {
            let total_starts: i64 = row.get(1)?;
            let total_uptime: i64 = row.get(4)?;
            
            Ok(ProcessStats {
                name: row.get(0)?,
                total_starts: total_starts as u32,
                total_restarts: row.get::<_, i64>(2)? as u32,
                total_failures: row.get::<_, i64>(3)? as u32,
                avg_uptime_seconds: if total_starts > 0 {
                    total_uptime as f64 / total_starts as f64
                } else {
                    0.0
                },
                last_start_time: row.get::<_, Option<String>>(5)?
                    .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                    .map(|dt| dt.with_timezone(&Utc)),
                last_exit_code: row.get(6)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;
        
        Ok(stats)
    }
    
    /// 清理旧记录（保留最近 N 天）
    pub fn cleanup_old_records(&self, days_to_keep: u32) -> SqliteResult<usize> {
        let cutoff = Utc::now() - chrono::Duration::days(days_to_keep as i64);
        let cutoff_str = cutoff.to_rfc3339();
        
        let rows_deleted = self.conn.execute(
            "DELETE FROM process_history WHERE start_time < ?1 AND end_time IS NOT NULL",
            (cutoff_str,),
        )?;
        
        info!(days = days_to_keep, rows = rows_deleted, "Old records cleaned up");
        Ok(rows_deleted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_storage_basic() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        
        let storage = Storage::new(db_path).unwrap();
        
        // 记录启动
        let id = storage.record_start("test-process", 1234, true).unwrap();
        assert!(id > 0);
        
        // 记录结束
        storage.record_end("test-process", 1234, Some(0)).unwrap();
        
        // 获取历史
        let history = storage.get_history("test-process", 10).unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].name, "test-process");
        assert_eq!(history[0].pid, 1234);
    }
    
    #[test]
    fn test_stats() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        
        let storage = Storage::new(db_path).unwrap();
        
        // 记录多次启动
        storage.record_start("test", 1234, true).unwrap();
        storage.record_end("test", 1234, Some(0)).unwrap();
        
        storage.record_start("test", 5678, true).unwrap();
        storage.record_end("test", 5678, Some(1)).unwrap();
        
        // 获取统计
        let stats = storage.get_stats("test").unwrap().unwrap();
        assert_eq!(stats.total_starts, 2);
        assert_eq!(stats.total_failures, 1);
    }
}
