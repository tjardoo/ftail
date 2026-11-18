use crate::Config;

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

    if message.is_empty() {
        return message;
    }

    let len = datetime.len() + 1; // +1 for the space after the datetime

    message[len..].to_string()
}

#[allow(clippy::module_inception)]
#[cfg(test)]
mod tests {
    use crate::{
        channels::test::TestLogger, error::FtailError, tests::remove_datetime_from_message, Config,
        Ftail,
    };
    use log::{Level, LevelFilter, Log, Metadata, Record};
    use std::sync::{Arc, Mutex};

    fn test_logger(
        config: Config,
        buffer: Arc<Mutex<Vec<String>>>,
    ) -> impl Fn(Config) -> Box<dyn Log + Send + Sync> {
        move |_config| {
            Box::new(TestLogger {
                config: config.clone(),
                buffer: Arc::clone(&buffer),
            }) as Box<dyn Log + Send + Sync>
        }
    }

    fn log_record(logger: &Ftail, metadata: Metadata, args: std::fmt::Arguments) {
        let record = Record::builder().metadata(metadata).args(args).build();

        logger.log(&record);
    }

    fn get_message_from_logger(
        config: &Config,
        metadata: Metadata,
        args: std::fmt::Arguments,
    ) -> String {
        let buffer = Arc::new(Mutex::new(Vec::new()));

        let mut ftail = Ftail::new()
            .custom(
                test_logger(config.clone(), Arc::clone(&buffer)),
                config.level_filter,
            )
            .datetime_format(&config.datetime_format);

        #[cfg(feature = "timezone")]
        {
            ftail = ftail.timezone(config.timezone);
        }

        if let Some(levels) = config.levels.clone() {
            ftail = ftail.filter_levels(levels);
        }

        if let Some(targets) = &config.targets {
            let targets = targets
                .iter()
                .map(|target| target.as_str())
                .collect::<Vec<&str>>();

            ftail = ftail.filter_targets(targets);
        }

        let logger = ftail.build().unwrap();

        log_record(&logger, metadata, args);

        let logs = buffer.lock().unwrap();

        logs.iter()
            .map(|log| log.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }

    #[test]
    fn it_can_log_message() {
        let config = Config::default();

        let metadata = Metadata::builder()
            .level(Level::Debug)
            .target("test")
            .build();

        let args = format_args!("Hello, world!");

        let message = get_message_from_logger(&config, metadata, args);
        let message_without_datetime = remove_datetime_from_message(message, config);

        assert_eq!(message_without_datetime, "DEBUG test Hello, world!");
    }

    #[test]
    fn it_can_log_message_with_custom_datetime_format() {
        let config = Config {
            datetime_format: "%Y-%m-%d %H:%M:%S%.3f".to_string(),
            ..Default::default()
        };

        let metadata = Metadata::builder()
            .level(Level::Debug)
            .target("test")
            .build();

        let args = format_args!("Hello, world!");

        let message = get_message_from_logger(&config, metadata, args);
        let message_without_datetime = remove_datetime_from_message(message, config);

        assert_eq!(message_without_datetime, "DEBUG test Hello, world!");
    }

    #[test]
    #[cfg(feature = "timezone")]
    fn it_can_log_message_with_custom_timezone() {
        // No daylight saving time, so the abbreviation doesn't change.
        let config = Config {
            datetime_format: "%Z".to_string(),
            timezone: crate::Tz::Asia__Tokyo,
            ..Default::default()
        };

        let metadata = Metadata::builder()
            .level(Level::Debug)
            .target("test")
            .build();

        let args = format_args!("Hello, world!");

        let message = get_message_from_logger(&config, metadata, args);

        assert_eq!(message, "JST DEBUG test Hello, world!");
    }

    #[test]
    fn it_does_not_log_message_with_lower_level_than_level_filter() {
        let config = Config {
            level_filter: log::LevelFilter::Error,
            ..Default::default()
        };

        let metadata = Metadata::builder()
            .level(Level::Debug)
            .target("test")
            .build();

        let args = format_args!("Hello, world!");

        let message = get_message_from_logger(&config, metadata, args);

        assert_eq!(message, "");
    }

    #[test]
    fn it_does_log_message_with_level_specified_in_levels() {
        let config = Config {
            levels: Some(vec![Level::Error]),
            ..Default::default()
        };

        let metadata = Metadata::builder()
            .level(Level::Error)
            .target("foo")
            .build();

        let args = format_args!("bar");

        let message = get_message_from_logger(&config, metadata, args);
        let message_without_datetime = remove_datetime_from_message(message, config);

        assert_eq!(message_without_datetime, "ERROR foo bar");
    }

    #[test]
    fn it_does_not_log_message_with_level_not_specified_in_levels() {
        let config = Config {
            levels: Some(vec![Level::Error]),
            ..Default::default()
        };

        let metadata = Metadata::builder().level(Level::Warn).target("foo").build();

        let args = format_args!("bar");

        let message = get_message_from_logger(&config, metadata, args);

        assert_eq!(message, "");
    }

    #[test]
    fn it_does_log_message_with_target_specified_in_targets() {
        let config = Config {
            targets: Some(vec!["foo".to_string()]),
            ..Default::default()
        };

        let metadata = Metadata::builder()
            .level(Level::Error)
            .target("foo")
            .build();

        let args = format_args!("bar");

        let message = get_message_from_logger(&config, metadata, args);
        let message_without_datetime = remove_datetime_from_message(message, config);

        assert_eq!(message_without_datetime, "ERROR foo bar");
    }

    #[test]
    fn it_does_not_log_message_with_target_not_specified_in_targets() {
        let config = Config {
            targets: Some(vec!["bar".to_string()]),
            ..Default::default()
        };

        let metadata = Metadata::builder().level(Level::Warn).target("foo").build();

        let args = format_args!("bar");

        let message = get_message_from_logger(&config, metadata, args);

        assert_eq!(message, "");
    }

    #[test]
    fn it_returns_an_error_when_no_channels_are_added() {
        let result = Ftail::new().build();

        assert!(matches!(result, Err(FtailError::NoChannelsError)));
    }

    #[test]
    #[cfg(feature = "daily_file")]
    fn it_returns_an_error_when_the_log_directory_does_not_exist() {
        let dir = std::env::temp_dir().join("ftail-test-directory-that-does-not-exist");

        let result = Ftail::new().daily_file(&dir, LevelFilter::Info).build();

        assert!(matches!(result, Err(FtailError::IoError(_))));
    }

    #[test]
    #[cfg(feature = "single_file")]
    fn it_returns_an_error_when_the_log_file_can_not_be_opened() {
        let path = std::env::temp_dir()
            .join("ftail-test-directory-that-does-not-exist")
            .join("app.log");

        let result = Ftail::new()
            .single_file(&path, true, LevelFilter::Info)
            .build();

        assert!(matches!(result, Err(FtailError::IoError(_))));
    }
}
