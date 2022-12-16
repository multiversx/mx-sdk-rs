use crate::codec::{
    try_execute_then_cast, DecodeError, DecodeErrorHandler, TopDecodeInput, TryStaticCast,
};
use alloc::boxed::Box;

use crate::{
    api::ManagedTypeApi,
    types::{BigInt, BigUint, ManagedBuffer},
};

use super::ManagedBufferNestedDecodeInput;

impl<M> TopDecodeInput for ManagedBuffer<M>
where
    M: ManagedTypeApi,
{
    type NestedBuffer = ManagedBufferNestedDecodeInput<M>;

    fn byte_len(&self) -> usize {
        self.len()
    }

    fn into_boxed_slice_u8(self) -> Box<[u8]> {
        self.to_boxed_bytes().into_box()
    }

    fn into_max_size_buffer<H, const MAX_LEN: usize>(
        self,
        buffer: &mut [u8; MAX_LEN],
        h: H,
    ) -> Result<&[u8], H::HandledErr>
    where
        H: DecodeErrorHandler,
    {
        let len = self.len();
        if len > MAX_LEN {
            return Err(h.handle_error(DecodeError::INPUT_TOO_LONG));
        }
        let byte_slice = &mut buffer[..len];
        let _ = self.load_slice(0, byte_slice);
        Ok(byte_slice)
    }

    #[inline]
    fn supports_specialized_type<T: TryStaticCast>() -> bool {
        T::type_eq::<ManagedBuffer<M>>() || T::type_eq::<BigUint<M>>() || T::type_eq::<BigInt<M>>()
    }

    #[inline]
    fn into_specialized<T, H>(self, h: H) -> Result<T, H::HandledErr>
    where
        T: TryStaticCast,
        H: DecodeErrorHandler,
    {
        if let Some(result) = try_execute_then_cast(|| self.clone()) {
            Ok(result)
        } else if let Some(result) = try_execute_then_cast(|| BigUint::from_bytes_be_buffer(&self))
        {
            Ok(result)
        } else if let Some(result) =
            try_execute_then_cast(|| BigInt::from_signed_bytes_be_buffer(&self))
        {
            Ok(result)
        } else {
            Err(h.handle_error(DecodeError::UNSUPPORTED_OPERATION))
        }
    }

    fn into_nested_buffer(self) -> Self::NestedBuffer {
        ManagedBufferNestedDecodeInput::new(self)
    }
}
