use log::{LevelFilter, Log};
use std::{
    fs::File,
    io::{LineWriter, Write},
    path::{Path, PathBuf},
    sync::Mutex,
};

use crate::{
    error::FtailError,
    formatters::{default::DefaultFormatter, Formatter},
    helpers::rotate_if_exceeds_max_file_size,
    Config,
};

/// A logger that logs messages to a daily log file.
pub struct DailyFileLogger {
    file: Mutex<LineWriter<File>>,
    file_path: PathBuf,
    dir: PathBuf,
    current_date: Mutex<String>,
    config: Config,
}

impl DailyFileLogger {
    pub fn new(dir: &Path, config: Config) -> Result<Self, FtailError> {
        let today_filename = chrono::Local::now().format("%Y-%m-%d.log").to_string();
        let path = dir.join(&today_filename);

        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .map_err(FtailError::IoError)?;

        let md = std::fs::metadata(dir).map_err(FtailError::IoError)?;

        if md.permissions().readonly() {
            return Err(FtailError::PermissionsError(dir.to_owned()));
        }

        Ok(DailyFileLogger {
            file: Mutex::new(LineWriter::new(file)),
            file_path: path,
            dir: dir.to_owned(),
            current_date: Mutex::new(today_filename),
            config,
        })
    }

    fn rotate_daily_file(&self) {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let mut current_date = self.current_date.lock().unwrap();

        if *current_date != today {
            let path = self.dir.join(format!("{today}.log"));

            let new_file = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .unwrap();

            let mut file = self.file.lock().unwrap();

            *file = LineWriter::new(new_file);
            *current_date = today;
        }

        if let Some(retention_days) = self.config.retention_days {
            remove_old_log_files(&self.dir, retention_days);
        }
    }
}

impl Log for DailyFileLogger {
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
        self.rotate_daily_file();

        let formatter = DefaultFormatter::new(record, &self.config);

        let mut file = self.file.lock().unwrap();
        writeln!(file, "{}", formatter.format()).unwrap();
        file.flush().unwrap();
    }

    fn flush(&self) {
        self.file.lock().unwrap().flush().unwrap();
    }
}

fn remove_old_log_files(dir: &Path, retention_days: u64) {
    let files = std::fs::read_dir(dir).unwrap();

    for file in files {
        let file = file.unwrap();
        let path = file.path();

        if path.extension().and_then(|ext| ext.to_str()) == Some("log") {
            let metadata = file.metadata().unwrap();
            let modified_system_time = metadata.modified().unwrap();
            let modified = chrono::DateTime::<chrono::Local>::from(modified_system_time);
            let now = chrono::Local::now();
            let duration = now.signed_duration_since(modified);

            if duration.num_days() > retention_days as i64 {
                std::fs::remove_file(path).unwrap();
            }
        }
    }
}
