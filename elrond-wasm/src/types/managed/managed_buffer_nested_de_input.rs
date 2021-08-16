use elrond_codec::{DecodeError, NestedDecode, NestedDecodeInput, TryStaticCast};

use crate::api::ManagedTypeApi;

use super::ManagedBuffer;

/// Nested decode buffer based on a managed buffer.
/// Uses the load/copy slice API to extract pieces of the managed buffer for deserialization.
pub struct ManagedBufferNestedDecodeInput<M: ManagedTypeApi> {
    pub managed_buffer: ManagedBuffer<M>,
    pub decode_index: usize,
    pub buffer_len: usize,
}

impl<M: ManagedTypeApi> ManagedBufferNestedDecodeInput<M> {
    pub fn new(managed_buffer: ManagedBuffer<M>) -> Self {
        // retrieves buffer length eagerly because:
        // - it always gets called anyway at the end to check that no leftover bytes remain
        // - it is sometimes required multiple times during serialization
        let buffer_len = managed_buffer.len();

        ManagedBufferNestedDecodeInput {
            managed_buffer,
            decode_index: 0,
            buffer_len,
        }
    }

    fn read_managed_buffer(&mut self) -> Result<ManagedBuffer<M>, DecodeError> {
        let size = usize::dep_decode(self)?;
        if let Some(managed_buffer) = self.managed_buffer.copy_slice(self.decode_index, size) {
            self.decode_index += size;
            Ok(managed_buffer)
        } else {
            Err(DecodeError::INPUT_TOO_SHORT)
        }
    }

    fn read_managed_buffer_or_exit<ExitCtx: Clone>(
        &mut self,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> ManagedBuffer<M> {
        let size = usize::dep_decode_or_exit(self, c.clone(), exit);
        if let Some(managed_buffer) = self.managed_buffer.copy_slice(self.decode_index, size) {
            self.decode_index += size;
            managed_buffer
        } else {
            exit(c, DecodeError::INPUT_TOO_SHORT)
        }
    }
}

impl<M: ManagedTypeApi> NestedDecodeInput for ManagedBufferNestedDecodeInput<M> {
    fn remaining_len(&self) -> usize {
        self.buffer_len - self.decode_index
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

    #[inline]
    fn read_specialized<T: TryStaticCast>(&mut self) -> Result<Option<T>, DecodeError> {
        if T::type_eq::<ManagedBuffer<M>>() {
            let managed_buffer = self.read_managed_buffer()?;
            Ok(managed_buffer.try_cast())
        } else {
            Ok(None)
        }
    }

    #[inline]
    fn read_specialized_or_exit<T: TryStaticCast, ExitCtx: Clone>(
        &mut self,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Option<T> {
        if T::type_eq::<ManagedBuffer<M>>() {
            let managed_buffer = self.read_managed_buffer_or_exit(c, exit);
            managed_buffer.try_cast()
        } else {
            None
        }
    }
}
