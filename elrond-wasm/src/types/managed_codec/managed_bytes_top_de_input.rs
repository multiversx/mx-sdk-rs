use alloc::boxed::Box;
use elrond_codec::{TopDecodeInput, TryStaticCast};

use crate::{
    api::ManagedTypeApi,
    types::{BoxedBytes, ManagedBuffer},
};

use super::ManagedBytesNestedDecodeInput;

pub struct ManagedBytesTopDecodeInput<M: ManagedTypeApi> {
    bytes: BoxedBytes,
    api: M,
}

impl<M: ManagedTypeApi> ManagedBytesTopDecodeInput<M> {
    pub fn new(bytes: BoxedBytes, api: M) -> Self {
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

    fn into_specialized<T: TryStaticCast>(self) -> Option<T> {
        if T::type_eq::<ManagedBuffer<M>>() {
            let mb = ManagedBuffer::new_from_bytes(self.api, self.bytes.as_slice());
            mb.try_cast()
        } else {
            None
        }
    }

    fn into_nested_buffer(self) -> Self::NestedBuffer {
        ManagedBytesNestedDecodeInput::new(self.bytes.into_box(), self.api)
    }
}
