use std::cmp::PartialEq;
use std::fmt::{Display, Formatter, Result as DisplayResult};
pub mod day01;

pub enum Part {
    One,
    Two
}

#[derive(Debug, Eq)]
pub enum Output {
    U8(u8),
    U16(u16),
    U32(u32),
    String(String),
}

impl From<u8> for Output {
    fn from(value: u8) -> Self {
        Output::U8(value)
    }
}
impl From<u16> for Output {
    fn from(value: u16) -> Self {
        Output::U16(value)
    }
}

impl From<u32> for Output {
    fn from(value: u32) -> Self {
        Output::U32(value)
    }
}

impl From<String> for Output {
    fn from(value: String) -> Self {
        Output::String(value)
    }
}

impl Display for Output {
    fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
        match self {
            Output::U8(v) => write!(f, "{v}"),
            Output::U16(v) => write!(f, "{v}"),
            Output::U32(v) => write!(f, "{v}"),
            Output::String(v) => write!(f, "{v}"),
        }
    }
}

impl<T: Display> PartialEq<T> for Output {
    fn eq(&self, other: &T) -> bool {
        *self.to_string() == other.to_string()
    }
}

