use crate::{
    num_conv::top_encode_number, EncodeError, EncodeErrorHandler, NestedEncodeOutput, TryStaticCast,
};
use alloc::vec::Vec;

/// Specifies objects that can receive the result of a TopEncode computation.

/// in principle from NestedEncode performed on nested items.
///
/// All methods consume the object, so they can only be called once.
///
/// The trait is used in 3 scenarios:
/// - SC results
/// - `#[storage_set(...)]`
/// - Serialize async call.
pub trait TopEncodeOutput: Sized {
    /// Type of `NestedEncodeOutput` that can be spawned to gather serializations of children.
    type NestedBuffer: NestedEncodeOutput;

    fn set_slice_u8(self, bytes: &[u8]);

    fn set_u64(self, value: u64) {
        let mut buffer = [0u8; 8];
        let slice = top_encode_number(value, false, &mut buffer);
        self.set_slice_u8(slice);
    }

    fn set_i64(self, value: i64) {
        let mut buffer = [0u8; 8];
        let slice = top_encode_number(value as u64, true, &mut buffer);
        self.set_slice_u8(slice);
    }

    /// The unit type `()` is serializable, but some TopEncodeOutput implementations might want to treat it differently.
    /// For instance, SC function result units do not cause `finish` to be called, no empty result produced.
    #[doc(hidden)]
    #[inline]
    fn set_unit(self) {
        self.set_slice_u8(&[]);
    }

    #[inline]
    fn supports_specialized_type<T: TryStaticCast>() -> bool {
        false
    }

    /// Allows special handling of special types.
    /// Also requires an alternative serialization, in case the special handling is not covered.
    /// The alternative serialization, `else_serialization` is only called when necessary and
    /// is normally compiled out via monomorphization.
    #[inline]
    fn set_specialized<T, H>(self, _value: &T, h: H) -> Result<(), H::HandledErr>
    where
        T: TryStaticCast,
        H: EncodeErrorHandler,
    {
        Err(h.handle_error(EncodeError::UNSUPPORTED_OPERATION))
    }

    fn start_nested_encode(&self) -> Self::NestedBuffer;

    fn finalize_nested_encode(self, nb: Self::NestedBuffer);
}

impl TopEncodeOutput for &mut Vec<u8> {
    type NestedBuffer = Vec<u8>;

    fn set_slice_u8(self, bytes: &[u8]) {
        self.extend_from_slice(bytes);
    }

    fn start_nested_encode(&self) -> Self::NestedBuffer {
        Vec::<u8>::new()
    }

    fn finalize_nested_encode(self, nb: Self::NestedBuffer) {
        *self = nb;
    }
}
