use crate::{ansi_escape::TextStyling, writer::LogWriter, Config};

use super::Formatter;

pub struct ReadableFormatter<'a> {
    record: &'a log::Record<'a>,
    config: &'a Config,
}

impl ReadableFormatter<'_> {
    pub fn new<'a>(record: &'a log::Record<'a>, config: &'a Config) -> ReadableFormatter<'a> {
        ReadableFormatter { record, config }
    }
}

impl Formatter for ReadableFormatter<'_> {
    fn format(&self) -> String {
        let writer = LogWriter::new(self.record, self.config);

        let mut result = String::new();

        let level = match self.record.level() {
            log::Level::Trace => writer.get_level().bold().black(),
            log::Level::Debug => writer.get_level().bold().blue(),
            log::Level::Info => writer.get_level().bold().green(),
            log::Level::Warn => writer.get_level().bold().yellow(),
            log::Level::Error => writer.get_level().bold().red(),
        };

        result.push_str(&format!("{} · {}\n", writer.get_datetime().black(), level));
        result.push_str(&format!("{}\n", writer.get_args().bold()));

        let file = writer.get_file();
        let line = writer.get_line();

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
