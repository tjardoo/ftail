//! # Ftail
//!
//! Ftail is simple logging implementation for the `log` crate with support for multiple drivers.
//!
//! - [Console](#console)
//! - [Formatted console](#formatted-console)
//! - [Single file](#single-file)
//! - [Daily file](#daily-file)
//! - [Custom driver](#custom-driver)
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
//!     .timezone(chrono_tz::Europe::Amsterdam) // optional (default is UTC)
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
//! ## Drivers
//!
//! ### Console
//!
//! Logs to the standard output without any formatting.
//!
//! The `stdout` driver takes the following parameters:
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
//! The `console` driver takes the following parameters:
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
//! The `single_file` driver takes the following parameters:
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
//! The `daily_file` driver takes the following parameters:
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
//! ### Custom driver
//!
//! Create your own log driver.
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
//!     fn enabled(&self, _metadata: &log::Metadata) -> bool {
//!         true
//!     }
//!
//!     fn log(&self, record: &log::Record) {
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

use drivers::{
    console::ConsoleLogger, daily_file::DailyFileLogger, formatted_console::FormattedConsoleLogger,
    single_file::SingleFileLogger,
};
use error::FtailError;
use log::Log;

/// Module containing the ANSI escape codes.
pub mod ansi_escape;
/// Module containing the drivers.
pub mod drivers;
/// Module containing the error type.
pub mod error;
mod formatters;
mod writer;

/// The main struct for configuring the logger.
pub struct Ftail {
    drivers: Vec<LogDriver>,
    initialized_drivers: Vec<InitializedLogDriver>,
    config: Config,
}

unsafe impl Send for Ftail {}
unsafe impl Sync for Ftail {}

pub(crate) struct LogDriver {
    constructor: Box<dyn Fn(Config) -> Box<dyn Log + Send + Sync>>,
    level: log::LevelFilter,
}

pub(crate) struct InitializedLogDriver {
    driver: Box<dyn Log + Send + Sync>,
    level: log::LevelFilter,
}

/// The configuration struct for the logger. Required for custom drivers.
#[derive(Clone)]
pub struct Config {
    pub datetime_format: String,
    pub timezone: chrono_tz::Tz,
}

impl Ftail {
    /// Create a new instance of `Ftail`.
    pub fn new() -> Self {
        Self {
            drivers: Vec::new(),
            initialized_drivers: Vec::new(),
            config: Config::new(),
        }
    }

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

    fn add_driver<F>(mut self, constructor: F, level: log::LevelFilter) -> Self
    where
        F: Fn(Config) -> Box<dyn Log + Send + Sync> + 'static,
    {
        self.drivers.push(LogDriver::new(constructor, level));
        self
    }

    /// Add a driver that logs messages to the console.
    pub fn console(self, level: log::LevelFilter) -> Self {
        let constructor =
            |config: Config| Box::new(ConsoleLogger::new(config)) as Box<dyn Log + Send + Sync>;

        self.add_driver(constructor, level)
    }

    /// Add a driver that logs formatted messages to the console.
    pub fn formatted_console(self, level: log::LevelFilter) -> Self {
        let constructor = |config: Config| {
            Box::new(FormattedConsoleLogger::new(config)) as Box<dyn Log + Send + Sync>
        };

        self.add_driver(constructor, level)
    }

    /// Add a driver that logs messages to a single file.
    pub fn single_file(self, path: &str, append: bool, level: log::LevelFilter) -> Self {
        let path = path.to_string();

        let constructor = move |config: Config| {
            Box::new(SingleFileLogger::new(&path, append, config).unwrap())
                as Box<dyn Log + Send + Sync>
        };

        self.add_driver(constructor, level)
    }

    /// Add a driver that logs messages to a daily log file.
    pub fn daily_file(self, path: &str, level: log::LevelFilter) -> Self {
        let path = path.to_string();

        let constructor = move |config: Config| {
            Box::new(DailyFileLogger::new(&path, config).unwrap()) as Box<dyn Log + Send + Sync>
        };

        self.add_driver(constructor, level)
    }

    /// Add a custom driver.
    pub fn custom<F>(self, constructor: F, level: log::LevelFilter) -> Self
    where
        F: Fn(Config) -> Box<dyn Log + Send + Sync> + 'static,
    {
        self.add_driver(constructor, level)
    }

    /// Initialize the logger.
    pub fn init(mut self) -> Result<(), FtailError> {
        if self.drivers.is_empty() {
            return Err(FtailError::NoDriversError);
        }

        let drivers = std::mem::take(&mut self.drivers);

        self.initialized_drivers = drivers
            .into_iter()
            .map(|driver| driver.init(self.config.clone()))
            .collect();

        log::set_max_level(log::LevelFilter::Trace);
        log::set_boxed_logger(Box::new(self)).map_err(FtailError::SetLoggerError)
    }
}

impl LogDriver {
    fn new<F>(constructor: F, level: log::LevelFilter) -> Self
    where
        F: Fn(Config) -> Box<dyn Log + Send + Sync> + 'static,
    {
        Self {
            constructor: Box::new(constructor),
            level,
        }
    }

    fn init(self, config: Config) -> InitializedLogDriver {
        InitializedLogDriver {
            driver: (self.constructor)(config),
            level: self.level,
        }
    }
}

impl Log for Ftail {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        self.initialized_drivers
            .iter()
            .any(|driver| metadata.level() <= driver.level && driver.driver.enabled(metadata))
    }

    fn log(&self, record: &log::Record) {
        for driver in &self.initialized_drivers {
            if driver.level >= record.level() || driver.level == log::LevelFilter::Off {
                driver.driver.log(record);
            }
        }
    }

    fn flush(&self) {
        for driver in &self.initialized_drivers {
            driver.driver.flush();
        }
    }
}

impl Default for Ftail {
    fn default() -> Self {
        Self::new()
    }
}
