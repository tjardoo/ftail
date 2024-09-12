use log::Log;
use std::{
    fs::File,
    io::{LineWriter, Write},
    sync::Mutex,
};

use crate::{
    error::FtailError,
    formatters::{default::DefaultFormatter, Config, Formatter},
};

/// A logger that logs messages to a single log file.
pub struct SingleFileLogger {
    file: Mutex<LineWriter<File>>,
    config: Config,
}

impl SingleFileLogger {
    pub fn new(path: &str, append: bool, config: Config) -> Result<Self, FtailError> {
        let file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .append(append)
            .open(path)
            .map_err(FtailError::IoError)?;

        let md = std::fs::metadata(path).map_err(FtailError::IoError)?;

        if md.permissions().readonly() {
            return Err(FtailError::PermissionsError(path.to_string()));
        }

        Ok(SingleFileLogger {
            file: Mutex::new(LineWriter::new(file)),
            config,
        })
    }
}

impl Log for SingleFileLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let formatter = DefaultFormatter::new(record, &self.config);

        let mut file = self.file.lock().unwrap();
        writeln!(file, "{}", formatter.format()).unwrap();
        file.flush().unwrap();
    }

    fn flush(&self) {
        self.file.lock().unwrap().flush().unwrap();
    }
}
