use ftail::Ftail;
use log::{LevelFilter, Log};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ftail::new()
        .custom(Box::new(Box::new(CustomLogger {})), LevelFilter::Debug)
        .init()?;

    log::trace!("This is a trace message");

    log::debug!("This is a debug message");

    log::info!(target: "foo", "bar");

    log::warn!("This is a warning");

    log::error!("This is an error");

    Ok(())
}

struct CustomLogger {}

impl Log for CustomLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let time = chrono::Local::now().format("%H:%M:%S").to_string();

        println!("{} {} {}", time, record.level(), record.args());
    }

    fn flush(&self) {}
}
