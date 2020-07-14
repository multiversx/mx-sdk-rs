
use alloc::vec::Vec;
use elrond_codec::EncodeError;

/// All types that can be returned as error result from smart contracts should implement this trait.
pub trait ErrorMessage {
    fn with_message_slice<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R;
}

#[derive(Debug, PartialEq, Eq)]
pub enum SCError {
    Static(&'static [u8]),
    Dynamic(Vec<u8>),
    PushAsyncEncodeErr(EncodeError),
}

impl SCError {
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            SCError::Static(slice) => slice,
            SCError::Dynamic(vec) => vec.as_slice(),
            SCError::PushAsyncEncodeErr(err) => err.message_bytes(),
        }
    }
}

impl<'a> ErrorMessage for SCError {
    #[inline]
    fn with_message_slice<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R {
        f(self.as_bytes())
    }
}

impl ErrorMessage for &str {
    #[inline]
    fn with_message_slice<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R {
        f(self.as_bytes())
    }
}

impl From<EncodeError> for SCError {
    #[inline]
    fn from(err: EncodeError) -> Self {
        SCError::PushAsyncEncodeErr(err)
    }
}