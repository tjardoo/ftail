use log::Log;

use crate::ansi_escape::TextStyling;

pub struct ConsoleLogger {}

impl ConsoleLogger {
    pub fn new() -> Box<ConsoleLogger> {
        Box::new(ConsoleLogger {})
    }
}

impl Log for ConsoleLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let level = match record.level() {
            log::Level::Trace => record.level().bold().black(),
            log::Level::Debug => record.level().bold().blue(),
            log::Level::Info => record.level().bold().green(),
            log::Level::Warn => record.level().bold().yellow(),
            log::Level::Error => record.level().bold().red(),
        };

        println!("{} · {}", now.black(), level);
        println!("{}", record.args().bold());
        println!(
            "{}{}{}",
            record.file().unwrap().black(),
            ":".black(),
            record.line().unwrap().black()
        );
        println!();
    }

    fn flush(&self) {}
}
