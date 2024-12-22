//! # Ftail
//!
//! Ftail is simple logging implementation for the `log` crate with support for multiple channels.
//!
//! - [Console](#console)
//! - [Formatted console](#formatted-console)
//! - [Single file](#single-file)
//! - [Daily file](#daily-file)
//! - [Custom channel](#custom-channel)
//!
//! ## Usage
//!
//! Add the following dependencies to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! ftail = "0.1"
//! ```
//!
//! Add the following code to your `main.rs` or `lib.rs` file:
//!
//! ```rust
//! use ftail::Ftail;
//! use log::LevelFilter;
//!
//! Ftail::new()
//!     .console(LevelFilter::Debug)
//!     .daily_file("logs", LevelFilter::Error)
//!     .init()?;
//!
//! // log messages anywhere in your code
//! log::trace!("This is a trace message");
//! log::debug!("This is a debug message");
//! log::info!(target: "foo", "bar");
//! log::warn!("This is a warning message");
//! log::error!("This is an error message");
//! ```
//!
//! You can set the following configuration options:
//!
//! - `.datetime_format("%Y-%m-%d %H:%M:%S.3f")` to set the datetime format
//! - `.timezone(ftail::Tz::UTC)` to set the timezone [requires feature `timezone`]
//! - `.max_file_size(100)` to set the maximum file size in MB (will move older logs to .old{N})
//! - `.filter_levels(vec![Level::Debug, Level::Error])` only log messages with the specified levels
//! - `.filter_targets(vec!["foo", "bar"])` only log messages with the specified targets
//!
//! ## Channels
//!
//! ### Console
//!
//! Logs to the standard output without any formatting.
//!
//! The `stdout` channel takes the following parameters:
//!
//! - `level`: the minumum log level to log
//!
//! ```rust
//! Ftail::new()
//!     .console(LevelFilter::Trace)
//!     .init()?;
//! ```
//!
//! ```sh
//! 13-09-2024 17:35:18 TRACE console This is a trace message
//! 13-09-2024 17:35:18 DEBUG console This is a debug message
//! 13-09-2024 17:35:18 INFO foo bar
//! 13-09-2024 17:35:18 WARN console This is a warning message
//! 13-09-2024 17:35:18 ERROR console This is an error message
//! ```
//!
//! ### Formatted Console
//!
//! Logs to the standard output with formatted and colored output.
//!
//! The `console` channel takes the following parameters:
//!
//! - `level`: the minumum log level to log
//!
//! ```rust
//! Ftail::new()
//!     .formatted_console(LevelFilter::Trace)
//!     .init()?;
//! ```
//!
//! ```sh
//! 2024-09-13 17:35:37 · TRACE
//! This is a trace message
//! examples\formatted_console\src/main.rs:9
//!
//! 2024-09-13 17:35:37 · DEBUG
//! This is a debug message
//! examples\formatted_console\src/main.rs:11
//!
//! 2024-09-13 17:35:37 · INFO
//! bar
//! examples\formatted_console\src/main.rs:13
//!
//! 2024-09-13 17:35:37 · WARN
//! This is a warning message
//! examples\formatted_console\src/main.rs:15
//!
//! 2024-09-13 17:35:37 · ERROR
//! This is an error message
//! examples\formatted_console\src/main.rs:17
//! ```
//!
//! ### Single file
//!
//! Logs to the single log file `logs/demo.log`.
//!
//! The `single_file` channel takes the following parameters:
//!
//! - `path`: the path to the log file
//! - `append`: whether to append to the log file or overwrite it
//! - `level`: the minumum log level to log
//!
//! ```rust
//! Ftail::new()
//!     .single_file("logs/demo.log", true, LevelFilter::Trace)
//!     .init()?;
//! ```
//!
//! ### Daily file
//!
//! Logs to a daily log file in the `logs` directory. The log files have the following format: `YYYY-MM-DD.log`.
//!
//! The `daily_file` channel takes the following parameters:
//!
//! - `dir`: the directory to store the log files
//! - `level`: the minumum log level to log
//!
//! ```rust
//! Ftail::new()
//!     .daily_file("logs", LevelFilter::Trace)
//!     .init()?;
//! ```
//!
//! ### Custom channel
//!
//! Create your own log channel.
//!
//! ```rust
//! Ftail::new()
//!     .custom(
//!         |config: ftail::Config| Box::new(CustomLogger { config }) as Box<dyn Log + Send + Sync>,
//!         LevelFilter::Debug,
//!     )
//!     .datetime_format("%H:%M:%S%.3f")
//!     .init()?;
//!
//! // the custom logger implementation
//! struct CustomLogger {
//!     config: Config,
//! }
//!
//! impl Log for CustomLogger {
//!     fn enabled(&self, metadata: &log::Metadata) -> bool {
//!         if self.config.level_filter == LevelFilter::Off {
//!             return true;
//!         }
//!
//!         metadata.level() <= self.config.level_filter
//!     }
//!
//!     fn log(&self, record: &log::Record) {
//!         if !self.enabled(record.metadata()) {
//!             return;
//!         }
//!
//!         let time = chrono::Local::now()
//!             .format(&self.config.datetime_format)
//!             .to_string();
//!
//!         println!("{} [{}] {}", time.black(), record.level().bold(), record.args());
//!     }
//!
//!     fn flush(&self) {}
//! }
//! ```
//!
//! ```sh
//! 19:37:22.402 [DEBUG] This is a debug message
//! 19:37:22.403 [INFO] bar
//! 19:37:22.403 [WARN] This is a warning message
//! 19:37:22.403 [ERROR] This is an error message
//! ```

use channels::{
    console::ConsoleLogger, daily_file::DailyFileLogger, formatted_console::FormattedConsoleLogger,
    single_file::SingleFileLogger,
};
use error::FtailError;
use log::{Level, LevelFilter, Log};

#[cfg(feature = "timezone")]
pub use chrono_tz::Tz;

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
    pub fn console(self, level: log::LevelFilter) -> Self {
        let constructor =
            |config: Config| Box::new(ConsoleLogger::new(config)) as Box<dyn Log + Send + Sync>;

        self.add_channel(constructor, level)
    }

    /// Add a channel that logs formatted messages to the console.
    pub fn formatted_console(self, level: log::LevelFilter) -> Self {
        let constructor = |config: Config| {
            Box::new(FormattedConsoleLogger::new(config)) as Box<dyn Log + Send + Sync>
        };

        self.add_channel(constructor, level)
    }

    /// Add a channel that logs messages to a single file.
    pub fn single_file(self, path: &str, append: bool, level: log::LevelFilter) -> Self {
        let path = path.to_string();

        let constructor = move |config: Config| {
            Box::new(SingleFileLogger::new(&path, append, config).unwrap())
                as Box<dyn Log + Send + Sync>
        };

        self.add_channel(constructor, level)
    }

    /// Add a channel that logs messages to a daily log file.
    pub fn daily_file(self, path: &str, level: log::LevelFilter) -> Self {
        let path = path.to_string();

        let constructor = move |config: Config| {
            Box::new(DailyFileLogger::new(&path, config).unwrap()) as Box<dyn Log + Send + Sync>
        };

        self.add_channel(constructor, level)
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
