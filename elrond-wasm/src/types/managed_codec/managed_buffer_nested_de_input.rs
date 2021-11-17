use core::marker::PhantomData;

use elrond_codec::{
    try_execute_then_cast, DecodeError, NestedDecode, NestedDecodeInput, TryStaticCast,
};

use crate::{
    api::ManagedTypeApi,
    types::{
        managed::{preloaded_managed_buffer::PreloadedManagedBuffer, ManagedBufferSizeContext},
        BigInt, BigUint, ManagedBuffer,
    },
};

/// Nested decode buffer based on a managed buffer.
/// Uses the load/copy slice API to extract pieces of the managed buffer for deserialization.
pub struct ManagedBufferNestedDecodeInput<M>
where
    M: ManagedTypeApi,
{
    buffer: PreloadedManagedBuffer<M>,
    decode_index: usize,
    buffer_len: usize,
    _phantom: PhantomData<M>,
}

impl<M> ManagedBufferNestedDecodeInput<M>
where
    M: ManagedTypeApi,
{
    pub fn new(managed_buffer: ManagedBuffer<M>) -> Self {
        // retrieves buffer length eagerly because:
        // - it always gets called anyway at the end to check that no leftover bytes remain
        // - it is sometimes required multiple times during serialization
        let buffer = PreloadedManagedBuffer::new(managed_buffer);
        let buffer_len = buffer.buffer_len;

        ManagedBufferNestedDecodeInput {
            buffer,
            decode_index: 0,
            buffer_len,
            _phantom: PhantomData,
        }
    }

    fn read_managed_buffer(&mut self) -> Result<ManagedBuffer<M>, DecodeError> {
        let size = usize::dep_decode(self)?;
        self.read_managed_buffer_of_size(size)
    }

    fn read_managed_buffer_of_size(
        &mut self,
        size: usize,
    ) -> Result<ManagedBuffer<M>, DecodeError> {
        if let Some(managed_buffer) = self.buffer.copy_slice(self.decode_index, size) {
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
        self.read_managed_buffer_of_size_or_exit(size, c, exit)
    }

    fn read_managed_buffer_of_size_or_exit<ExitCtx: Clone>(
        &mut self,
        size: usize,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> ManagedBuffer<M> {
        if let Some(managed_buffer) = self.buffer.copy_slice(self.decode_index, size) {
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

impl<M> NestedDecodeInput for ManagedBufferNestedDecodeInput<M>
where
    M: ManagedTypeApi,
{
    fn remaining_len(&self) -> usize {
        self.buffer_len - self.decode_index
    }

    fn read_into(&mut self, into: &mut [u8]) -> Result<(), DecodeError> {
        let err_result = self.buffer.load_slice(self.decode_index, into);
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
        let err_result = self.buffer.load_slice(self.decode_index, into);
        if err_result.is_err() {
            exit(c, DecodeError::INPUT_TOO_SHORT);
        }
        self.decode_index += into.len();
    }

    #[inline]
    fn read_specialized<T, C, F>(&mut self, context: C, else_deser: F) -> Result<T, DecodeError>
    where
        T: TryStaticCast,
        C: TryStaticCast,
        F: FnOnce(&mut Self) -> Result<T, DecodeError>,
    {
        if let Some(result) = self.buffer.type_manager().try_cast_ref::<T>() {
            // API for instancing empty Vec
            Ok(result.clone())
        } else if let Some(result) = try_execute_then_cast(|| {
            if let Some(mb_context) = context.try_cast_ref::<ManagedBufferSizeContext>() {
                self.read_managed_buffer_of_size(mb_context.0)
            } else {
                self.read_managed_buffer()
            }
        }) {
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
    fn read_specialized_or_exit<T, C, ExitCtx, F>(
        &mut self,
        context: C,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
        else_deser: F,
    ) -> T
    where
        T: TryStaticCast,
        C: TryStaticCast,
        F: FnOnce(&mut Self, ExitCtx) -> T,
        ExitCtx: Clone,
    {
        if let Some(result) = self.buffer.type_manager().try_cast_ref::<T>() {
            // API for instancing empty Vec
            result.clone()
        } else if let Some(result) = try_execute_then_cast(|| {
            if let Some(mb_context) = context.try_cast_ref::<ManagedBufferSizeContext>() {
                self.read_managed_buffer_of_size_or_exit(mb_context.0, c.clone(), exit)
            } else {
                self.read_managed_buffer_or_exit(c.clone(), exit)
            }
        }) {
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
