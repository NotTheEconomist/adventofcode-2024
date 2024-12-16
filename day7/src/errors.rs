use thiserror::Error;

#[derive(Debug, Error)]
pub enum Day7Error {
    #[error("Failed to parse: {}", .0)]
    InputParseError(#[from] nom::Err<nom::error::Error<&'static str>>),
}
