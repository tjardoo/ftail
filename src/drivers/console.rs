use log::{LevelFilter, Log};

use crate::{
    formatters::{default::DefaultFormatter, Formatter},
    Config,
};

/// A logger that logs messages to the console.
pub struct ConsoleLogger {
    config: Config,
}

impl ConsoleLogger {
    pub fn new(config: Config) -> Self {
        ConsoleLogger { config }
    }
}

impl Log for ConsoleLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        if self.config.level_filter == LevelFilter::Off {
            return true;
        }

        metadata.level() <= self.config.level_filter
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let formatter = DefaultFormatter::new(record, &self.config);

        println!("{}", formatter.format());
    }

    fn flush(&self) {}
}
