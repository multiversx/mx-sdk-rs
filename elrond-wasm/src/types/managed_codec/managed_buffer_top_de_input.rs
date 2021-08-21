use alloc::boxed::Box;
use elrond_codec::{TopDecodeInput, TryStaticCast};

use crate::{api::ManagedTypeApi, types::ManagedBuffer};

use super::ManagedBufferNestedDecodeInput;

impl<M> TopDecodeInput for &ManagedBuffer<M>
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

    fn into_specialized<T: TryStaticCast>(self) -> Option<T> {
        if T::type_eq::<ManagedBuffer<M>>() {
            // TODO: try to get rid of the clone
            self.clone().try_cast()
        } else {
            None
        }
    }

    fn into_nested_buffer(self) -> Self::NestedBuffer {
        // TODO: get rid of the clone, by making ManagedBufferNestedDecodeInput only take a reference
        ManagedBufferNestedDecodeInput::new(self.clone())
    }
}
