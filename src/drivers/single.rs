use log::Log;
use std::{
    fs::File,
    io::{LineWriter, Write},
    sync::Mutex,
};

pub struct SingleLogger {
    file: Mutex<LineWriter<File>>,
}

impl SingleLogger {
    pub fn new(path: &str, append: bool) -> Self {
        let file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .append(append)
            .open(path)
            .unwrap();

        // @todo
        // let md = std::fs::metadata("logs").unwrap();
        // let permissions = md.permissions();

        // if permissions.readonly() {
        //     return Err(P2000Error::ConfigurationError(format!(
        //         "directory /logs is readonly"
        //     )));
        // }

        SingleLogger {
            file: Mutex::new(LineWriter::new(file)),
        }
    }
}

impl Log for SingleLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let line = format!(
            "{} {} {} {}",
            now,
            record.level(),
            record.target(),
            record.args()
        );

        let mut file = self.file.lock().unwrap();
        writeln!(file, "{}", line).unwrap();
        file.flush().unwrap();
    }

    fn flush(&self) {
        self.file.lock().unwrap().flush().unwrap();
    }
}
