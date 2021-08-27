use elrond_codec::{EncodeError, TopEncodeOutput, TryStaticCast};

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
    fn set_specialized<T, F>(self, value: &T, else_serialization: F) -> Result<(), EncodeError>
    where
        T: TryStaticCast,
        F: FnOnce(Self) -> Result<(), EncodeError>,
    {
        if let Some(managed_buffer) = value.try_cast_ref::<ManagedBuffer<M>>() {
            *self = managed_buffer.clone();
            Ok(())
        } else if let Some(big_uint) = value.try_cast_ref::<BigUint<M>>() {
            *self = big_uint.to_bytes_be_buffer();
            Ok(())
        } else if let Some(big_int) = value.try_cast_ref::<BigInt<M>>() {
            *self = big_int.to_signed_bytes_be_buffer();
            Ok(())
        } else {
            else_serialization(self)
        }
    }

    fn start_nested_encode(&self) -> Self::NestedBuffer {
        (*self).clone()
    }

    fn finalize_nested_encode(self, nb: Self::NestedBuffer) {
        *self = nb;
    }
}
