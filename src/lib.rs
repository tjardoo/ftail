use log::Log;

mod ansi_escape;
pub mod drivers;

pub struct LogDriver {
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

    pub fn add_driver(mut self, driver: Box<dyn Log>, level: log::LevelFilter) -> Self {
        self.drivers.push(LogDriver { driver, level });

        self
    }

    pub fn init(self) -> Result<(), log::SetLoggerError> {
        log::set_max_level(log::LevelFilter::Trace);
        log::set_boxed_logger(Box::new(self))
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
