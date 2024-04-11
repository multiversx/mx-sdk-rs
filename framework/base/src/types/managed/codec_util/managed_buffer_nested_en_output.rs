use crate::codec::{EncodeError, EncodeErrorHandler, NestedEncodeOutput, TryStaticCast};

use crate::{
    api::ManagedTypeApi,
    types::{BigInt, BigUint, ManagedBuffer},
};

impl<'a, M: ManagedTypeApi<'a>> NestedEncodeOutput for ManagedBuffer<'a, M> {
    fn write(&mut self, bytes: &[u8]) {
        self.append_bytes(bytes);
    }

    #[inline]
    fn supports_specialized_type<T: TryStaticCast>() -> bool {
        T::type_eq::<ManagedBuffer<'a, M>>() || T::type_eq::<BigUint<'a, M>>() || T::type_eq::<BigInt<'a, M>>()
    }

    #[inline]
    fn push_specialized<T, C, H>(
        &mut self,
        _context: C,
        value: &T,
        h: H,
    ) -> Result<(), H::HandledErr>
    where
        T: TryStaticCast,
        C: TryStaticCast,
        H: EncodeErrorHandler,
    {
        if let Some(managed_buffer) = value.try_cast_ref::<ManagedBuffer<'a, M>>() {
            self.append(managed_buffer);
            Ok(())
        } else {
            Err(h.handle_error(EncodeError::UNSUPPORTED_OPERATION))
        }
    }
}
