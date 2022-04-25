use std::fmt::Display;

use serde::{de, ser};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Message(String),
    #[error("There are {0} trailing bytes in the data which weren't parsed. Your types must match the exact content of the data.")]
    TrailingBytes(usize),
    #[error("The player data format is not self-describing, only concrete types are supported.")]
    UnknownStructure,
    #[error("This type is not supported.")]
    UnsupportedType,
    #[error("Unexpected end of input.")]
    Eof,
    #[error("Encoded enum does not match structure.")]
    InvalidEnum,
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}
