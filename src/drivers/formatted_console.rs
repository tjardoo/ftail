use log::Log;

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
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let formatter = ReadableFormatter::new(record, &self.config);

        println!("{}", formatter.format());
    }

    fn flush(&self) {}
}
