#[derive(Debug, PartialEq, Eq)]
pub struct EncodeError(&'static [u8]);

impl From<&'static [u8]> for EncodeError {
    #[inline]
    fn from(message_bytes: &'static [u8]) -> Self {
        EncodeError(message_bytes)
    }
}

impl EncodeError {
    #[inline]
    pub fn message_bytes(&self) -> &'static [u8] {
        self.0
    }

    pub const UNSUPPORTED_OPERATION: EncodeError = EncodeError(b"unsupported operation");
}

#[derive(Debug, PartialEq, Eq)]
pub struct DecodeError(&'static [u8]);

impl From<&'static [u8]> for DecodeError {
    #[inline]
    fn from(message_bytes: &'static [u8]) -> Self {
        DecodeError(message_bytes)
    }
}

impl DecodeError {
    #[inline]
    pub fn message_bytes(&self) -> &'static [u8] {
        self.0
    }

    pub const INPUT_TOO_SHORT: DecodeError = DecodeError(b"input too short");
    pub const INPUT_TOO_LONG: DecodeError = DecodeError(b"input too long");
    pub const INPUT_OUT_OF_RANGE: DecodeError = DecodeError(b"input out of range");
    pub const INVALID_VALUE: DecodeError = DecodeError(b"invalid value");
    pub const UNSUPPORTED_OPERATION: DecodeError = DecodeError(b"unsupported operation");
    pub const ARRAY_DECODE_ERROR: DecodeError = DecodeError(b"array decode error");
    pub const UTF8_DECODE_ERROR: DecodeError = DecodeError(b"utf-8 decode error");
}
