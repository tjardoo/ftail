pub mod default;
pub mod readable;

pub trait Formatter {
    fn format(&self) -> String;
}
