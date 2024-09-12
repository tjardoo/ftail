use ftail::Ftail;
use log::LevelFilter;

// This example demonstrates how to log messages to stdout.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ftail::new().console(LevelFilter::Off).init()?;

    log::trace!("This is a trace message");

    log::debug!("This is a debug message");

    log::info!(target: "foo", "bar");

    log::warn!("This is a warning message");

    log::error!("This is an error message");

    Ok(())
}
