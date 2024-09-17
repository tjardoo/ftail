use log::{LevelFilter, Log};

use crate::{
    formatters::{readable::ReadableFormatter, Formatter},
    Config,
};

/// A logger that logs formatted messages to the console.
pub struct FormattedConsoleLogger {
    config: Config,
}

impl FormattedConsoleLogger {
    pub fn new(config: Config) -> Self {
        FormattedConsoleLogger { config }
    }
}

impl Log for FormattedConsoleLogger {
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

        let formatter = ReadableFormatter::new(record, &self.config);

        println!("{}", formatter.format());
    }

    fn flush(&self) {}
}
