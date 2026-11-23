use log::Log;
use std::{
    fs::File,
    io::{LineWriter, Write},
    path::{Path, PathBuf},
    sync::{Arc, Mutex, PoisonError},
};

use crate::{
    error::FtailError,
    formatters::{default::DefaultFormatter, Formatter},
    helpers::rotate_if_exceeds_max_file_size,
    Config,
};

/// A logger that logs messages to a single log file.
pub struct SingleFileLogger {
    file: Arc<Mutex<LogFile>>,
    file_path: PathBuf,
    config: Config,
}

struct LogFile {
    writer: LineWriter<File>,
    truncate_pending: bool,
}

impl LogFile {
    fn truncate_if_pending(&mut self) {
        if std::mem::take(&mut self.truncate_pending) {
            let _ = self.writer.get_ref().set_len(0);
        }
    }
}

impl SingleFileLogger {
    pub fn new(path: &Path, append: bool, config: Config) -> Result<Self, FtailError> {
        let file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .append(append)
            .truncate(false)
            .open(path)
            .map_err(FtailError::IoError)?;

        let md = std::fs::metadata(path).map_err(FtailError::IoError)?;

        if md.permissions().readonly() {
            return Err(FtailError::PermissionsError(path.to_owned()));
        }

        Ok(SingleFileLogger {
            file: Arc::new(Mutex::new(LogFile {
                writer: LineWriter::new(file),
                truncate_pending: !append,
            })),
            file_path: PathBuf::from(path),
            config,
        })
    }

    /// Returns a callback that empties the file when `append` is false, unless a message was
    /// already written. `Ftail::init` runs it after the logger is installed.
    pub(crate) fn truncate_callback(&self) -> impl FnOnce() + Send + Sync + 'static {
        let file = Arc::clone(&self.file);

        move || {
            file.lock()
                .unwrap_or_else(PoisonError::into_inner)
                .truncate_if_pending()
        }
    }
}

impl Log for SingleFileLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.config.level_filter
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let message = DefaultFormatter::new(record, &self.config).format();

        let mut file = self.file.lock().unwrap_or_else(PoisonError::into_inner);

        file.truncate_if_pending();
        rotate_if_exceeds_max_file_size(&mut file.writer, &self.file_path, &self.config);

        let _ = writeln!(file.writer, "{}", message);
        let _ = file.writer.flush();
    }

    fn flush(&self) {
        let _ = self
            .file
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .writer
            .flush();
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

    fn log_line(logger: &SingleFileLogger, message: &str) {
        logger.log(
            &Record::builder()
                .level(Level::Info)
                .target("test")
                .args(format_args!("{}", message))
                .build(),
        );
    }

    fn config() -> Config {
        Config {
            level_filter: LevelFilter::Trace,
            ..Config::default()
        }
    }

    #[test]
    fn it_overwrites_the_file_when_append_is_false() {
        let dir = test_dir("single-overwrite");
        let path = dir.join("app.log");
        std::fs::write(&path, "a long line from a previous run\nanother old line\n").unwrap();

        let logger = SingleFileLogger::new(&path, false, config()).unwrap();
        log_line(&logger, "new");

        let content = std::fs::read_to_string(&path).unwrap();
        assert_eq!(content.lines().count(), 1);
        assert!(content.trim_end().ends_with("INFO test new"));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn it_does_not_log_anything_when_the_level_is_off() {
        let dir = test_dir("single-off");
        let path = dir.join("app.log");

        let config = Config {
            level_filter: LevelFilter::Off,
            ..config()
        };
        let logger = SingleFileLogger::new(&path, true, config).unwrap();
        log_line(&logger, "hidden");

        assert_eq!(std::fs::read_to_string(&path).unwrap(), "");

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn it_keeps_the_existing_content_when_append_is_true() {
        let dir = test_dir("single-append");
        let path = dir.join("app.log");
        std::fs::write(&path, "old line\n").unwrap();

        let logger = SingleFileLogger::new(&path, true, config()).unwrap();
        log_line(&logger, "new");

        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.starts_with("old line\n"));
        assert!(content.trim_end().ends_with("INFO test new"));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn it_rotates_the_file_when_it_exceeds_the_max_file_size() {
        let dir = test_dir("single-rotate");
        let path = dir.join("app.log");

        let config = Config {
            max_file_size: Some(10),
            ..config()
        };
        let logger = SingleFileLogger::new(&path, true, config).unwrap();
        log_line(&logger, "first");
        log_line(&logger, "second");
        log_line(&logger, "third");

        let read = |name: &str| std::fs::read_to_string(dir.join(name)).unwrap();
        assert!(read("app.log.old").contains("first"));
        assert!(read("app.log.old1").contains("second"));
        assert!(read("app.log").contains("third"));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn it_keeps_the_old_log_when_init_fails() {
        let dir = test_dir("single-init-fails");
        let path = dir.join("app.log");
        std::fs::write(&path, "old line\n").unwrap();

        let result = crate::Ftail::new()
            .single_file(&path, false, LevelFilter::Trace)
            .single_file(
                &dir.join("missing").join("app.log"),
                true,
                LevelFilter::Trace,
            )
            .build();

        assert!(result.is_err());
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "old line\n");

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn it_empties_the_file_once_the_logger_is_installed() {
        let dir = test_dir("single-installed");
        let path = dir.join("app.log");
        std::fs::write(&path, "old line\n").unwrap();

        let mut ftail = crate::Ftail::new()
            .single_file(&path, false, LevelFilter::Trace)
            .build()
            .unwrap();

        assert_eq!(std::fs::read_to_string(&path).unwrap(), "old line\n");

        for callback in ftail.take_on_installed() {
            callback();
        }

        assert_eq!(std::fs::read_to_string(&path).unwrap(), "");

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn it_recreates_the_file_when_a_rotation_could_not_create_it() {
        let dir = test_dir("single-recreate");
        let path = dir.join("app.log");

        let config = Config {
            max_file_size: Some(10),
            ..config()
        };
        let logger = SingleFileLogger::new(&path, true, config).unwrap();
        log_line(&logger, "first");

        std::fs::rename(&path, dir.join("app.log.old")).unwrap();

        log_line(&logger, "second");
        log_line(&logger, "third");

        let read = |name: &str| std::fs::read_to_string(dir.join(name)).unwrap();
        assert!(read("app.log.old").contains("first"));
        assert!(read("app.log.old1").contains("second"));
        assert!(read("app.log").contains("third"));

        let _ = std::fs::remove_dir_all(&dir);
    }
}
