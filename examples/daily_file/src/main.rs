use std::path::Path;

use ftail::Ftail;
use log::LevelFilter;

// This example demonstrates how to log messages to a daily log file in the logs directory.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all("examples/daily_file/logs")?;

    Ftail::new()
        .retention_days(14)
        .daily_file(Path::new("examples/daily_file/logs"), LevelFilter::Trace)
        .init()?;

    log::trace!("This is a trace message");

    log::debug!("This is a debug message");

    log::info!(target: "foo", "bar");

    log::warn!("This is a warning message");

    log::error!("This is an error message");

    Ok(())
}
