#[cfg(feature = "console")]
pub mod console;
#[cfg(feature = "daily_file")]
pub mod daily_file;
#[cfg(feature = "formatted_console")]
pub mod formatted_console;
#[cfg(feature = "single_file")]
pub mod single_file;
#[cfg(test)]
pub mod test;
