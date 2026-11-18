use log::Log;
use std::{
    fs::File,
    io::{LineWriter, Write},
    path::{Path, PathBuf},
    sync::{Mutex, PoisonError},
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
    dir: PathBuf,
    current_date: Mutex<String>,
    config: Config,
}

impl DailyFileLogger {
    pub fn new(dir: &Path, config: Config) -> Result<Self, FtailError> {
        let today = today();

        let file = open_log_file(&log_file_path(dir, &today)).map_err(FtailError::IoError)?;

        let md = std::fs::metadata(dir).map_err(FtailError::IoError)?;

        if md.permissions().readonly() {
            return Err(FtailError::PermissionsError(dir.to_owned()));
        }

        Ok(DailyFileLogger {
            file: Mutex::new(LineWriter::new(file)),
            dir: dir.to_owned(),
            current_date: Mutex::new(today),
            config,
        })
    }

    fn rotate_daily_file(&self, current_date: &mut String, file: &mut LineWriter<File>) {
        let today = today();

        if *current_date != today {
            if let Ok(new_file) = open_log_file(&log_file_path(&self.dir, &today)) {
                *file = LineWriter::new(new_file);
                *current_date = today;
            }
        }

        if let Some(retention_days) = self.config.retention_days {
            remove_old_log_files(&self.dir, retention_days);
        }
    }
}

impl Log for DailyFileLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.config.level_filter
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let message = DefaultFormatter::new(record, &self.config).format();

        let mut current_date = self
            .current_date
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        let mut file = self.file.lock().unwrap_or_else(PoisonError::into_inner);

        self.rotate_daily_file(&mut current_date, &mut file);
        rotate_if_exceeds_max_file_size(
            &mut file,
            &log_file_path(&self.dir, &current_date),
            &self.config,
        );

        let _ = writeln!(file, "{}", message);
        let _ = file.flush();
    }

    fn flush(&self) {
        let _ = self
            .file
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .flush();
    }
}

fn today() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

fn log_file_path(dir: &Path, date: &str) -> PathBuf {
    dir.join(format!("{}.log", date))
}

fn open_log_file(path: &Path) -> std::io::Result<File> {
    std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
}

fn remove_old_log_files(dir: &Path, retention_days: u64) {
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    let now = chrono::Local::now();

    for entry in entries.flatten() {
        let path = entry.path();

        if path.extension().and_then(|ext| ext.to_str()) != Some("log") {
            continue;
        }

        let modified_system_time = match entry.metadata().and_then(|md| md.modified()) {
            Ok(modified) => modified,
            Err(_) => continue,
        };
        let modified = chrono::DateTime::<chrono::Local>::from(modified_system_time);
        let duration = now.signed_duration_since(modified);

        if duration.num_days() > retention_days as i64 {
            let _ = std::fs::remove_file(path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::{Level, LevelFilter, Record};

    fn test_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("ftail-{}-{}", name, std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn log_line(logger: &DailyFileLogger, message: &str) {
        logger.log(
            &Record::builder()
                .level(Level::Info)
                .target("test")
                .args(format_args!("{}", message))
                .build(),
        );
    }

    #[test]
    fn it_writes_to_todays_file() {
        let dir = test_dir("daily-today");
        let config = Config {
            level_filter: LevelFilter::Trace,
            ..Config::default()
        };

        let logger = DailyFileLogger::new(&dir, config).unwrap();
        log_line(&logger, "hello");

        let content = std::fs::read_to_string(log_file_path(&dir, &today())).unwrap();
        assert!(content.trim_end().ends_with("INFO test hello"));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn it_rotates_todays_file_after_the_date_changed() {
        let dir = test_dir("daily-midnight");
        let config = Config {
            level_filter: LevelFilter::Trace,
            max_file_size: Some(10),
            ..Config::default()
        };

        let yesterday = "2000-01-01";
        let yesterday_path = log_file_path(&dir, yesterday);
        let logger = DailyFileLogger {
            file: Mutex::new(LineWriter::new(open_log_file(&yesterday_path).unwrap())),
            dir: dir.clone(),
            current_date: Mutex::new(yesterday.to_string()),
            config,
        };

        log_line(&logger, "first");
        log_line(&logger, "second");
        log_line(&logger, "third");

        let today_path = log_file_path(&dir, &today());
        let read = |path: &Path| std::fs::read_to_string(path).unwrap_or_default();

        assert_eq!(read(&yesterday_path), "");
        assert!(!dir.join(format!("{}.log.old", yesterday)).exists());
        assert!(read(&today_path.with_extension("log.old")).contains("first"));
        assert!(read(&today_path.with_extension("log.old1")).contains("second"));
        assert!(read(&today_path).contains("third"));

        let _ = std::fs::remove_dir_all(&dir);
    }
}
