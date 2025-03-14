use std::{error::Error, fmt};

#[derive(Debug)]
pub struct WasmerAltExecutorFileNotFoundError(pub(super) String);

impl fmt::Display for WasmerAltExecutorFileNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Contract {}", &self.0)
    }
}

impl Error for WasmerAltExecutorFileNotFoundError {}
