use log::LevelFilter;

use crate::Config;

#[cfg(feature = "standard")]
pub mod default;
#[cfg(feature = "formatted")]
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
