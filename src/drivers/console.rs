use log::Log;

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
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let formatter = DefaultFormatter::new(record, &self.config);

        println!("{}", formatter.format());
    }

    fn flush(&self) {}
}
