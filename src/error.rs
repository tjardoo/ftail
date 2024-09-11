use std::fmt::Display;

use log::SetLoggerError;

#[derive(Debug)]
pub enum FtailError {
    SetLoggerError(SetLoggerError),
    DuplicatedDriver(String),
}

impl std::error::Error for FtailError {}

impl Display for FtailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FtailError::SetLoggerError(e) => write!(f, "Error setting logger: {}", e),
            FtailError::DuplicatedDriver(driver) => write!(f, "Duplicated driver: {}", driver),
        }
    }
}
