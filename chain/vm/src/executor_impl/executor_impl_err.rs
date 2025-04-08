use std::{error::Error, fmt};

#[derive(Debug)]
pub struct ExecutorFileNotFoundError(pub(super) String);

impl fmt::Display for ExecutorFileNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Contract {}", &self.0)
    }
}

impl Error for ExecutorFileNotFoundError {}
