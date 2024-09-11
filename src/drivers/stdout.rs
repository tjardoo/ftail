use log::Log;

use crate::formatters::{default::DefaultFormatter, Formatter};

pub struct StdOutLogger {}

impl Log for StdOutLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let formatter = DefaultFormatter::new(record);

        println!("{}", formatter.format());
    }

    fn flush(&self) {}
}
