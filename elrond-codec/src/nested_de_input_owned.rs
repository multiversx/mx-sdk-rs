use alloc::boxed::Box;

use crate::{DecodeError, NestedDecodeInput};

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
        self.decode_index += len;
    }
}

impl NestedDecodeInput for OwnedBytesNestedDecodeInput {
    fn remaining_len(&self) -> usize {
        self.bytes.len() - self.decode_index
    }

    fn read_into(&mut self, into: &mut [u8]) -> Result<(), DecodeError> {
        if into.len() > self.remaining_len() {
            return Err(DecodeError::INPUT_TOO_SHORT);
        }
        self.perform_read_into(into);
        Ok(())
    }

    fn read_into_or_exit<ExitCtx: Clone>(
        &mut self,
        into: &mut [u8],
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) {
        if into.len() > self.remaining_len() {
            exit(c, DecodeError::INPUT_TOO_SHORT);
        }
        self.perform_read_into(into);
    }
}
