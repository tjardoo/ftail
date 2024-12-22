use std::fmt::Display;

use log::SetLoggerError;

#[derive(Debug)]
pub enum FtailError {
    SetLoggerError(SetLoggerError),
    NoChannelsError,
    IoError(std::io::Error),
    PermissionsError(String),
}

impl std::error::Error for FtailError {}

impl Display for FtailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FtailError::SetLoggerError(e) => write!(f, "Error setting logger: {}", e),
            FtailError::NoChannelsError => write!(f, "No channels were added to the logger"),
            FtailError::IoError(e) => write!(f, "I/O error: {}", e),
            FtailError::PermissionsError(path) => {
                write!(f, "The path {} is read-only", path)
            }
        }
    }
}
