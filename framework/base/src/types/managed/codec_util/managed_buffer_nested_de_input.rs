use core::marker::PhantomData;

use crate::codec::{
    try_execute_then_cast, DecodeError, DecodeErrorHandler, NestedDecode, NestedDecodeInput,
    TryStaticCast,
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

    fn read_managed_buffer_of_size<H>(
        &mut self,
        size: usize,
        h: H,
    ) -> Result<ManagedBuffer<M>, H::HandledErr>
    where
        H: DecodeErrorHandler,
    {
        if let Some(managed_buffer) = self.buffer.copy_slice(self.decode_index, size) {
            self.decode_index += size;
            Ok(managed_buffer)
        } else {
            Err(h.handle_error(DecodeError::INPUT_TOO_SHORT))
        }
    }

    fn read_managed_buffer<H>(&mut self, h: H) -> Result<ManagedBuffer<M>, H::HandledErr>
    where
        H: DecodeErrorHandler,
    {
        let size = usize::dep_decode_or_handle_err(self, h)?;
        self.read_managed_buffer_of_size(size, h)
    }

    fn read_big_uint<H: DecodeErrorHandler>(&mut self, h: H) -> Result<BigUint<M>, H::HandledErr> {
        Ok(BigUint::from_bytes_be_buffer(&self.read_managed_buffer(h)?))
    }

    fn read_big_int<H: DecodeErrorHandler>(&mut self, h: H) -> Result<BigInt<M>, H::HandledErr> {
        Ok(BigInt::from_signed_bytes_be_buffer(
            &self.read_managed_buffer(h)?,
        ))
    }
}

impl<M> NestedDecodeInput for ManagedBufferNestedDecodeInput<M>
where
    M: ManagedTypeApi,
{
    fn remaining_len(&self) -> usize {
        self.buffer_len - self.decode_index
    }

    fn peek_into<H>(&mut self, into: &mut [u8], h: H) -> Result<(), H::HandledErr>
    where
        H: DecodeErrorHandler,
    {
        self.buffer
            .load_slice(self.decode_index, into)
            .map_err(|_| h.handle_error(DecodeError::INPUT_TOO_SHORT))
    }

    fn read_into<H>(&mut self, into: &mut [u8], h: H) -> Result<(), H::HandledErr>
    where
        H: DecodeErrorHandler,
    {
        self.peek_into(into, h)?;
        self.decode_index += into.len();
        Ok(())
    }

    fn supports_specialized_type<T: TryStaticCast>() -> bool {
        T::type_eq::<ManagedBuffer<M>>() || T::type_eq::<BigUint<M>>() || T::type_eq::<BigInt<M>>()
    }

    fn read_specialized<T, C, H>(&mut self, context: C, h: H) -> Result<T, H::HandledErr>
    where
        T: TryStaticCast,
        C: TryStaticCast,
        H: DecodeErrorHandler,
    {
        if let Some(result) = try_execute_then_cast(|| {
            if let Some(mb_context) = context.try_cast_ref::<ManagedBufferSizeContext>() {
                self.read_managed_buffer_of_size(mb_context.0, h)
            } else {
                self.read_managed_buffer(h)
            }
        }) {
            result
        } else if let Some(result) = try_execute_then_cast(|| self.read_big_uint(h)) {
            result
        } else if let Some(result) = try_execute_then_cast(|| self.read_big_int(h)) {
            result
        } else {
            Err(h.handle_error(DecodeError::UNSUPPORTED_OPERATION))
        }
    }
}
