
// use alloc::vec::Vec;

#[derive(Debug)]
pub enum DeError {
    InputTooShort,
    InputTooLong,
    InvalidValue,
    Custom(&'static [u8]),
}




