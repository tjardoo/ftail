use log::Log;
use std::{
    fs::File,
    io::{LineWriter, Write},
    sync::Mutex,
};

use crate::formatters::{default::DefaultFormatter, Formatter};

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

        let md = std::fs::metadata(path).unwrap();

        if md.permissions().readonly() {
            panic!("The logs directory `{path}` is readonly");
        }

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
        let formatter = DefaultFormatter::new(record);

        let mut file = self.file.lock().unwrap();
        writeln!(file, "{}", formatter.format()).unwrap();
        file.flush().unwrap();
    }

    fn flush(&self) {
        self.file.lock().unwrap().flush().unwrap();
    }
}
