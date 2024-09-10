use ftail::{drivers::console::ConsoleLogger, Ftail};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    Ftail::new()
        .add_driver(ConsoleLogger::new(), log::LevelFilter::Debug)
        .init()?;

    log::trace!("This is a trace message");

    log::debug!("This is a debug message");

    log::info!(target: "foo", "bar");

    log::warn!("This is a warning");

    log::error!("This is an error");

    Ok(())
}
