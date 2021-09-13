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

    #[error("'{0}' is not a legal value for the number of sides")]
    BadSidesString(String, std::num::ParseIntError),

    // Do not use this in new grammar.
    // TODO: delete this.
    #[error("'{0}' is not a legal repeat string")]
    ParseRepeatError(String, std::num::ParseIntError),

    #[error("Dice cannot have zero sides or one side.")]
    ZeroOrOneSide,

    #[error("Repeating zero times is not allowed")]
    ZeroRepeats,

    // TODO: get rid of UnknownError
    #[error("an unknown error has occurred. This should never happen.")]
    UnknownError,
}

pub type Error = FourADError;

mod executor;
mod rolldesc;
mod roller;

pub fn roll(diecode: &str) -> Result<i16> {
    Ok(execute(diecode.parse()?, true))
}
