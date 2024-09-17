use crate::{
    formatters::{default::DefaultFormatter, Formatter},
    Config,
};
use log::{LevelFilter, Log};
use std::sync::{Arc, Mutex};

pub struct TestLogger {
    pub buffer: Arc<Mutex<Vec<String>>>,
    pub config: Config,
}

impl TestLogger {
    #[cfg(test)]
    pub fn new(config: Config) -> Self {
        TestLogger {
            buffer: Arc::new(Mutex::new(Vec::new())),
            config,
        }
    }
}

impl Log for TestLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        if self.config.level_filter == LevelFilter::Off {
            return true;
        }

        metadata.level() <= self.config.level_filter
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let formatter = DefaultFormatter::new(record, &self.config);

        let mut buffer = self.buffer.lock().unwrap();
        buffer.push(formatter.format());
    }

    fn flush(&self) {}
}
