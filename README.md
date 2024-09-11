# Ftail

Ftail is simple logging implementation for the `log` crate with support for multiple drivers.

- [Stdout (standard output logging)](#stdout)
- [Console (formatted output logging)](#console)
- [Single (single log file)](#single)
- [Daily (daily log rotation)](#daily)
- [Custom (custom log driver)](#custom)

## Usage

```rust
use ftail::Ftail;
use log::LevelFilter;

Ftail::new()
    .stdout(LevelFilter::Trace)
    .init()?;

log::debug!("This is a debug message");
log::info!("This is an info message");
```

## Drivers

### Stdout

Logs to the standard output.

```rust
Ftail::new()
    .stdout(LevelFilter::Trace)
    .init()?;
```

```log
2024-09-10 14:41:57 TRACE stdout This is a trace message
2024-09-10 14:41:57 DEBUG stdout This is a debug message
2024-09-10 14:41:57 INFO foo bar
2024-09-10 14:41:57 WARN stdout This is a warning
2024-09-10 14:41:57 ERROR stdout This is an error
```

### Console

Logs to the standard output with formatted and colored output.

```rust
Ftail::new()
    .console(LevelFilter::Trace)
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
This is a warning
examples\console\src/main.rs:14

2024-09-10 14:42:21 · ERROR
This is an error
examples\console\src/main.rs:16
```

### Single

TODO

### Daily

TODO

### Custom

Create your own log driver.

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
