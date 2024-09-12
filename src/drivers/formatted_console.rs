use log::Log;

use crate::formatters::{readable::ReadableFormatter, Formatter};

/// A logger that logs formatted messages to the console.
pub struct FormattedConsoleLogger {}

impl Log for FormattedConsoleLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let formatter = ReadableFormatter::new(record);

        println!("{}", formatter.format());
    }

    fn flush(&self) {}
}
