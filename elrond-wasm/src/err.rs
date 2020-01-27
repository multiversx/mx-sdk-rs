
use alloc::string::String;

/// All types that can be returned as error result from smart contracts should implement this trait.
pub trait ErrorMessage {
    fn message_ptr_and_len(&self) -> (*const u8, usize);
}

impl ErrorMessage for str {
    fn message_ptr_and_len(&self) -> (*const u8, usize) {
        (str::as_ptr(self), str::len(self))
    }
}

impl ErrorMessage for String {
    fn message_ptr_and_len(&self) -> (*const u8, usize) {
        (str::as_ptr(self), str::len(self))
    }
}