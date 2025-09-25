//! # Ftail
//!
//! **Ftail** is a simple logging implementation for the [`log`](https://crates.io/crates/log) crate with support for multiple channels, including console output, files, and custom loggers.
//!
//! ---
//!
//! ## Features
//!
//! * Multiple logging channels: console, formatted console, single file, daily file, and custom
//! * Level and target filtering
//! * Optional timestamp formatting and timezone support
//! * Automatic log rotation and retention
//! * Use `RUST_LOG` environment variable for dynamic log level control (use '_env_level' postfix)
//!
//! ---
//!
//! ## Quick Start
//!
//! Add Ftail to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! ftail = "0.3"
//! ```
//!
//! Initialize Ftail in your `main.rs` or `lib.rs`:
//!
//! ```rust
//! use ftail::Ftail;
//! use log::LevelFilter;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     Ftail::new()
//!         .console(LevelFilter::Info)        // log to console
//!         .daily_file("logs", LevelFilter::Error) // log errors to daily files
//!         .init()?; // initialize logger
//!
//!     log::info!("Hello, Ftail!");
//!     log::error!("This is an error message");
//!     Ok(())
//! }
//! ```
//!
//! ---
//!
//! ## Configuration Options
//!
//! | Option               | Description                      | Notes                         |
//! | -------------------- | -------------------------------- | ----------------------------- |
//! | `.datetime_format()` | Set the datetime format          | Ex: `"%Y-%m-%d %H:%M:%S%.3f"` |
//! | `.timezone()`        | Set the timezone                 | Requires `timezone` feature   |
//! | `.max_file_size()`   | Maximum file size in MB          | Older logs renamed `.old{N}`  |
//! | `.retention_days()`  | Number of days to keep log files | Daily file only               |
//! | `.filter_levels()`   | Log only the specified levels    | `Vec<Level>`                  |
//! | `.filter_targets()`  | Log only the specified targets   | `Vec<&str>`                   |
//!
//! ---
//!
//! ## Channels
//!
//! ### Console
//!
//! Logs to the standard output without formatting.
//!
//! ```rust
//! Ftail::new()
//!     .console(LevelFilter::Trace)
//!     .init()?;
//! ```
//!
//! **Output:**
//!
//! ```text
//! 2024-09-13 17:35:18 TRACE console This is a trace message
//! 2024-09-13 17:35:18 DEBUG console This is a debug message
//! 2024-09-13 17:35:18 INFO foo bar
//! 2024-09-13 17:35:18 WARN console This is a warning message
//! 2024-09-13 17:35:18 ERROR console This is an error message
//! ```
//!
//! ---
//!
//! ### Formatted Console
//!
//! Logs with formatted and colored output.
//!
//! ```rust
//! Ftail::new()
//!     .formatted_console(LevelFilter::Trace)
//!     .init()?;
//! ```
//!
//! **Output:**
//!
//! ```text
//! 2024-09-13 17:35:37 · TRACE
//! This is a trace message
//! examples/formatted_console/src/main.rs:9
//!
//! 2024-09-13 17:35:37 · DEBUG
//! This is a debug message
//! examples/formatted_console/src/main.rs:11
//! ```
//!
//! ---
//!
//! ### Single File
//!
//! Logs to a single file (e.g., `logs/demo.log`).
//!
//! ```rust
//! Ftail::new()
//!     .single_file(Path::new("logs/demo.log"), true, LevelFilter::Trace)
//!     .init()?;
//! ```
//!
//! * `append = true` keeps existing logs, `false` overwrites.
//!
//! ---
//!
//! ### Daily File
//!
//! Logs to daily files in a directory (e.g., `logs/2025-09-25.log`).
//!
//! ```rust
//! Ftail::new()
//!     .daily_file(Path::new("logs"), LevelFilter::Trace)
//!     .init()?;
//! ```
//!
//! * Automatically rotates files per day.
//!
//! ---
//!
//! ### Custom Channel
//!
//! Create your own logger by implementing the `log::Log` trait:
//!
//! ```rust
//! Ftail::new()
//!     .custom(
//!         |config| Box::new(CustomLogger { config }) as Box<dyn Log + Send + Sync>,
//!         LevelFilter::Debug,
//!     )
//!     .datetime_format("%H:%M:%S%.3f")
//!     .init()?;
//!
//! struct CustomLogger {
//!     config: ftail::Config,
//! }
//!
//! impl log::Log for CustomLogger {
//!     fn enabled(&self, metadata: &log::Metadata) -> bool {
//!         metadata.level() <= self.config.level_filter
//!     }
//!
//!     fn log(&self, record: &log::Record) {
//!         if !self.enabled(record.metadata()) {
//!             return;
//!         }
//!         let time = chrono::Local::now()
//!             .format(&self.config.datetime_format)
//!             .to_string();
//!         println!("[{}] {}: {}", time, record.level(), record.args());
//!     }
//!
//!     fn flush(&self) {}
//! }
//! ```
//!
//! **Output:**
//!
//! ```text
//! 19:37:22.402 [DEBUG] This is a debug message
//! 19:37:22.403 [INFO] bar
//! 19:37:22.403 [WARN] This is a warning message
//! 19:37:22.403 [ERROR] This is an error message
//! ```
//!
//! ---
//!
//! ## Tips
//!
//! * Use `.console(LevelFilter::Debug)` for development.
//! * Use `.daily_file()` in production to organize logs by date.
//! * Combine `.max_file_size()` and `.retention_days()` to prevent disk bloat.
//! * Only enable the channels you need to reduce overhead by disabling default features in `Cargo.toml`.

