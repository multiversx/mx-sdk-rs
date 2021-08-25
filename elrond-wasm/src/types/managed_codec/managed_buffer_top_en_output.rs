use elrond_codec::{TopEncodeOutput, TryStaticCast};

use crate::{
    api::ManagedTypeApi,
    types::{BigInt, BigUint, ManagedBuffer},
};

impl<M: ManagedTypeApi> TopEncodeOutput for &mut ManagedBuffer<M> {
    type NestedBuffer = ManagedBuffer<M>;

    fn set_slice_u8(self, bytes: &[u8]) {
        self.overwrite(bytes);
    }

    #[inline]
    fn set_specialized<T: TryStaticCast, F: FnOnce(Self)>(self, value: &T, else_serialization: F) {
        if let Some(managed_buffer) = value.try_cast_ref::<ManagedBuffer<M>>() {
            *self = managed_buffer.clone();
        } else if let Some(big_uint) = value.try_cast_ref::<BigUint<M>>() {
            *self = big_uint.to_bytes_be_buffer();
        } else if let Some(big_int) = value.try_cast_ref::<BigInt<M>>() {
            *self = big_int.to_signed_bytes_be_buffer();
        } else {
            else_serialization(self);
        }
    }

    fn start_nested_encode(&self) -> Self::NestedBuffer {
        (*self).clone()
    }

    fn finalize_nested_encode(self, nb: Self::NestedBuffer) {
        *self = nb;
    }
}
