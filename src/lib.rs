use crate::executor::execute;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, FourADError>;

#[derive(Debug, Error)]
pub enum FourADError {
    #[error("Expected '{0}' but found '{1}'")]
    UnexpectedChar(char, String),

    #[error("Unexpected end of input while parsing '{0}'.")]
    UnexpectedEndOfString(String),

    // TODO: consider making this an error with the bad string in it.
    #[error("Could not parse a number")]
    ParseNumberError(#[from] std::num::ParseIntError),

    #[error("Dice cannot have zero sides or one side.")]
    ZeroOrOneSide,

    #[error("Repeating zero times is not allowed")]
    ZeroRepeats,
}

pub type Error = FourADError;

mod executor;
mod rolldesc;
mod roller;

pub fn roll(diecode: &str, explodes: bool) -> Result<i16> {
    Ok(execute(diecode.parse()?, explodes))
}