#[cfg(any(
    feature = "console",
    feature = "formatted_console",
    feature = "file_channels"
))]
use crate::helpers::get_env_log_level;
#[cfg(feature = "console")]
use channels::console::ConsoleLogger;
#[cfg(feature = "daily_file")]
use channels::daily_file::DailyFileLogger;
#[cfg(feature = "formatted_console")]
use channels::formatted_console::FormattedConsoleLogger;
#[cfg(feature = "single_file")]
use channels::single_file::SingleFileLogger;
#[cfg(feature = "timezone")]
pub use chrono_tz::Tz;
use error::FtailError;
use log::{Level, LevelFilter, Log};
#[cfg(feature = "file_channels")]
use std::path::Path;

/// Module containing the ANSI escape codes.
pub mod ansi_escape;
/// Module containing the channels.
pub mod channels;
/// Module containing the error type.
pub mod error;
mod formatters;
mod helpers;
#[cfg(test)]
mod tests;
mod writer;

/// The main struct for configuring the logger.
pub struct Ftail {
    channels: Vec<LogChannel>,
    initialized_channels: Vec<InitializedLogChannel>,
    config: Config,
}

unsafe impl Send for Ftail {}
unsafe impl Sync for Ftail {}

pub(crate) struct LogChannel {
    constructor: Box<dyn Fn(Config) -> Box<dyn Log + Send + Sync>>,
    level: log::LevelFilter,
}

pub(crate) struct InitializedLogChannel {
    channel: Box<dyn Log + Send + Sync>,
}

/// The configuration struct for the logger. Required for custom channels.
#[derive(Clone)]
pub struct Config {
    pub level_filter: LevelFilter,
    pub datetime_format: String,
    #[cfg(feature = "timezone")]
    pub timezone: chrono_tz::Tz,
    pub max_file_size: Option<u64>,
    pub retention_days: Option<u64>,
    pub levels: Option<Vec<Level>>,
    pub targets: Option<Vec<String>>,
}

impl Ftail {
    /// Create a new instance of `Ftail`.
    pub fn new() -> Self {
        Self {
            channels: Vec::new(),
            initialized_channels: Vec::new(),
            config: Config::new(),
        }
    }

    #[cfg(feature = "timezone")]
    /// Set the timezone for the logger.
    pub fn timezone(mut self, timezone: chrono_tz::Tz) -> Self {
        self.config.timezone = timezone;

        self
    }

    /// Set the datetime format for the logger.
    pub fn datetime_format(mut self, datetime_format: &str) -> Self {
        self.config.datetime_format = datetime_format.to_string();

        self
    }

    /// Set the maximum file size for the logger.
    pub fn max_file_size(mut self, max_file_size_in_mb: u64) -> Self {
        self.config.max_file_size = Some(max_file_size_in_mb * 1024 * 1024);

        self
    }

    /// Set the retention days for the logger (daily file logger only).
    pub fn retention_days(mut self, retention_days: u64) -> Self {
        self.config.retention_days = Some(retention_days);

        self
    }

    /// Only log messages with the specified levels. The default is to log all levels.
    pub fn filter_levels(mut self, levels: Vec<Level>) -> Self {
        self.config.levels = Some(levels);

        self
    }

    /// Only log messages with the specified targets. The default is to log all targets.
    pub fn filter_targets(mut self, targets: Vec<&str>) -> Self {
        self.config.targets = Some(targets.iter().map(|s| s.to_string()).collect());

        self
    }

    fn add_channel<F>(mut self, constructor: F, level: log::LevelFilter) -> Self
    where
        F: Fn(Config) -> Box<dyn Log + Send + Sync> + 'static,
    {
        self.channels.push(LogChannel::new(constructor, level));
        self
    }

