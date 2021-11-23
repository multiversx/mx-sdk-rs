use alloc::boxed::Box;
use elrond_codec::{try_execute_then_cast, DecodeError, TopDecodeInput, TryStaticCast};

use crate::{
    api::ManagedTypeApi,
    types::{BigInt, BigUint, BoxedBytes, ManagedBuffer},
};

use super::ManagedBytesNestedDecodeInput;

pub struct ManagedBytesTopDecodeInput<M: ManagedTypeApi> {
    bytes: BoxedBytes,
    api: M,
}

impl<M: ManagedTypeApi> ManagedBytesTopDecodeInput<M> {
    pub fn new(api: M, bytes: BoxedBytes) -> Self {
        ManagedBytesTopDecodeInput { bytes, api }
    }
}

impl<M> TopDecodeInput for ManagedBytesTopDecodeInput<M>
where
    M: ManagedTypeApi,
{
    type NestedBuffer = ManagedBytesNestedDecodeInput<M>;

    fn byte_len(&self) -> usize {
        self.bytes.len()
    }

    fn into_boxed_slice_u8(self) -> Box<[u8]> {
        self.bytes.into_box()
    }

    fn into_specialized<T, F>(self, else_deser: F) -> Result<T, DecodeError>
    where
        T: TryStaticCast,
        F: FnOnce(Self) -> Result<T, DecodeError>,
    {
        if let Some(result) =
            try_execute_then_cast(|| ManagedBuffer::<M>::new_from_bytes(self.bytes.as_slice()))
        {
            Ok(result)
        } else if let Some(result) =
            try_execute_then_cast(|| BigUint::<M>::from_bytes_be(self.bytes.as_slice()))
        {
            Ok(result)
        } else if let Some(result) =
            try_execute_then_cast(|| BigInt::<M>::from_signed_bytes_be(self.bytes.as_slice()))
        {
            Ok(result)
        } else {
            else_deser(self)
        }
    }

    fn into_nested_buffer(self) -> Self::NestedBuffer {
        ManagedBytesNestedDecodeInput::new(self.api, self.bytes.into_box())
    }
}
