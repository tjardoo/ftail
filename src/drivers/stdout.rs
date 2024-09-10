use log::Log;

pub struct StdOutLogger {}

impl StdOutLogger {
    pub fn new() -> Box<StdOutLogger> {
        Box::new(StdOutLogger {})
    }
}

impl Log for StdOutLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        println!(
            "{} {} {} {}",
            now,
            record.level(),
            record.target(),
            record.args()
        );
    }

    fn flush(&self) {}
}
