
use alloc::vec::Vec;

/// All types that can be returned as error result from smart contracts should implement this trait.
pub trait ErrorMessage {
    fn with_message_slice<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R;
}

#[derive(Debug, PartialEq, Eq)]
pub enum SCError {
    Static(&'static [u8]),
    Dynamic(Vec<u8>),
}

impl SCError {
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            SCError::Static(slice) => slice,
            SCError::Dynamic(vec) => vec.as_slice(),
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
