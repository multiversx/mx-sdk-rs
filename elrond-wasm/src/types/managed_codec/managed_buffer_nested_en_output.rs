use elrond_codec::{EncodeError, NestedEncodeOutput, TryStaticCast};

use crate::{
    api::ManagedTypeApi,
    types::{BigInt, BigUint, ManagedBuffer},
};

#[inline]
fn push_nested_managed_buffer<M: ManagedTypeApi>(
    accumulator: &mut ManagedBuffer<M>,
    item: &ManagedBuffer<M>,
) {
    accumulator.append_u32_be(item.len() as u32);
    accumulator.append(item);
}

impl<M: ManagedTypeApi> NestedEncodeOutput for ManagedBuffer<M> {
    fn write(&mut self, bytes: &[u8]) {
        self.append_bytes(bytes);
    }

    #[inline]
    fn push_specialized<T, F>(
        &mut self,
        value: &T,
        else_serialization: F,
    ) -> Result<(), EncodeError>
    where
        T: TryStaticCast,
        F: FnOnce(&mut Self) -> Result<(), EncodeError>,
    {
        if let Some(managed_buffer) = value.try_cast_ref::<ManagedBuffer<M>>() {
            push_nested_managed_buffer(self, managed_buffer);
            Ok(())
        } else if let Some(big_uint) = value.try_cast_ref::<BigUint<M>>() {
            push_nested_managed_buffer(self, &big_uint.to_bytes_be_buffer());
            Ok(())
        } else if let Some(big_int) = value.try_cast_ref::<BigInt<M>>() {
            push_nested_managed_buffer(self, &big_int.to_signed_bytes_be_buffer());
            Ok(())
        } else {
            else_serialization(self)
        }
    }
}
