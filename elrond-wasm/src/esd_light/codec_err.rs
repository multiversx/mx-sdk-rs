
// use alloc::vec::Vec;

#[derive(Debug)]
pub enum DeError {
    InputTooShort,
    InputTooLong,
    InvalidValue,
    Custom(&'static [u8]),
}

impl DeError {
    pub fn message_bytes(&self) -> &'static [u8] {
        match self {
            DeError::InputTooShort => &b"input too short"[..],
            DeError::InputTooLong => &b"input too long"[..],
            DeError::InvalidValue => &b"invalid value"[..],
            DeError::Custom(msg) => msg,
        }
    }
}



