use drivers::{
    console::ConsoleLogger, daily::DailyLogger, formatted_console::FormattedConsoleLogger,
    single::SingleLogger,
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

pub(crate) struct LogDriver {
    driver: Box<dyn Log>,
    level: log::LevelFilter,
}

/// The main struct for configuring the logger.
pub struct Ftail {
    drivers: Vec<LogDriver>,
}

impl Ftail {
    /// Create a new instance of `Ftail`.
    pub fn new() -> Self {
        Self {
            drivers: Vec::new(),
        }
    }

    fn add_driver(mut self, driver: Box<dyn Log>, level: log::LevelFilter) -> Self {
        self.drivers.push(LogDriver { driver, level });

        self
    }

    /// Add a driver that logs messages to the console.
    pub fn console(self, level: log::LevelFilter) -> Self {
        self.add_driver(Box::new(Box::new(ConsoleLogger {})), level)
    }

    /// Add a driver that logs formatted messages to the console.
    pub fn formatted_console(self, level: log::LevelFilter) -> Self {
        self.add_driver(Box::new(Box::new(FormattedConsoleLogger {})), level)
    }

    /// Add a driver that logs messages to a single file.
    pub fn single(self, path: &str, append: bool, level: log::LevelFilter) -> Self {
        self.add_driver(Box::new(SingleLogger::new(path, append).unwrap()), level)
    }

    /// Add a driver that logs messages to a daily log file.
    pub fn daily(self, path: &str, level: log::LevelFilter) -> Self {
        self.add_driver(Box::new(DailyLogger::new(path).unwrap()), level)
    }

    /// Add a custom driver.
    pub fn custom(self, driver: Box<dyn Log>, level: log::LevelFilter) -> Self {
        self.add_driver(Box::new(driver), level)
    }

    /// Initialize the logger.
    pub fn init(self) -> Result<(), FtailError> {
        if self.drivers.is_empty() {
            return Err(FtailError::NoDriversError);
        }

        log::set_max_level(log::LevelFilter::Trace);
        log::set_boxed_logger(Box::new(self)).map_err(FtailError::SetLoggerError)
    }
}

impl Log for Ftail {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        self.drivers
            .iter()
            .any(|driver| metadata.level() <= driver.level && driver.driver.enabled(metadata))
    }

    fn log(&self, record: &log::Record) {
        for driver in &self.drivers {
            if driver.level >= record.level() || driver.level == log::LevelFilter::Off {
                driver.driver.log(record);
            }
        }
    }

    fn flush(&self) {
        for driver in &self.drivers {
            driver.driver.flush();
        }
    }
}

impl Default for Ftail {
    fn default() -> Self {
        Self::new()
    }
}
