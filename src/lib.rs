use thiserror::Error;
use crate::executor::execute;

pub type Result<T> = std::result::Result<T, FourADError>;

#[derive(Debug, Error)]
pub enum FourADError {
    #[error("'{0}' is not a legal value for the number of sides")]
    BadSidesString(String, std::num::ParseIntError),

    #[error("'{0}' is not a legal repeat string")]
    ParseRepeatError(String, std::num::ParseIntError),

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
