use ftail::{ansi_escape::TextStyling, Config, Ftail};
use log::{LevelFilter, Log};

// This example demonstrates how to log messages to stdout with custom styling.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ftail::new()
        .custom(
            |config: ftail::Config| Box::new(CustomLogger { config }) as Box<dyn Log + Send + Sync>,
            LevelFilter::Debug,
        )
        .datetime_format("%H:%M:%S%.3f")
        .init()?;

    log::trace!("This is a trace message");

    log::debug!("This is a debug message");

    log::info!(target: "foo", "bar");

    log::warn!("This is a warning message");

    log::error!("This is an error message");

    Ok(())
}

struct CustomLogger {
    config: Config,
}

impl Log for CustomLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let time = chrono::Local::now()
            .format(&self.config.datetime_format)
            .to_string();

        let level = match record.level() {
            log::Level::Trace => record.level().black().to_string(),
            log::Level::Debug => record.level().blue().to_string(),
            log::Level::Info => record.level().green().to_string(),
            log::Level::Warn => record.level().yellow().to_string(),
            log::Level::Error => record.level().red().to_string(),
        };

        println!("{} [{}] {}", time.black(), level.bold(), record.args());
    }

    fn flush(&self) {}
}
