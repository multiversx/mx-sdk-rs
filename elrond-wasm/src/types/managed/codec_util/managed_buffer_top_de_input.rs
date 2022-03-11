use alloc::boxed::Box;
use elrond_codec::{
    try_execute_then_cast, DecodeError, DecodeErrorHandler, TopDecodeInput, TryStaticCast,
};

use crate::{
    api::{ErrorApiImpl, ManagedTypeApi},
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

    fn into_u64(self) -> u64 {
        if let Some(num) = self.parse_as_u64() {
            num
        } else {
            M::error_api_impl().signal_error(DecodeError::INPUT_TOO_LONG.message_bytes())
        }
    }

    fn into_i64(self) -> i64 {
        if let Some(num) = self.parse_as_i64() {
            num
        } else {
            M::error_api_impl().signal_error(DecodeError::INPUT_TOO_LONG.message_bytes())
        }
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
