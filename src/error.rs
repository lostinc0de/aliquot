use std::error::Error;
use std::fmt::{Display, Formatter, Result};
use std::num::ParseIntError;

#[derive(Clone, Debug, PartialEq)]
pub enum AliquotError {
    InvalidArg(String),
    InvalidRange(String),
    ConversionError(String),
    OverflowError(String),
}

impl Error for AliquotError {}

impl Display for AliquotError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            AliquotError::InvalidArg(msg) => {
                write!(f, "Invalid argument: {msg}")
            }
            AliquotError::InvalidRange(msg) => {
                write!(f, "Invalid range: {msg}")
            }
            AliquotError::ConversionError(msg) => {
                write!(f, "Conversion error: {msg}")
            }
            AliquotError::OverflowError(msg) => {
                write!(f, "Overflow error: {msg}")
            }
        }
    }
}

impl From<ParseIntError> for AliquotError {
    fn from(error: ParseIntError) -> AliquotError {
        AliquotError::ConversionError(error.to_string())
    }
}
