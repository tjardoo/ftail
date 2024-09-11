use log::Log;

use crate::formatters::{readable::ReadableFormatter, Formatter};

pub struct ConsoleLogger {}

impl Log for ConsoleLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let formatter = ReadableFormatter::new(record);

        println!("{}", formatter.format());
    }

    fn flush(&self) {}
}
