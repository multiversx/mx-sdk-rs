#[derive(Debug)]
pub enum KeystoreError {
    InvalidPassword,
    InvalidKdf,
    InvalidCipher,
    HexDecodeError(hex::FromHexError),
    Other(anyhow::Error),
}

impl std::fmt::Display for KeystoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeystoreError::InvalidPassword => write!(f, "invalid password"),
            KeystoreError::InvalidKdf => write!(f, "invalid KDF"),
            KeystoreError::InvalidCipher => write!(f, "invalid cipher"),
            KeystoreError::HexDecodeError(e) => write!(f, "hex decode error: {e}"),
            KeystoreError::Other(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for KeystoreError {}

impl From<anyhow::Error> for KeystoreError {
    fn from(e: anyhow::Error) -> Self {
        KeystoreError::Other(e)
    }
}

impl From<hex::FromHexError> for KeystoreError {
    fn from(e: hex::FromHexError) -> Self {
        KeystoreError::HexDecodeError(e)
    }
}
