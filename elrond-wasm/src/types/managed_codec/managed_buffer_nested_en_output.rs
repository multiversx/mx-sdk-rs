use elrond_codec::{EncodeError, NestedEncodeOutput, TryStaticCast};

use crate::{
    api::ManagedTypeApi,
    types::{BigInt, BigUint, ManagedBuffer, ManagedBufferSizeContext},
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
    fn push_specialized<T, C, F>(
        &mut self,
        context: C,
        value: &T,
        else_serialization: F,
    ) -> Result<(), EncodeError>
    where
        T: TryStaticCast,
        C: TryStaticCast,
        F: FnOnce(&mut Self) -> Result<(), EncodeError>,
    {
        if let Some(managed_buffer) = value.try_cast_ref::<ManagedBuffer<M>>() {
            if context.try_cast_ref::<ManagedBufferSizeContext>().is_some() {
                // managed buffers originating from fixed-length types don't need to serialize the length
                self.append(managed_buffer);
            } else {
                push_nested_managed_buffer(self, managed_buffer);
            }
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
