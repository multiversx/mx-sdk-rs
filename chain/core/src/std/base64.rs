use base64::{Engine as _, engine::general_purpose};

/// Decodes a base64-encoded byte slice using the standard alphabet.
///
/// Returns an error if the input contains characters outside the base64 alphabet
/// or has invalid padding.
pub fn base64_decode<T>(to_decode: T) -> Result<Vec<u8>, base64::DecodeError>
where
    T: AsRef<[u8]>,
{
    general_purpose::STANDARD.decode(to_decode)
}

/// Encodes a byte slice as a base64 string using the standard alphabet with padding.
pub fn base64_encode<T>(to_encode: T) -> String
where
    T: AsRef<[u8]>,
{
    general_purpose::STANDARD.encode(to_encode)
}
