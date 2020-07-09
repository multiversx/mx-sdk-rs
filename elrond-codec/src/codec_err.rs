

// TODO: also add an EncodeError

#[derive(Debug)]
pub enum DecodeError {
    InputTooShort,
    InputTooLong,
    InvalidValue,
    UnsupportedOperation,
    ArrayDecodeErr,
    Custom(&'static [u8]),
}

impl DecodeError {
    pub fn message_bytes(&self) -> &'static [u8] {
        match self {
            DecodeError::InputTooShort => &b"input too short"[..],
            DecodeError::InputTooLong => &b"input too long"[..],
            DecodeError::InvalidValue => &b"invalid value"[..],
            DecodeError::UnsupportedOperation => &b"unsupported operation"[..],
            DecodeError::ArrayDecodeErr => &b"array decode error"[..],
            DecodeError::Custom(msg) => msg,
        }
    }
}



