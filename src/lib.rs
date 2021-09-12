use thiserror::Error;

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