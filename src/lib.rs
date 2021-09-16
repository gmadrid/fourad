use thiserror::Error;

mod executor;
use executor::execute;

mod grammar;
mod roller;

#[macro_use]
mod spew;

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

    #[error("An I/O error occurred")]
    IO(#[from] std::io::Error),

    #[error("Unexpected input at end of line: {0}")]
    UnexpectedEOL(String),

    #[error("Error: (0)")]
    GeneralError(String),
}

pub type Error = FourADError;

pub fn roll(diecode: &str, explode: bool, force_66: bool) -> Result<i16> {
    Ok(execute(diecode.parse()?, explode, force_66))
}

// *_fa functions are exported for the macros to use.
pub use spew::{quiet_fa, set_level, spew_fa, verbose_fa, SpewLevel};
