use ftail::Ftail;
use log::LevelFilter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ftail::new().stdout(LevelFilter::Trace).init()?;

    log::trace!("This is a trace message");

    log::debug!("This is a debug message");

    log::info!(target: "foo", "bar");

    log::warn!("This is a warning");

    log::error!("This is an error");

    Ok(())
}
