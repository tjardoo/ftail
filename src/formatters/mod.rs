pub mod default;
pub mod readable;

pub trait Formatter {
    fn format(&self) -> String;
}

#[derive(Clone)]
pub struct Config {
    pub datetime_format: String,
    pub timezone: chrono_tz::Tz,
}

impl Config {
    pub fn new() -> Config {
        Config {
            datetime_format: "%Y-%m-%d %H:%M:%S".to_string(),
            timezone: chrono_tz::Tz::UTC,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
