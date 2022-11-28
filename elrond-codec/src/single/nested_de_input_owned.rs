use alloc::boxed::Box;

use crate::{DecodeError, DecodeErrorHandler, NestedDecodeInput};

/// A nested decode buffer that owns its data.
pub struct OwnedBytesNestedDecodeInput {
    bytes: Box<[u8]>,
    decode_index: usize,
}

impl OwnedBytesNestedDecodeInput {
    pub fn new(bytes: Box<[u8]>) -> Self {
        OwnedBytesNestedDecodeInput {
            bytes,
            decode_index: 0,
        }
    }

    fn perform_read_into(&mut self, into: &mut [u8]) {
        let len = into.len();
        into.copy_from_slice(&self.bytes[self.decode_index..self.decode_index + len]);
    }
}

impl NestedDecodeInput for OwnedBytesNestedDecodeInput {
    fn remaining_len(&self) -> usize {
        self.bytes.len() - self.decode_index
    }

    fn peek_into<H>(&mut self, into: &mut [u8], h: H) -> Result<(), H::HandledErr>
    where
        H: DecodeErrorHandler,
    {
        if into.len() > self.remaining_len() {
            return Err(h.handle_error(DecodeError::INPUT_TOO_SHORT));
        }
        self.perform_read_into(into);
        Ok(())
    }

    fn read_into<H>(&mut self, into: &mut [u8], h: H) -> Result<(), H::HandledErr>
    where
        H: DecodeErrorHandler,
    {
        self.peek_into(into, h)?;
        self.decode_index += into.len();
        Ok(())
    }
}
