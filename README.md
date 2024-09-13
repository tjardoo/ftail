# Ftail

Ftail is simple logging implementation for the `log` crate with support for multiple drivers.

- [Console](#console)
- [Formatted console](#formatted-console)
- [Single file](#single-file)
- [Daily file](#daily-file)
- [Custom driver](#custom-driver)

## Usage

Add the following dependencies to your `Cargo.toml` file:

```toml
[dependencies]
ftail = "0.1"
```

Add the following code to your `main.rs` or `lib.rs` file:

```rust
use ftail::Ftail;
use log::LevelFilter;

Ftail::new()
    .timezone(chrono_tz::Europe::Amsterdam) // optional (default is UTC)
    .console(LevelFilter::Debug)
    .daily_file("logs", LevelFilter::Error)
    .init()?;

// log messages anywhere in your code
log::trace!("This is a trace message");
log::debug!("This is a debug message");
log::info!(target: "foo", "bar");
log::warn!("This is a warning message");
log::error!("This is an error message");
```

## Drivers

### Console

Logs to the standard output without any formatting.

The `stdout` driver takes the following parameters:

- `level`: the minumum log level to log

```rust
Ftail::new()
    .console(LevelFilter::Trace)
    .init()?;
```

```sh
13-09-2024 17:35:18 TRACE console This is a trace message
13-09-2024 17:35:18 DEBUG console This is a debug message
13-09-2024 17:35:18 INFO foo bar
13-09-2024 17:35:18 WARN console This is a warning message
13-09-2024 17:35:18 ERROR console This is an error message
```

### Formatted Console

Logs to the standard output with formatted and colored output.

The `console` driver takes the following parameters:

- `level`: the minumum log level to log

```rust
Ftail::new()
    .formatted_console(LevelFilter::Trace)
    .init()?;
```

```sh
2024-09-13 17:35:37 · TRACE
This is a trace message
examples\formatted_console\src/main.rs:9

2024-09-13 17:35:37 · DEBUG
This is a debug message
examples\formatted_console\src/main.rs:11

2024-09-13 17:35:37 · INFO
bar
examples\formatted_console\src/main.rs:13

2024-09-13 17:35:37 · WARN
This is a warning message
examples\formatted_console\src/main.rs:15

2024-09-13 17:35:37 · ERROR
This is an error message
examples\formatted_console\src/main.rs:17
```

### Single file

Logs to the single log file `logs/demo.log`.

The `single_file` driver takes the following parameters:

- `path`: the path to the log file
- `append`: whether to append to the log file or overwrite it
- `level`: the minumum log level to log

```rust
Ftail::new()
    .single_file("logs/demo.log", true, LevelFilter::Trace)
    .init()?;
```

### Daily file

Logs to a daily log file in the `logs` directory. The log files have the following format: `YYYY-MM-DD.log`.

The `daily_file` driver takes the following parameters:

- `dir`: the directory to store the log files
- `level`: the minumum log level to log

```rust
Ftail::new()
    .daily_file("logs", LevelFilter::Trace)
    .init()?;
```

### Custom driver

Create your own log driver.

```rust
Ftail::new()
    .custom(
        |config: ftail::Config| Box::new(CustomLogger { config }) as Box<dyn Log + Send + Sync>,
        LevelFilter::Debug,
    )
    .datetime_format("%H:%M:%S%.3f")
    .init()?;

// the custom logger implementation
struct CustomLogger {
    config: Config,
}

impl Log for CustomLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let time = chrono::Local::now()
            .format(&self.config.datetime_format)
            .to_string();

        println!("{} [{}] {}", time.black(), record.level().bold(), record.args());
    }

    fn flush(&self) {}
}
```

```sh
19:37:22.402 [DEBUG] This is a debug message
19:37:22.403 [INFO] bar
19:37:22.403 [WARN] This is a warning message
19:37:22.403 [ERROR] This is an error message
```
