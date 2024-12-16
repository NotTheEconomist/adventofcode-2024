use thiserror::Error;
use nom;

#[derive(Error, Debug)]
pub enum Day6Error {
    #[error("failed to parse input data")]
    InputParseError(#[from] nom::Err<nom::error::Error<&'static str>>),
    #[error("Failed to parse input {}.\n\nGot error: {}", .0, .1)]
    ParseError(String, String)
}
