use alloc::string::String;

use core::fmt;
use alloc::string::ToString;

use serde;

/// The result of a serialization or deserialization operation.
pub type Result<T> = ::core::result::Result<T, SDError>;

/// The kind of error that can be produced during a serialization or deserialization.
#[derive(Debug)]
pub enum SDError {
    UnsupportedOperation,
    NotImplemented,
    SequenceLengthRequired,

    InputTooShort,
    InputTooLong,
    InvalidValue,

    /// A custom error message from Serde.
    Custom(String),
}

impl fmt::Display for SDError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SDError::UnsupportedOperation => write!(fmt, "unsupported operation"),
            SDError::NotImplemented => write!(fmt, "not yet implemented"),
            SDError::SequenceLengthRequired => write!(fmt, "sequence length required"),
            SDError::InputTooShort => write!(fmt, "input too short"),
            SDError::InputTooLong => write!(fmt, "input too long"),
            SDError::InvalidValue => write!(fmt, "invalid value"),
            SDError::Custom(ref s) => s.fmt(fmt),
        }
    }
}

impl serde::de::Error for SDError {
    fn custom<T: fmt::Display>(desc: T) -> SDError {
        SDError::Custom(desc.to_string())
    }
}

impl serde::ser::Error for SDError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        SDError::Custom(msg.to_string())
    }
}
