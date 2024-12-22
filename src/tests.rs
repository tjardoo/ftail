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

// The `log` crate allow only one logger to be set. When running `cargo test` multiple tests are run concurrently.
// This causes the logger to be set multiple times which results in a panic. To avoid this, we need to run the tests invidually.

#[allow(clippy::module_inception)]
#[cfg(test)]
mod tests {
    use crate::{channels::test::TestLogger, tests::remove_datetime_from_message, Config, Ftail};
    use log::{Level, Log, Metadata, Record};
    use std::sync::{Arc, Mutex};

    fn get_message_from_logger(
        config: &Config,
        metadata: Metadata,
        args: std::fmt::Arguments,
    ) -> String {
        let buffer = Arc::new(Mutex::new(Vec::new()));

        let level_filter = config.level_filter;
        let datetime_format = config.datetime_format.clone();
        let timezone = config.timezone;
        let levels = config.levels.clone();
        let targets = config.targets.clone();

        let mut ftail = Ftail::new()
            .custom(
                {
                    let config = config.clone();
                    let buffer = Arc::clone(&buffer);

                    move |_config| {
                        Box::new(TestLogger {
                            config: config.clone(),
                            buffer: Arc::clone(&buffer),
                        }) as Box<dyn Log + Send + Sync>
                    }
                },
                level_filter,
            )
            .datetime_format(&datetime_format);

        #[cfg(feature = "timezone")]
        {
            ftail = ftail.timezone(timezone);
        }

        if let Some(levels) = levels {
            ftail = ftail.filter_levels(levels);
        }

        if let Some(targets) = targets {
            let targets = targets
                .iter()
                .map(|target| target.as_str())
                .collect::<Vec<&str>>();

            ftail = ftail.filter_targets(targets);
        }

        ftail.init().unwrap();

        let record = Record::builder().metadata(metadata).args(args).build();

        log::logger().log(&record);

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
        let config = Config {
            datetime_format: "%Z".to_string(),
            timezone: ftail::Tz::Australia__Sydney,
            ..Default::default()
        };

        let metadata = Metadata::builder()
            .level(Level::Debug)
            .target("test")
            .build();

        let args = format_args!("Hello, world!");

        let message = get_message_from_logger(&config, metadata, args);

        assert_eq!(message, "AEDT DEBUG test Hello, world!");
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
}
