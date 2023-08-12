#[derive(Debug, PartialEq, Eq)]
pub struct EncodeError(&'static str);

impl From<&'static str> for EncodeError {
    #[inline]
    fn from(message_bytes: &'static str) -> Self {
        EncodeError(message_bytes)
    }
}

// TODO: convert to "from_bytes" deprecated method in next minor release.
// Please avoid: it bloats the contract with an unnecessary utf8 validation.
impl From<&'static [u8]> for EncodeError {
    #[inline]
    fn from(message_bytes: &'static [u8]) -> Self {
        EncodeError(core::str::from_utf8(message_bytes).unwrap())
    }
}

impl EncodeError {
    #[inline]
    pub fn message_bytes(&self) -> &'static [u8] {
        self.0.as_bytes()
    }

    #[inline]
    pub fn message_str(&self) -> &'static str {
        self.0
    }

    pub const UNSUPPORTED_OPERATION: EncodeError = EncodeError("unsupported operation");
}

#[derive(Debug, PartialEq, Eq)]
pub struct DecodeError(&'static str);

impl From<&'static str> for DecodeError {
    #[inline]
    fn from(message_bytes: &'static str) -> Self {
        DecodeError(message_bytes)
    }
}

// TODO: convert to "from_bytes" deprecated method in next minor release.
// Please avoid: it bloats the contract with an unnecessary utf8 validation.
impl From<&'static [u8]> for DecodeError {
    #[inline]
    fn from(message_bytes: &'static [u8]) -> Self {
        DecodeError(core::str::from_utf8(message_bytes).unwrap())
    }
}

impl DecodeError {
    #[inline]
    pub fn message_bytes(&self) -> &'static [u8] {
        self.0.as_bytes()
    }

    #[inline]
    pub fn message_str(&self) -> &'static str {
        self.0
    }

    pub const INPUT_TOO_SHORT: DecodeError = DecodeError("input too short");
    pub const INPUT_TOO_LONG: DecodeError = DecodeError("input too long");
    pub const INPUT_OUT_OF_RANGE: DecodeError = DecodeError("input out of range");
    pub const INVALID_VALUE: DecodeError = DecodeError("invalid value");
    pub const UNSUPPORTED_OPERATION: DecodeError = DecodeError("unsupported operation");
    pub const ARRAY_DECODE_ERROR: DecodeError = DecodeError("array decode error");
    pub const UTF8_DECODE_ERROR: DecodeError = DecodeError("utf-8 decode error");
    pub const CAPACITY_EXCEEDED_ERROR: DecodeError = DecodeError("capacity exceeded");

    pub const MULTI_TOO_FEW_ARGS: DecodeError = DecodeError("too few arguments");
    pub const MULTI_TOO_MANY_ARGS: DecodeError = DecodeError("too many arguments");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn decode_error_from_bytes() {
        let from_bytes = DecodeError::from(&b"error as bytes"[..]);
        assert_eq!(from_bytes.message_bytes(), b"error as bytes");
        assert_eq!(from_bytes.message_str(), "error as bytes");
    }

    #[test]
    #[should_panic]
    fn decode_error_from_bad_bytes() {
        let _ = DecodeError::from(&[0, 159, 146, 150][..]);
    }

    #[test]
    fn encode_error_from_bytes() {
        let from_bytes = EncodeError::from(&b"error as bytes"[..]);
        assert_eq!(from_bytes.message_bytes(), b"error as bytes");
        assert_eq!(from_bytes.message_str(), "error as bytes");
    }

    #[test]
    #[should_panic]
    fn encode_error_from_bad_bytes() {
        let _ = EncodeError::from(&[0, 159, 146, 150][..]);
    }
}
