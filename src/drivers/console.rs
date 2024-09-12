use log::Log;

use crate::formatters::{default::DefaultFormatter, Formatter};

/// A logger that logs messages to the console.
pub struct ConsoleLogger {}

impl Log for ConsoleLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let formatter = DefaultFormatter::new(record);

        println!("{}", formatter.format());
    }

    fn flush(&self) {}
}
