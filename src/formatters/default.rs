use crate::writer::LogWriter;

use super::Formatter;

pub struct DefaultFormatter<'a> {
    record: &'a log::Record<'a>,
}

impl DefaultFormatter<'_> {
    pub fn new<'a>(record: &'a log::Record<'a>) -> DefaultFormatter<'a> {
        DefaultFormatter { record }
    }
}

impl<'a> Formatter for DefaultFormatter<'a> {
    fn format(&self) -> String {
        let writer = LogWriter::new();

        format!(
            "{} {} {} {}",
            writer.get_datetime(),
            writer.get_level(self.record),
            writer.get_target(self.record),
            writer.get_args(self.record),
        )
    }
}
