use crate::{ansi_escape::TextStyling, writer::LogWriter};

use super::Formatter;

pub struct ReadableFormatter<'a> {
    record: &'a log::Record<'a>,
}

impl ReadableFormatter<'_> {
    pub fn new<'a>(record: &'a log::Record<'a>) -> ReadableFormatter<'a> {
        ReadableFormatter { record }
    }
}

impl<'a> Formatter for ReadableFormatter<'a> {
    fn format(&self) -> String {
        let writer = LogWriter::new();

        let mut result = String::new();

        let level = match self.record.level() {
            log::Level::Trace => writer.get_level(self.record).bold().black(),
            log::Level::Debug => writer.get_level(self.record).bold().blue(),
            log::Level::Info => writer.get_level(self.record).bold().green(),
            log::Level::Warn => writer.get_level(self.record).bold().yellow(),
            log::Level::Error => writer.get_level(self.record).bold().red(),
        };

        result.push_str(&format!("{} · {}\n", writer.get_datetime().black(), level));
        result.push_str(&format!("{}\n", writer.get_args(self.record).bold()));

        let file = writer.get_file(self.record);
        let line = writer.get_line(self.record);

        if file.is_some() && line.is_some() {
            result.push_str(&format!(
                "{}{}{}\n",
                file.unwrap().black(),
                ":".black(),
                line.unwrap().black()
            ));
        }

        result
    }
}
