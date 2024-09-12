use log::Log;

use crate::formatters::{readable::ReadableFormatter, Config, Formatter};

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
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let config = self.config.clone();

        let formatter = ReadableFormatter::new(record, config);

        println!("{}", formatter.format());
    }

    fn flush(&self) {}
}
