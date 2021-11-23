use alloc::boxed::Box;
use elrond_codec::{try_execute_then_cast, DecodeError, TopDecodeInput, TryStaticCast};

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

    fn into_u64(self) -> u64 {
        if let Some(num) = self.parse_as_u64() {
            num
        } else {
            M::instance().signal_error(DecodeError::INPUT_TOO_LONG.message_bytes())
        }
    }

    fn into_specialized<T, F>(self, else_deser: F) -> Result<T, DecodeError>
    where
        T: TryStaticCast,
        F: FnOnce(Self) -> Result<T, DecodeError>,
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
            else_deser(self)
        }
    }

    fn into_nested_buffer(self) -> Self::NestedBuffer {
        ManagedBufferNestedDecodeInput::new(self)
    }
}
