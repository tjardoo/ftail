# Ftail

**Ftail** is a simple logging implementation for the [`log`](https://crates.io/crates/log) crate with support for multiple channels, including console output, files, and custom loggers.

---

## Features

* Multiple logging channels: console, formatted console, single file, daily file, and custom
* Level and target filtering
* Optional timestamp formatting and timezone support
* Automatic log rotation and retention
* Use `RUST_LOG` environment variable for dynamic log level control (use '_env_level' postfix)

---

## Quick Start

Add Ftail to your `Cargo.toml`:

```toml
[dependencies]
ftail = "0.3"
```

Initialize Ftail in your `main.rs` or `lib.rs`:

```rust
use ftail::Ftail;
use log::LevelFilter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ftail::new()
        .console(LevelFilter::Info)        // log to console
        .daily_file("logs", LevelFilter::Error) // log errors to daily files
        .init()?; // initialize logger

    log::info!("Hello, Ftail!");
    log::error!("This is an error message");
    Ok(())
}
```

---

## Configuration Options

| Option               | Description                      | Notes                         |
| -------------------- | -------------------------------- | ----------------------------- |
| `.datetime_format()` | Set the datetime format          | Ex: `"%Y-%m-%d %H:%M:%S%.3f"` |
| `.timezone()`        | Set the timezone                 | Requires `timezone` feature   |
| `.max_file_size()`   | Maximum file size in MB          | Older logs renamed `.old{N}`  |
| `.retention_days()`  | Number of days to keep log files | Daily file only               |
| `.filter_levels()`   | Log only the specified levels    | `Vec<Level>`                  |
| `.filter_targets()`  | Log only the specified targets   | `Vec<&str>`                   |

---

## Channels

### Console

Logs to the standard output without formatting.

```rust
Ftail::new()
    .console(LevelFilter::Trace)
    .init()?;
```

**Output:**

```text
2024-09-13 17:35:18 TRACE console This is a trace message
2024-09-13 17:35:18 DEBUG console This is a debug message
2024-09-13 17:35:18 INFO foo bar
2024-09-13 17:35:18 WARN console This is a warning message
2024-09-13 17:35:18 ERROR console This is an error message
```

---

### Formatted Console

Logs with formatted and colored output.

```rust
Ftail::new()
    .formatted_console(LevelFilter::Trace)
    .init()?;
```

**Output:**

```text
2024-09-13 17:35:37 · TRACE
This is a trace message
examples/formatted_console/src/main.rs:9

2024-09-13 17:35:37 · DEBUG
This is a debug message
examples/formatted_console/src/main.rs:11
```

---

### Single File

Logs to a single file (e.g., `logs/demo.log`).

```rust
Ftail::new()
    .single_file(Path::new("logs/demo.log"), true, LevelFilter::Trace)
    .init()?;
```

* `append = true` keeps existing logs, `false` overwrites.

---

### Daily File

Logs to daily files in a directory (e.g., `logs/2025-09-25.log`).

```rust
Ftail::new()
    .daily_file(Path::new("logs"), LevelFilter::Trace)
    .init()?;
```

* Automatically rotates files per day.

---

### Custom Channel

Create your own logger by implementing the `log::Log` trait:

```rust
Ftail::new()
    .custom(
        |config| Box::new(CustomLogger { config }) as Box<dyn Log + Send + Sync>,
        LevelFilter::Debug,
    )
    .datetime_format("%H:%M:%S%.3f")
    .init()?;

struct CustomLogger {
    config: ftail::Config,
}

impl log::Log for CustomLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.config.level_filter
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let time = chrono::Local::now()
            .format(&self.config.datetime_format)
            .to_string();
        println!("[{}] {}: {}", time, record.level(), record.args());
    }

    fn flush(&self) {}
}
```

**Output:**

```text
19:37:22.402 [DEBUG] This is a debug message
19:37:22.403 [INFO] bar
19:37:22.403 [WARN] This is a warning message
19:37:22.403 [ERROR] This is an error message
```

---

## Tips

* Use `.console(LevelFilter::Debug)` for development.
* Use `.daily_file()` in production to organize logs by date.
* Combine `.max_file_size()` and `.retention_days()` to prevent disk bloat.
* Only enable the channels you need to reduce overhead by disabling default features in `Cargo.toml`.
