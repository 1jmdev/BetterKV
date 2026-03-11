use std::fs::OpenOptions;
use std::path::Path;

use crate::config::Config;

pub struct LoggingGuard {
    pub file_guard: Option<tracing_appender::non_blocking::WorkerGuard>,
}

pub fn init_logging(config: &Config) -> Result<LoggingGuard, String> {
    let max_level = parse_level(&config.log_level)?;

    if let Some(log_file) = &config.log_file {
        let path = Path::new(log_file);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|err| {
                format!("failed to create log directory {}: {err}", parent.display())
            })?;
        }

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .map_err(|err| format!("failed to open log file {}: {err}", path.display()))?;

        let (writer, guard) = tracing_appender::non_blocking(file);
        tracing_subscriber::fmt()
            .with_max_level(max_level)
            .with_target(true)
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_ansi(false)
            .with_writer(writer)
            .try_init()
            .map_err(|err| format!("failed to initialize logging: {err}"))?;

        Ok(LoggingGuard {
            file_guard: Some(guard),
        })
    } else {
        tracing_subscriber::fmt()
            .with_max_level(max_level)
            .with_target(true)
            .with_thread_ids(true)
            .with_thread_names(true)
            .try_init()
            .map_err(|err| format!("failed to initialize logging: {err}"))?;

        Ok(LoggingGuard { file_guard: None })
    }
}

fn parse_level(raw: &str) -> Result<tracing::Level, String> {
    match raw.to_ascii_lowercase().as_str() {
        "trace" => Ok(tracing::Level::TRACE),
        "debug" => Ok(tracing::Level::DEBUG),
        "info" => Ok(tracing::Level::INFO),
        "warn" | "warning" => Ok(tracing::Level::WARN),
        "error" => Ok(tracing::Level::ERROR),
        _ => Err(format!("invalid log level '{raw}'")),
    }
}
