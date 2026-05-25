/// Error type returned by [`super::Bech32Address::try_from_bech32_string`].
#[derive(Debug)]
pub enum Bech32AddressError {
    /// The string could not be decoded as a valid bech32 value.
    /// Contains a message with the offending input and the underlying error.
    DecodeError(String),
    /// The decoded payload is not exactly 32 bytes.
    /// Contains the actual decoded length.
    InvalidLength(usize),
}

impl core::fmt::Display for Bech32AddressError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Bech32AddressError::DecodeError(msg) => write!(f, "bech32 decode error: {msg}"),
            Bech32AddressError::InvalidLength(len) => {
                write!(
                    f,
                    "invalid address length after decoding: expected 32, got {len}"
                )
            }
        }
    }
}

impl std::error::Error for Bech32AddressError {}
