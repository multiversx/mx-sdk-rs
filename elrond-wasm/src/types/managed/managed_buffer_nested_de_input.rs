use elrond_codec::{DecodeError, NestedDecodeInput};

use crate::api::ManagedTypeApi;

use super::ManagedBuffer;

/// Nested decode buffer based on a managed buffer.
/// Uses the load/copy slice API to extract pieces of the managed buffer for deserialization.
pub struct ManagedBufferNestedDecodeInput<M: ManagedTypeApi> {
    pub managed_buffer: ManagedBuffer<M>,
    pub decode_index: usize,
}

impl<M: ManagedTypeApi> ManagedBufferNestedDecodeInput<M> {
    pub fn new(managed_buffer: ManagedBuffer<M>) -> Self {
        ManagedBufferNestedDecodeInput {
            managed_buffer,
            decode_index: 0,
        }
    }
}

impl<M: ManagedTypeApi> NestedDecodeInput for ManagedBufferNestedDecodeInput<M> {
    fn remaining_len(&self) -> usize {
        self.managed_buffer.len() - self.decode_index
    }

    fn read_into(&mut self, into: &mut [u8]) -> Result<(), DecodeError> {
        let err_result = self.managed_buffer.load_slice(self.decode_index, into);
        if err_result.is_ok() {
            self.decode_index += into.len();
            Ok(())
        } else {
            Err(DecodeError::INPUT_TOO_SHORT)
        }
    }

    fn read_into_or_exit<ExitCtx: Clone>(
        &mut self,
        into: &mut [u8],
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) {
        let err_result = self.managed_buffer.load_slice(self.decode_index, into);
        if err_result.is_err() {
            exit(c, DecodeError::INPUT_TOO_SHORT);
        }
        self.decode_index += into.len();
    }
}
