use crate::{formatters::Config, writer::LogWriter};

use super::Formatter;

pub struct DefaultFormatter<'a> {
    record: &'a log::Record<'a>,
    config: Config,
}

impl DefaultFormatter<'_> {
    pub fn new<'a>(record: &'a log::Record<'a>, config: Config) -> DefaultFormatter<'a> {
        DefaultFormatter { record, config }
    }
}

impl<'a> Formatter for DefaultFormatter<'a> {
    fn format(&self) -> String {
        let config = self.config.clone();

        let writer = LogWriter::new(self.record, config);

        format!(
            "{} {} {} {}",
            writer.get_datetime(),
            writer.get_level(),
            writer.get_target(),
            writer.get_args(),
        )
    }
}
