use log::Log;

pub struct StdOutLogger {}

impl Log for StdOutLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
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
