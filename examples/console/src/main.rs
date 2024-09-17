use ftail::Ftail;
use log::{Level, LevelFilter};
use std::vec;

// This example demonstrates how to log messages to stdout.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ftail::new()
        // .timezone(ftail::Tz::UTC)
        .timezone(ftail::Tz::Europe__Amsterdam) // optional
        .datetime_format("%d-%m-%Y %H:%M:%S%.3f") // optional
        .console(LevelFilter::Off)
        .filter_levels(vec![
            Level::Trace,
            Level::Debug,
            Level::Info,
            // Level::Warn,
            Level::Error,
        ]) // optional
        .filter_targets(vec!["foo".to_string()]) // optional
        .init()?;

    log::trace!("This is a trace message");

    log::debug!("This is a debug message");

    log::info!(target: "foo", "bar");

    log::warn!("This is a warning message");

    log::error!("This is an error message");

    Ok(())
}
