use nom::error::ErrorKind;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ParseError {
    #[error("Nom error: {0:?}")]
    Muls(String, ErrorKind)
}

impl From<nom::Err<nom::error::Error<&str>>> for ParseError {
    fn from(value: nom::Err<nom::error::Error<&str>>) -> Self {
        match value {
            nom::Err::Incomplete(_) => unreachable!("we don't handle streaming data"),
            nom::Err::Error(e) => Self::Muls(e.input.to_string(), e.code),
            nom::Err::Failure(e) => Self::Muls(e.input.to_string(), e.code),
        }
    }
}
