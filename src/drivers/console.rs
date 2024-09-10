pub struct ConsoleLogger {}

impl ConsoleLogger {
    pub fn new() -> Box<ConsoleLogger> {
        Box::new(ConsoleLogger {})
    }
}

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Debug
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            println!(
                "{} [{}] {} {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}
