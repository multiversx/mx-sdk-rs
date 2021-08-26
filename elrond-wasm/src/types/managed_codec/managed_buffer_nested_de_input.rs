use elrond_codec::{
    try_execute_then_cast, DecodeError, NestedDecode, NestedDecodeInput, TryStaticCast,
};

use crate::{
    api::ManagedTypeApi,
    types::{BigInt, BigUint, ManagedBuffer},
};

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

    fn read_big_uint(&mut self) -> Result<BigUint<M>, DecodeError> {
        Ok(BigUint::from_bytes_be_buffer(&self.read_managed_buffer()?))
    }

    fn read_big_uint_or_exit<ExitCtx: Clone>(
        &mut self,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> BigUint<M> {
        BigUint::from_bytes_be_buffer(&self.read_managed_buffer_or_exit(c, exit))
    }

    fn read_big_int(&mut self) -> Result<BigInt<M>, DecodeError> {
        Ok(BigInt::from_signed_bytes_be_buffer(
            &self.read_managed_buffer()?,
        ))
    }

    fn read_big_int_or_exit<ExitCtx: Clone>(
        &mut self,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> BigInt<M> {
        BigInt::from_signed_bytes_be_buffer(&self.read_managed_buffer_or_exit(c, exit))
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
    fn read_specialized<T, F>(&mut self, else_deser: F) -> Result<T, DecodeError>
    where
        T: TryStaticCast,
        F: FnOnce(&mut Self) -> Result<T, DecodeError>,
    {
        if let Some(result) = try_execute_then_cast(|| self.read_managed_buffer()) {
            result
        } else if let Some(result) = try_execute_then_cast(|| self.read_big_uint()) {
            result
        } else if let Some(result) = try_execute_then_cast(|| self.read_big_int()) {
            result
        } else {
            else_deser(self)
        }
    }

    #[inline]
    fn read_specialized_or_exit<T, ExitCtx, F>(
        &mut self,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
        else_deser: F,
    ) -> T
    where
        T: TryStaticCast,
        F: FnOnce(&mut Self, ExitCtx) -> T,
        ExitCtx: Clone,
    {
        if let Some(result) =
            try_execute_then_cast(|| self.read_managed_buffer_or_exit(c.clone(), exit))
        {
            result
        } else if let Some(result) =
            try_execute_then_cast(|| self.read_big_uint_or_exit(c.clone(), exit))
        {
            result
        } else if let Some(result) =
            try_execute_then_cast(|| self.read_big_int_or_exit(c.clone(), exit))
        {
            result
        } else {
            else_deser(self, c)
        }
    }
}
