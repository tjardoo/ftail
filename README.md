# Ftail

Ftail is simple logging implementation for the `log` crate with support for multiple drivers.

- [Console](#console)
- [Formatted console](#formatted-console)
- [Single file](#single-file)
- [Daily file](#daily-file)
- [Custom driver](#custom-driver)

## Usage

```rust
use ftail::Ftail;
use log::LevelFilter;

Ftail::new()
    .console(LevelFilter::Debug)
    .daily_file("logs", LevelFilter::Error)
    .init()?;

log::trace!("This is a trace message");
log::debug!("This is a debug message");
log::info!("This is an info message");
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

```log
2024-09-10 14:41:57 TRACE stdout This is a trace message
2024-09-10 14:41:57 DEBUG stdout This is a debug message
2024-09-10 14:41:57 INFO foo bar
2024-09-10 14:41:57 WARN stdout This is a warning message
2024-09-10 14:41:57 ERROR stdout This is an error message
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

```log
2024-09-10 14:42:21 · TRACE
This is a trace message
examples\console\src/main.rs:8

2024-09-10 14:42:21 · DEBUG
This is a debug message
examples\console\src/main.rs:10

2024-09-10 14:42:21 · INFO
bar
examples\console\src/main.rs:12

2024-09-10 14:42:21 · WARN
This is a warning message
examples\console\src/main.rs:14

2024-09-10 14:42:21 · ERROR
This is an error message
examples\console\src/main.rs:16
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

Logs to a daily log file in the `logs` directory.

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

You can add text formatting, by using the `use ftail::ansi_escape::TextStyling;` module.

```rust
Ftail::new()
    .custom(Box::new(Box::new(CustomLogger {})), LevelFilter::Debug)
    .init()?;

// the custom logger implementation
struct CustomLogger {}

impl Log for CustomLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let time = chrono::Local::now().format("%H:%M:%S").to_string();

        println!("{} {} {}", time, record.level(), record.args());
    }

    fn flush(&self) {}
}
```
