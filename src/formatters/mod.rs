use log::LevelFilter;

use crate::Config;

pub mod default;
pub mod readable;

pub trait Formatter {
    fn format(&self) -> String;
}

impl Config {
    pub fn new() -> Config {
        Config {
            level_filter: LevelFilter::Off,
            datetime_format: "%Y-%m-%d %H:%M:%S".to_string(),
            #[cfg(feature = "timezone")]
            timezone: chrono_tz::Tz::UTC,
            max_file_size: None,
            retention_days: None,
            levels: None,
            targets: None,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
