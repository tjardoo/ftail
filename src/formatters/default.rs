use crate::{formatters::Config, writer::LogWriter};

use super::Formatter;

pub struct DefaultFormatter<'a> {
    record: &'a log::Record<'a>,
    config: &'a Config,
}

impl DefaultFormatter<'_> {
    pub fn new<'a>(record: &'a log::Record<'a>, config: &'a Config) -> DefaultFormatter<'a> {
        DefaultFormatter { record, config }
    }
}

impl Formatter for DefaultFormatter<'_> {
    fn format(&self) -> String {
        let writer = LogWriter::new(self.record, self.config);

        format!(
            "{} {} {} {}",
            writer.get_datetime(),
            writer.get_level(),
            writer.get_target(),
            writer.get_args(),
        )
    }
}
