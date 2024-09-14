use ftail::Ftail;
use log::LevelFilter;

// This example demonstrates how to log messages to a single file in the logs directory.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ftail::new()
        .single_file("logs/demo.log", true, LevelFilter::Trace)
        .max_file_size(10)
        .init()?;

    log::trace!("This is a trace message");

    log::debug!("This is a debug message");

    log::info!(target: "foo", "bar");

    log::warn!("This is a warning message");

    log::error!("This is an error message");

    Ok(())
}
