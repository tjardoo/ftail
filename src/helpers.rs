#[cfg(feature = "file_channels")]
use crate::Config;
#[cfg(feature = "file_channels")]
use std::{fs::File, io::LineWriter, path::Path};

#[cfg(feature = "file_channels")]
pub(crate) fn rotate_if_exceeds_max_file_size(
    file: &mut LineWriter<File>,
    file_path: &Path,
    config: &Config,
) {
    let max_file_size = match config.max_file_size {
        Some(max_file_size) => max_file_size,
        None => return,
    };

    match file.get_ref().metadata() {
        Ok(md) if md.len() > max_file_size => {}
        _ => return,
    }

    let mut new_path = file_path.as_os_str().to_owned();
    new_path.push(".old");

    let mut counter = 1;
    while std::fs::metadata(&new_path).is_ok() {
        new_path = file_path.as_os_str().to_owned();
        new_path.push(format!(".old{}", counter));
        counter += 1;
    }

    match std::fs::rename(file_path, &new_path) {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(_) => return,
    }

    let new_file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path);

    if let Ok(new_file) = new_file {
        *file = LineWriter::new(new_file);
    }
}

#[cfg(any(
    feature = "console",
    feature = "formatted_console",
    feature = "file_channels"
))]
pub(crate) fn get_env_log_level() -> log::LevelFilter {
    std::env::var("RUST_LOG")
        .ok()
        .and_then(|value| parse_env_log_level(&value))
        .unwrap_or(log::LevelFilter::Info)
}

#[cfg(any(
    feature = "console",
    feature = "formatted_console",
    feature = "file_channels"
))]
fn parse_env_log_level(value: &str) -> Option<log::LevelFilter> {
    // Skip env_logger's optional `/filter` suffix.
    let directives = value.split('/').next().unwrap_or("");

    directives
        .split(',')
        .map(str::trim)
        .filter(|directive| !directive.contains('='))
        .find_map(|directive| directive.parse().ok())
}

#[cfg(all(test, feature = "console"))]
mod tests {
    use super::parse_env_log_level;
    use log::LevelFilter;

    #[test]
    fn it_parses_a_plain_level() {
        assert_eq!(parse_env_log_level("off"), Some(LevelFilter::Off));
        assert_eq!(parse_env_log_level("error"), Some(LevelFilter::Error));
        assert_eq!(parse_env_log_level("warn"), Some(LevelFilter::Warn));
        assert_eq!(parse_env_log_level("info"), Some(LevelFilter::Info));
        assert_eq!(parse_env_log_level("debug"), Some(LevelFilter::Debug));
        assert_eq!(parse_env_log_level("trace"), Some(LevelFilter::Trace));
        assert_eq!(parse_env_log_level("DEBUG"), Some(LevelFilter::Debug));
        assert_eq!(parse_env_log_level(" warn "), Some(LevelFilter::Warn));
    }

    #[test]
    fn it_uses_the_bare_level_from_env_logger_style_values() {
        assert_eq!(
            parse_env_log_level("info,hyper=warn"),
            Some(LevelFilter::Info)
        );
        assert_eq!(
            parse_env_log_level("hyper=warn,error"),
            Some(LevelFilter::Error)
        );
        assert_eq!(parse_env_log_level("debug/foo"), Some(LevelFilter::Debug));
    }

    #[test]
    fn it_returns_none_when_there_is_no_level() {
        assert_eq!(parse_env_log_level(""), None);
        assert_eq!(parse_env_log_level("my_crate=debug"), None);
        assert_eq!(parse_env_log_level("my_crate"), None);
        assert_eq!(parse_env_log_level("verbose"), None);
    }
}
