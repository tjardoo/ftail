use std::{fs::File, io::LineWriter, path::PathBuf, sync::Mutex};

use crate::Config;

pub(crate) fn rotate_if_exceeds_max_file_size(
    file: &Mutex<LineWriter<File>>,
    file_path: PathBuf,
    config: &Config,
) {
    if config.max_file_size.is_none() {
        return;
    }

    let mut file = file.lock().unwrap();

    let md = file.get_ref().metadata().unwrap();

    if md.len() > config.max_file_size.unwrap() {
        let path = file_path.to_str().unwrap();

        let mut new_path = format!("{}.old", path);

        let mut counter = 1;
        while std::fs::metadata(&new_path).is_ok() {
            new_path = format!("{}.old{}", path, counter);
            counter += 1;
        }

        std::fs::rename(path, &new_path).unwrap();

        let new_file = std::fs::File::create(path).unwrap();
        *file = LineWriter::new(new_file);
    }
}

pub(crate) fn get_env_log_level() -> log::LevelFilter {
    let level = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());

    match level.to_lowercase().as_str() {
        "trace" => log::LevelFilter::Trace,
        "debug" => log::LevelFilter::Debug,
        "info" => log::LevelFilter::Info,
        "warn" => log::LevelFilter::Warn,
        "error" => log::LevelFilter::Error,
        "off" => log::LevelFilter::Off,
        _ => panic!("invalid RUST_LOG env var"),
    }
}
