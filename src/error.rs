use libheif_sys::heif_error;
use std::result::Result as StdResult;
use std::{error::Error as StdError, fmt::Display};
#[derive(Debug)]
pub enum Error {
    HeifError(heif_error),
}

impl From<heif_error> for Error {
    fn from(err: heif_error) -> Self {
        Error::HeifError(err)
    }
}

pub type Result<T> = StdResult<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::ffi::CStr;
        match *self {
            Error::HeifError(err) => write!(f, "{}", unsafe {
                CStr::from_ptr(err.message).to_string_lossy()
            }),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}
