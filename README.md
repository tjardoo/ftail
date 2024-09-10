# Ftail

Ftail is simple logging implementation for the `log` crate with support for multiple drivers.

- Stdout (standard output logging)
- Console (formatted output logging)
- Daily (daily log rotation)
- Single (single log file)

## Usage

```rust
use ftail::Ftail;

Ftail::new()
    .add_driver(ConsoleLogger::new(), log::LevelFilter::Trace)
    .init()?;

log::debug!("This is a debug message");
log::info!("This is an info message");
```

## Drivers

### Stdout

Logs to the standard output.

```sh
2024-09-10 14:41:57 TRACE stdout This is a trace message
2024-09-10 14:41:57 DEBUG stdout This is a debug message
2024-09-10 14:41:57 INFO foo bar
2024-09-10 14:41:57 WARN stdout This is a warning
2024-09-10 14:41:57 ERROR stdout This is an error
```

### Console

Logs to the standard output with formatted and colored output.

```sh
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
