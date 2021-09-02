use crate::{
    num_conv::top_encode_number_to_output, EncodeError, NestedEncodeOutput, TryStaticCast,
};
use alloc::{boxed::Box, vec::Vec};

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

    #[inline]
    #[allow(clippy::boxed_local)]
    fn set_boxed_bytes(self, bytes: Box<[u8]>) {
        self.set_slice_u8(&*bytes);
    }

    fn set_u64(self, value: u64) {
        let mut buffer = Vec::<u8>::with_capacity(8);
        top_encode_number_to_output(&mut buffer, value, false);
        self.set_slice_u8(&buffer[..]);
    }

    fn set_i64(self, value: i64) {
        let mut buffer = Vec::<u8>::with_capacity(8);
        top_encode_number_to_output(&mut buffer, value as u64, true);
        self.set_slice_u8(&buffer[..]);
    }

    /// The unit type `()` is serializable, but some TopEncodeOutput implementations might want to treat it differently.
    /// For instance, SC function result units do not cause `finish` to be called, no empty result produced.
    #[doc(hidden)]
    #[inline]
    fn set_unit(self) {
        self.set_slice_u8(&[]);
    }

    /// Allows special handling of special types.
    /// Also requires an alternative serialization, in case the special handling is not covered.
    /// The alternative serialization, `else_serialization` is only called when necessary and
    /// is normally compiled out via monomorphization.
    #[inline]
    fn set_specialized<T, F>(self, _value: &T, else_serialization: F) -> Result<(), EncodeError>
    where
        T: TryStaticCast,
        F: FnOnce(Self) -> Result<(), EncodeError>,
    {
        else_serialization(self)
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
