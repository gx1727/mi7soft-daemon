use std::path::PathBuf;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Initialize logging system
/// 
/// Log files are stored in the same directory as the log_file config
/// - rotation: daily
/// - max files: keep 7 days
pub fn init_logging(log_file: Option<&str>, verbose: u8) -> anyhow::Result<()> {
    let log_dir = if let Some(path) = log_file {
        PathBuf::from(path).parent().map(|p| p.to_path_buf())
    } else {
        None
    };

    let env_filter = if verbose > 0 {
        EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info"))
    } else {
        EnvFilter::new("mi7soft_daemon=info,warn")
    };

    let subscriber = tracing_subscriber::registry()
        .with(env_filter);

    // If log file specified, use file appender
    if let Some(path) = log_file {
        let file_appender = RollingFileAppender::new(
            Rotation::DAILY,
            log_dir.unwrap_or_else(|| PathBuf::from("/var/log")),
            path.split('/').last().unwrap_or("mi7soft-daemon.log"),
        );

        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

        // Keep the guard alive for the lifetime of the program
        // Note: In production, this should be stored in a static or passed around
        std::mem::forget(_guard);

        subscriber
            .with(fmt::layer().with_writer(non_blocking).with_ansi(false))
            .init();
    } else {
        // Console only
        subscriber
            .with(fmt::layer())
            .init();
    }

    tracing::info!("Logging initialized");
    Ok(())
}
