# Changelog

## v0.4.0 (2026-11-23)

- **Behaviour change:** `LevelFilter::Off` (and `RUST_LOG=off`) now disables a channel. Previously it logged every level; use `LevelFilter::Trace` to log everything. `Config::new()` now defaults to `LevelFilter::Trace`
- **Breaking:** the constructor passed to `custom()` must be `Send + Sync`. Closures that only capture thread-safe values (as in the examples) are unaffected. This removes the `unsafe impl Send/Sync for Ftail`, which allowed a data race in safe code
- Fix `single_file` with `append = false` not overwriting the existing file
- Fix the daily file channel writing to the previous day's file after a `max_file_size` rotation
- `RUST_LOG` values like `info,hyper=warn` no longer panic; the bare level is used, otherwise `info`
- `init()` returns an error instead of panicking when a log file can't be opened
- File channels no longer panic on I/O errors while logging
- Formatted console: TRACE, timestamps and file locations are readable on dark terminals
- Fix the examples in the README

## v0.3.1 (2025-09-25)

- Moved all channels behind a feature flag
- Support environment variable `RUST_LOG` for each channel

## v0.3.0 (2025-05-13)

- Use `Path`/`Pathbuf` for directory and file paths

## v0.2.1 (2025-03-04)

- Add retention policy to daily file channel

## v0.2.0 (2024-12-22)

- Rename drivers to channels

## v0.1.2 (2024-09-17)

- Rotate log file if file size exceeds `max_file_size`
- Add `levels` filter to `Config`
- Add `targets` filter to `Config`
- Add `test` channel to be used in tests

## v0.1.1 (2024-09-14)

- Made timezone optional with `timezone` feature flag
- Re-exported `chrono_tz` crate

## v0.1.0 (2024-09-13)

- Initial release
