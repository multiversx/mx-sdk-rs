#[derive(Debug, PartialEq, Eq)]
pub struct EncodeError(&'static str);

impl From<&'static str> for EncodeError {
    #[inline]
    fn from(message_bytes: &'static str) -> Self {
        EncodeError(message_bytes)
    }
}

impl EncodeError {
    #[inline]
    pub fn message_bytes(&self) -> &'static [u8] {
        self.0.as_bytes()
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

impl DecodeError {
    #[inline]
    pub fn message_bytes(&self) -> &'static [u8] {
        self.0.as_bytes()
    }

    pub const INPUT_TOO_SHORT: DecodeError = DecodeError("input too short");
    pub const INPUT_TOO_LONG: DecodeError = DecodeError("input too long");
    pub const INPUT_OUT_OF_RANGE: DecodeError = DecodeError("input out of range");
    pub const INVALID_VALUE: DecodeError = DecodeError("invalid value");
    pub const UNSUPPORTED_OPERATION: DecodeError = DecodeError("unsupported operation");
    pub const ARRAY_DECODE_ERROR: DecodeError = DecodeError("array decode error");
    pub const UTF8_DECODE_ERROR: DecodeError = DecodeError("utf-8 decode error");
    pub const CAPACITY_EXCEEDED_ERROR: DecodeError = DecodeError("capacity exceeded");
}