    /// Add a channel that logs messages to the console.
    #[cfg(feature = "console")]
    pub fn console(self, level: log::LevelFilter) -> Self {
        let constructor =
            |config: Config| Box::new(ConsoleLogger::new(config)) as Box<dyn Log + Send + Sync>;

        self.add_channel(constructor, level)
    }

    // Add a channel that logs messages to the console with the log level set from the environment variable `RUST_LOG`.
    #[cfg(feature = "console")]
    pub fn console_env_level(self) -> Self {
        self.console(get_env_log_level())
    }

    /// Add a channel that logs formatted messages to the console.
    #[cfg(feature = "formatted_console")]
    pub fn formatted_console(self, level: log::LevelFilter) -> Self {
        let constructor = |config: Config| {
            Box::new(FormattedConsoleLogger::new(config)) as Box<dyn Log + Send + Sync>
        };

        self.add_channel(constructor, level)
    }

    /// Add a channel that logs formatted messages to the console with the log level set from the environment variable `RUST_LOG`.
    #[cfg(feature = "formatted_console")]
    pub fn formatted_console_env_level(self) -> Self {
        self.formatted_console(get_env_log_level())
    }

    /// Add a channel that logs messages to a single file.
    #[cfg(feature = "single_file")]
    pub fn single_file(self, path: &Path, append: bool, level: log::LevelFilter) -> Self {
        let path = path.to_owned();

        let constructor = move |config: Config| {
            Box::new(SingleFileLogger::new(&path, append, config).unwrap())
                as Box<dyn Log + Send + Sync>
        };

        self.add_channel(constructor, level)
    }

    /// Add a channel that logs messages to a single file with the log level set from the environment variable `RUST_LOG`.
    #[cfg(feature = "single_file")]
    pub fn single_file_env_level(self, path: &Path, append: bool) -> Self {
        self.single_file(path, append, get_env_log_level())
    }

    /// Add a channel that logs messages to a daily log file.
    #[cfg(feature = "daily_file")]
    pub fn daily_file(self, path: &Path, level: log::LevelFilter) -> Self {
        let path = path.to_owned();

        let constructor = move |config: Config| {
            Box::new(DailyFileLogger::new(&path, config).unwrap()) as Box<dyn Log + Send + Sync>
        };

        self.add_channel(constructor, level)
    }

    /// Add a channel that logs messages to a daily log file with the log level set from the environment variable `RUST_LOG`.
    #[cfg(feature = "daily_file")]
    pub fn daily_file_env_level(self, path: &Path) -> Self {
        self.daily_file(path, get_env_log_level())
    }

    /// Add a custom channel.
    pub fn custom<F>(self, constructor: F, level: log::LevelFilter) -> Self
    where
        F: Fn(Config) -> Box<dyn Log + Send + Sync> + 'static,
    {
        self.add_channel(constructor, level)
    }

    /// Initialize the logger.
    pub fn init(mut self) -> Result<(), FtailError> {
        if self.channels.is_empty() {
            return Err(FtailError::NoChannelsError);
        }

        let channels = std::mem::take(&mut self.channels);

        self.initialized_channels = channels
            .into_iter()
            .map(|channel| {
                let mut config = self.config.clone();
                config.level_filter = channel.level;

                channel.init(config)
            })
            .collect();

        log::set_max_level(log::LevelFilter::Trace);
        log::set_boxed_logger(Box::new(self)).map_err(FtailError::SetLoggerError)
    }
}

impl LogChannel {
    fn new<F>(constructor: F, level: log::LevelFilter) -> Self
    where
        F: Fn(Config) -> Box<dyn Log + Send + Sync> + 'static,
    {
        Self {
            constructor: Box::new(constructor),
            level,
        }
    }

    fn init(self, config: Config) -> InitializedLogChannel {
        InitializedLogChannel {
            channel: (self.constructor)(config),
        }
    }
}

impl Log for Ftail {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        if self.config.levels.is_some()
            && !self
                .config
                .levels
                .as_ref()
                .unwrap()
                .contains(&metadata.level())
        {
            return false;
        }

        if self.config.targets.is_some()
            && !self
                .config
                .targets
                .as_ref()
                .unwrap()
                .contains(&metadata.target().to_string())
        {
            return false;
        }

        true
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        for channel in &self.initialized_channels {
            channel.channel.log(record);
        }
    }

    fn flush(&self) {
        for channel in &self.initialized_channels {
            channel.channel.flush();
        }
    }
}

impl Default for Ftail {
    fn default() -> Self {
        Self::new()
    }
}
