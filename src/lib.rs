use drivers::{
    console::ConsoleLogger, daily::DailyLogger, single::SingleLogger, stdout::StdOutLogger,
};
use error::FtailError;
use log::Log;

mod ansi_escape;
pub mod drivers;
pub mod error;

pub(crate) struct LogDriver {
    driver: Box<dyn Log>,
    level: log::LevelFilter,
}

pub struct Ftail {
    drivers: Vec<LogDriver>,
}

impl Ftail {
    pub fn new() -> Self {
        Self {
            drivers: Vec::new(),
        }
    }

    fn add_driver(mut self, driver: Box<dyn Log>, level: log::LevelFilter) -> Self {
        self.drivers.push(LogDriver { driver, level });

        self
    }

    pub fn stdout(self, level: log::LevelFilter) -> Self {
        self.add_driver(Box::new(Box::new(StdOutLogger {})), level)
    }

    pub fn console(self, level: log::LevelFilter) -> Self {
        self.add_driver(Box::new(Box::new(ConsoleLogger {})), level)
    }

    pub fn single(self, path: &str, append: bool, level: log::LevelFilter) -> Self {
        self.add_driver(Box::new(SingleLogger::new(path, append)), level)
    }

    pub fn daily(self, path: &str, level: log::LevelFilter) -> Self {
        self.add_driver(Box::new(DailyLogger::new(path)), level)
    }

    pub fn custom(self, driver: Box<dyn Log>, level: log::LevelFilter) -> Self {
        self.add_driver(Box::new(driver), level)
    }

    pub fn init(self) -> Result<(), FtailError> {
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
            if driver.level >= record.level() {
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
