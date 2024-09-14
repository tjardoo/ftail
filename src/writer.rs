use log::Record;

use crate::Config;

pub(crate) struct LogWriter<'a> {
    record: &'a Record<'a>,
    config: &'a Config,
}

impl<'a> LogWriter<'a> {
    pub fn new(record: &'a Record<'a>, config: &'a Config) -> LogWriter<'a> {
        LogWriter { record, config }
    }

    pub fn get_datetime(&self) -> String {
        #[cfg(not(feature = "timezone"))]
        return chrono::Local::now()
            .format(&self.config.datetime_format)
            .to_string();

        #[cfg(feature = "timezone")]
        return chrono::Local::now()
            .with_timezone(&self.config.timezone)
            .format(&self.config.datetime_format)
            .to_string();
    }

    pub fn get_level(&self) -> String {
        self.record.level().to_string()
    }

    pub fn get_target(&self) -> String {
        self.record.target().to_string()
    }

    pub fn get_args(&self) -> String {
        self.record.args().to_string()
    }

    pub fn get_file(&self) -> Option<String> {
        self.record.file().map(|f| f.to_string())
    }

    pub fn get_line(&self) -> Option<u32> {
        self.record.line()
    }
}
