use crate::{
    formatters::{default::DefaultFormatter, Formatter},
    Config,
};
use log::Log;
use std::{io::Write, sync::Mutex};

struct TestLogger {
    buffer: Mutex<Vec<u8>>,
    config: Config,
}

impl TestLogger {
    #[cfg(test)]
    pub fn new(config: Config) -> Self {
        TestLogger {
            buffer: Mutex::new(vec![]),
            config,
        }
    }
}

impl Log for TestLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let mut buffer = self.buffer.lock().unwrap();

        let formatter = DefaultFormatter::new(record, &self.config);

        buffer.write_all(formatter.format().as_bytes()).unwrap();
    }

    fn flush(&self) {}
}

fn remove_datetime_from_message(message: String, config: Config) -> String {
    #[cfg(feature = "timezone")]
    let datetime = chrono::Local::now()
        .with_timezone(&config.timezone)
        .format(&config.datetime_format)
        .to_string();

    #[cfg(not(feature = "timezone"))]
    let datetime = chrono::Local::now()
        .format(&config.datetime_format)
        .to_string();

    let len = datetime.len() + 1; // +1 for the space after the datetime

    message[len..].to_string()
}

#[cfg(test)]
mod tests {
    use log::{Level, Log, Metadata, Record};

    use crate::{
        drivers::test::{remove_datetime_from_message, TestLogger},
        Config,
    };

    fn get_message_from_logger(config: Config, record: Record, is_remove_datetime: bool) -> String {
        let logger = TestLogger::new(config.clone());

        logger.log(&record);

        let buffer = logger.buffer.lock().unwrap();
        let message = std::str::from_utf8(&buffer).unwrap().to_string();

        if !is_remove_datetime {
            return message;
        }

        remove_datetime_from_message(message, config)
    }

    #[test]
    fn test_log_message() {
        let config = Config::default();

        let record = Record::builder()
            .metadata(
                Metadata::builder()
                    .level(Level::Info)
                    .target("test")
                    .build(),
            )
            .args(format_args!("Hello, world!"))
            .build();

        assert_eq!(
            get_message_from_logger(config, record, true),
            "INFO test Hello, world!"
        );
    }

    #[test]
    fn test_log_message_with_custom_datetime_format() {
        let config = Config {
            datetime_format: "%Y-%m-%d %H:%M:%S%.3f".to_string(),
            ..Default::default()
        };

        let record = Record::builder()
            .metadata(
                Metadata::builder()
                    .level(Level::Error)
                    .target("foo")
                    .build(),
            )
            .args(format_args!("bar"))
            .build();

        assert_eq!(
            get_message_from_logger(config, record, true),
            "ERROR foo bar"
        );
    }

    #[test]
    #[cfg(feature = "timezone")]
    fn test_log_message_with_specified_timezone() {
        let config = Config {
            datetime_format: "%Z".to_string(),
            #[cfg(feature = "timezone")]
            timezone: ftail::Tz::Australia__Sydney,
            ..Default::default()
        };

        let record = Record::builder()
            .metadata(
                Metadata::builder()
                    .level(Level::Debug)
                    .target("test")
                    .build(),
            )
            .args(format_args!("Hello, world!"))
            .build();

        assert_eq!(
            get_message_from_logger(config, record, false),
            "AEST DEBUG test Hello, world!"
        );
    }
}
