use std::path::Path;

use ftail::Ftail;
use log::LevelFilter;

// This example demonstrates how to log messages to different files based on their log level.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all("examples/stack/logs")?;

    Ftail::new()
        .console(LevelFilter::Info)
        .single_file(
            Path::new("examples/stack/logs/trace.log"),
            true,
            LevelFilter::Trace,
        )
        .single_file(
            Path::new("examples/stack/logs/error.log"),
            true,
            LevelFilter::Error,
        )
        .daily_file_env_level(Path::new("examples/stack/logs"))
        .init()?;

    log::trace!("This is a trace message");

    log::debug!("This is a debug message");

    log::info!(target: "foo", "bar");

    log::warn!("This is a warning message");

    // the `error.log` will only contain this message
    log::error!("This is an error message");

    Ok(())
}
