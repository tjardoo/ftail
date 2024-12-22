use log::{LevelFilter, Log};
use std::{
    fs::File,
    io::{LineWriter, Write},
    path::PathBuf,
    sync::Mutex,
};

use crate::{
    error::FtailError,
    formatters::{default::DefaultFormatter, Formatter},
    helpers::rotate_if_exceeds_max_file_size,
    Config,
};

/// A logger that logs messages to a single log file.
pub struct SingleFileLogger {
    file: Mutex<LineWriter<File>>,
    file_path: PathBuf,
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
            file_path: PathBuf::from(path),
            config,
        })
    }
}

impl Log for SingleFileLogger {
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

        rotate_if_exceeds_max_file_size(&self.file, self.file_path.clone(), &self.config);

        let formatter = DefaultFormatter::new(record, &self.config);

        let mut file = self.file.lock().unwrap();
        writeln!(file, "{}", formatter.format()).unwrap();
        file.flush().unwrap();
    }

    fn flush(&self) {
        self.file.lock().unwrap().flush().unwrap();
    }
}
