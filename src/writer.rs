use log::Record;

pub(crate) struct LogWriter {}

impl LogWriter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_datetime(&self) -> String {
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
    }

    pub fn get_level(&self, record: &Record) -> String {
        record.level().to_string()
    }

    pub fn get_target(&self, record: &Record) -> String {
        record.target().to_string()
    }

    pub fn get_args(&self, record: &Record) -> String {
        record.args().to_string()
    }

    pub fn get_file(&self, record: &Record) -> Option<String> {
        record.file().map(|f| f.to_string())
    }

    pub fn get_line(&self, record: &Record) -> Option<u32> {
        record.line()
    }
}
