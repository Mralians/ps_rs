use std::num::ParseIntError;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum ProcessError {
    #[error("parse: {0}")]
    Parse(#[from] ParseIntError),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("cannot find value: {0}")]
    Match(String),
    #[error("Field parsing error: {0}")]
    FieldParseErrpr(String),
    #[error("Signal conversion error: cannot convert signal {0}")]
    SignalConversionError(u32),
}

impl From<ProcessError> for std::io::Error {
    fn from(value: ProcessError) -> Self {
        match value {
            ProcessError::Io(e) => e,
            e => std::io::Error::new(std::io::ErrorKind::Other, e),
        }
    }
}
