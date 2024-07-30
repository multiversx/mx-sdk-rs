use crate::api::ManagedTypeApi;
use crate::types::{BigInt, BigUint, BoxedBytes, ManagedBuffer};
use multiversx_sc_codec::{
    try_execute_then_cast, DecodeError, DecodeErrorHandler, EncodeError, EncodeErrorHandler,
    NestedDecode, NestedDecodeInput, NestedEncode, NestedEncodeOutput, TopDecode, TopDecodeInput,
    TopEncode, TopEncodeOutput, TryStaticCast,
};

/// A wrapper over a ManagedBuffer with different decode properties. It reads until the end of the buffer.
#[repr(transparent)]
pub struct ManagedNestedBuffer<M: ManagedTypeApi> {
    pub(crate) buffer: ManagedBuffer<M>,
}

impl<M: ManagedTypeApi> ManagedNestedBuffer<M> {
    #[inline]
    pub fn new_from_bytes(bytes: &[u8]) -> Self {
        Self {
            buffer: ManagedBuffer::new_from_bytes(bytes),
        }
    }

    #[inline]
    pub fn new_from_buf(buf: ManagedBuffer<M>) -> Self {
        Self { buffer: buf }
    }

    fn read_nested_managed_buffer<C, H>(
        &mut self,
        context: C,
        h: H,
    ) -> Result<ManagedNestedBuffer<M>, H::HandledErr>
    where
        C: TryStaticCast,
        H: DecodeErrorHandler,
    {
        self.read_and_flush_managed_buffer(context, h)
    }

    fn read_and_flush_managed_buffer<C, H>(
        &mut self,
        context: C,
        _h: H,
    ) -> Result<ManagedNestedBuffer<M>, H::HandledErr>
    where
        C: TryStaticCast,
        H: DecodeErrorHandler,
    {
        let mut result_managed_buf = ManagedBuffer::new();
        while let Some(buffer) = context.try_cast_ref::<ManagedBuffer<M>>() {
            // read until end
            result_managed_buf.append(&buffer)
        }

        Ok(ManagedNestedBuffer::from(result_managed_buf))
    }
}

impl<M> From<ManagedBuffer<M>> for ManagedNestedBuffer<M>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn from(buf: ManagedBuffer<M>) -> Self {
        Self::new_from_buf(buf)
    }
}

impl<M: ManagedTypeApi> TryStaticCast for ManagedNestedBuffer<M> {}

impl<M: ManagedTypeApi> TopEncode for ManagedNestedBuffer<M> {
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        if O::supports_specialized_type::<Self>() {
            output.set_specialized(self, h)
        } else {
            output.set_slice_u8(self.buffer.to_boxed_bytes().as_slice());
            Ok(())
        }
    }
}

impl<M: ManagedTypeApi> TopDecode for ManagedNestedBuffer<M> {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        if I::supports_specialized_type::<Self>() {
            input.into_specialized(h)
        } else {
            Ok(ManagedNestedBuffer::new_from_bytes(
                &input.into_boxed_slice_u8(),
            ))
        }
    }
}

impl<M: ManagedTypeApi> NestedDecode for ManagedNestedBuffer<M> {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        if I::supports_specialized_type::<Self>() {
            input.read_specialized((), h)
        } else {
            let boxed_bytes = BoxedBytes::dep_decode_or_handle_err(input, h)?;
            Ok(Self::new_from_bytes(boxed_bytes.as_slice()))
        }
    }
}

impl<M: ManagedTypeApi> NestedEncode for ManagedNestedBuffer<M> {
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        if O::supports_specialized_type::<Self>() {
            let len_bytes = (self.buffer.len() as u32).to_be_bytes();
            dest.write(&len_bytes[..]);
            dest.push_specialized((), self, h)
        } else {
            self.buffer
                .to_boxed_bytes()
                .dep_encode_or_handle_err(dest, h)
        }
    }
}

impl<M: ManagedTypeApi> NestedDecodeInput for ManagedNestedBuffer<M> {
    fn is_depleted(&self) -> bool {
        self.remaining_len() == 0
    }

    fn supports_specialized_type<T: TryStaticCast>() -> bool {
        T::type_eq::<ManagedBuffer<M>>()
            || T::type_eq::<BigUint<M>>()
            || T::type_eq::<BigInt<M>>()
            || T::type_eq::<ManagedNestedBuffer<M>>()
    }

    fn read_specialized<T, C, H>(&mut self, context: C, h: H) -> Result<T, H::HandledErr>
    where
        T: TryStaticCast,
        C: TryStaticCast,
        H: DecodeErrorHandler,
    {
        if let Some(result) = try_execute_then_cast(|| self.read_nested_managed_buffer(context, h))
        {
            result
        } else {
            Err(h.handle_error(DecodeError::UNSUPPORTED_OPERATION))
        }
    }

    //default
    fn read_byte<H>(&mut self, h: H) -> Result<u8, H::HandledErr>
    where
        H: DecodeErrorHandler,
    {
        let mut buf = [0u8];
        self.read_into(&mut buf[..], h)?;
        Ok(buf[0])
    }

    fn remaining_len(&self) -> usize {
        todo!()
    }

    fn peek_into<H>(&mut self, _into: &mut [u8], _h: H) -> Result<(), H::HandledErr>
    where
        H: DecodeErrorHandler,
    {
        todo!()
    }

    fn read_into<H>(&mut self, _into: &mut [u8], _h: H) -> Result<(), H::HandledErr>
    where
        H: DecodeErrorHandler,
    {
        todo!()
    }
}

impl<M: ManagedTypeApi> NestedEncodeOutput for ManagedNestedBuffer<M> {
    fn push_byte(&mut self, byte: u8) {
        self.write(&[byte]);
    }

    fn supports_specialized_type<T: TryStaticCast>() -> bool {
        T::type_eq::<ManagedBuffer<M>>()
            || T::type_eq::<BigUint<M>>()
            || T::type_eq::<BigInt<M>>()
            || T::type_eq::<ManagedNestedBuffer<M>>()
    }

    fn push_specialized<T, C, H>(
        &mut self,
        _context: C,
        value: &T,
        h: H,
    ) -> Result<(), H::HandledErr>
    where
        T: TryStaticCast,
        C: TryStaticCast,
        H: EncodeErrorHandler,
    {
        if let Some(managed_buffer) = value.try_cast_ref::<ManagedNestedBuffer<M>>() {
            self.buffer.append(&managed_buffer.buffer);
            Ok(())
        } else if let Some(managed_buffer) = value.try_cast_ref::<ManagedBuffer<M>>() {
            self.buffer.append(managed_buffer);
            Ok(())
        } else {
            Err(h.handle_error(EncodeError::UNSUPPORTED_OPERATION))
        }
    }

    fn write(&mut self, bytes: &[u8]) {
        self.buffer.append_bytes(bytes);
    }
}
