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

/// A logger that logs messages to a daily log file.
pub struct DailyFileLogger {
    file: Mutex<LineWriter<File>>,
    dir: String,
    current_date: Mutex<String>,
    config: Config,
}

impl DailyFileLogger {
    pub fn new(dir: &str, config: Config) -> Result<Self, FtailError> {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let path = format!("{}/{}.log", dir, today);

        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .map_err(FtailError::IoError)?;

        let md = std::fs::metadata(dir).map_err(FtailError::IoError)?;

        if md.permissions().readonly() {
            return Err(FtailError::PermissionsError(dir.to_string()));
        }

        Ok(DailyFileLogger {
            file: Mutex::new(LineWriter::new(file)),
            dir: dir.to_string(),
            current_date: Mutex::new(today),
            config,
        })
    }

    fn rotate_file_if_needed(&self) {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let mut current_date = self.current_date.lock().unwrap();

        if *current_date != today {
            let path = format!("{}/{}.log", self.dir, today);

            let new_file = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .unwrap();

            let mut file = self.file.lock().unwrap();

            *file = LineWriter::new(new_file);
            *current_date = today;
        }
    }
}

impl Log for DailyFileLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        self.rotate_file_if_needed();

        let formatter = DefaultFormatter::new(record, &self.config);

        let mut file = self.file.lock().unwrap();
        writeln!(file, "{}", formatter.format()).unwrap();
        file.flush().unwrap();
    }

    fn flush(&self) {
        self.file.lock().unwrap().flush().unwrap();
    }
}
