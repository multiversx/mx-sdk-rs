use alloc::boxed::Box;
use elrond_codec::{TopEncodeOutput, TryStaticCast};

use crate::api::ManagedTypeApi;

use super::ManagedBuffer;

impl<M: ManagedTypeApi> TopEncodeOutput for &mut ManagedBuffer<M> {
    type NestedBuffer = ManagedBuffer<M>;

    fn set_slice_u8(self, bytes: &[u8]) {
        self.overwrite(bytes);
    }

    #[inline]
    fn set_specialized<T: TryStaticCast, F: FnOnce() -> Box<[u8]>>(self, value: &T, else_bytes: F) {
        if let Some(managed_buffer) = value.try_cast_ref::<ManagedBuffer<M>>() {
            *self = managed_buffer.clone();
        } else {
            self.set_boxed_bytes(else_bytes());
        }
    }

    fn start_nested_encode(&self) -> Self::NestedBuffer {
        (*self).clone()
    }

    fn finalize_nested_encode(self, nb: Self::NestedBuffer) {
        *self = nb;
    }
}
