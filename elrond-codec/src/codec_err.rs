use alloc::vec::Vec;

#[derive(Debug, PartialEq, Eq)]
pub enum EncodeError {
    UnsupportedOperation,
    Static(&'static [u8]),
    Dynamic(Vec<u8>),
}

impl EncodeError {
    pub fn message_bytes(&self) -> &[u8] {
        match self {
            EncodeError::UnsupportedOperation => &b"unsupported operation"[..],
            EncodeError::Static(msg) => msg,
            EncodeError::Dynamic(msg) => msg.as_slice(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum DecodeError {
    InputTooShort,
    InputTooLong,
    InvalidValue,
    UnsupportedOperation,
    ArrayDecodeErr,
    Static(&'static [u8]),
    Dynamic(Vec<u8>),
}

impl DecodeError {
    pub fn message_bytes(&self) -> &[u8] {
        match self {
            DecodeError::InputTooShort => &b"input too short"[..],
            DecodeError::InputTooLong => &b"input too long"[..],
            DecodeError::InvalidValue => &b"invalid value"[..],
            DecodeError::UnsupportedOperation => &b"unsupported operation"[..],
            DecodeError::ArrayDecodeErr => &b"array decode error"[..],
            DecodeError::Static(msg) => msg,
            DecodeError::Dynamic(msg) => msg.as_slice(),
        }
    }
}



