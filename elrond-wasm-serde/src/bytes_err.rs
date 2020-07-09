
use alloc::vec::Vec;
use core::fmt::Write;

use core::fmt;

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
    Custom(ErrorBuffer),
}

impl fmt::Display for SDError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "sd error")
    }
}

impl SDError {
    pub fn err_msg_bytes(&self) -> &[u8] {
        match *self {
            SDError::UnsupportedOperation => b"unsupported operation",
            SDError::NotImplemented => b"not yet implemented",
            SDError::SequenceLengthRequired => b"sequence length required",
            SDError::InputTooShort => b"input too short",
            SDError::InputTooLong => b"input too long",
            SDError::InvalidValue => b"invalid value",
            SDError::Custom(ref ebuf) => ebuf.err_msg_slice(),
        }
    }
}

impl serde::de::Error for SDError {
    fn custom<T: fmt::Display>(msg: T) -> SDError {
        SDError::Custom(ErrorBuffer::from_display(msg))
    }
}

impl serde::ser::Error for SDError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        SDError::Custom(ErrorBuffer::from_display(msg))
    }
}

#[derive(Debug)]
pub struct ErrorBuffer {
    output: Vec<u8>,
}

/// Relays custom error messages as byte array.
impl ErrorBuffer {
    pub fn new() -> Self {
        ErrorBuffer {
            output: Vec::new(),
        }
    }
}

impl Default for ErrorBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl ErrorBuffer {
    fn from_display<T: fmt::Display>(msg: T) -> Self {
        let mut ebuf = ErrorBuffer::new();
        if ebuf.write_fmt(format_args!("{}", msg)).is_err() {
            ebuf.output.extend_from_slice(b"fmt err");
        }
        ebuf
    }

    #[inline]
    pub fn err_msg_slice(&self) -> &[u8] {
        self.output.as_slice()
    }
}

impl fmt::Write for ErrorBuffer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.output.extend_from_slice(s.as_bytes());
        Ok(())
    }
}
