use crate::codec::{EncodeError, EncodeErrorHandler, TopEncodeOutput, TryStaticCast};

use crate::{
    api::ManagedTypeApi,
    types::{BigInt, BigUint, ManagedBuffer},
};

impl<'a, M: ManagedTypeApi<'a>> TopEncodeOutput for &mut ManagedBuffer<'a, M> {
    type NestedBuffer = ManagedBuffer<'a, M>;

    fn set_slice_u8(self, bytes: &[u8]) {
        self.overwrite(bytes);
    }

    #[inline]
    fn supports_specialized_type<T: TryStaticCast>() -> bool {
        T::type_eq::<ManagedBuffer<'a, M>>() || T::type_eq::<BigUint<'a, M>>() || T::type_eq::<BigInt<'a, M>>()
    }

    #[inline]
    fn set_specialized<T, H>(self, value: &T, h: H) -> Result<(), H::HandledErr>
    where
        T: TryStaticCast,
        H: EncodeErrorHandler,
    {
        if let Some(managed_buffer) = value.try_cast_ref::<ManagedBuffer<'a, M>>() {
            *self = managed_buffer.clone();
            Ok(())
        } else if let Some(big_uint) = value.try_cast_ref::<BigUint<'a, M>>() {
            *self = big_uint.to_bytes_be_buffer();
            Ok(())
        } else if let Some(big_int) = value.try_cast_ref::<BigInt<'a, M>>() {
            *self = big_int.to_signed_bytes_be_buffer();
            Ok(())
        } else {
            Err(h.handle_error(EncodeError::UNSUPPORTED_OPERATION))
        }
    }

    fn start_nested_encode(&self) -> Self::NestedBuffer {
        (*self).clone()
    }

    fn finalize_nested_encode(self, nb: Self::NestedBuffer) {
        *self = nb;
    }
}
