use crate::{
    num_conv::bytes_to_number, transmute::vec_into_boxed_slice, DecodeError, DecodeErrorHandler,
    NestedDecodeInput, OwnedBytesNestedDecodeInput, TryStaticCast,
};
use alloc::{boxed::Box, vec::Vec};

/// Trait that abstracts away an underlying API for a top-level object deserializer.
/// The underlying API can provide pre-parsed i64/u64 or pre-bundled boxed slices.
pub trait TopDecodeInput: Sized {
    type NestedBuffer: NestedDecodeInput;

    /// Length of the underlying data, in bytes.
    fn byte_len(&self) -> usize;

    /// Provides the underlying data as an owned byte slice box.
    /// Consumes the input object in the process.
    fn into_boxed_slice_u8(self) -> Box<[u8]>;

    /// Retrieves the underlying data as a pre-parsed u64.
    /// Expected to panic if the conversion is not possible.
    ///
    /// Consumes the input object in the process.
    fn into_u64(self) -> u64 {
        bytes_to_number(&*self.into_boxed_slice_u8(), false)
    }

    /// Retrieves the underlying data as a pre-parsed i64.
    /// Expected to panic if the conversion is not possible.
    ///
    /// Consumes the input object in the process.
    fn into_i64(self) -> i64 {
        bytes_to_number(&*self.into_boxed_slice_u8(), true) as i64
    }

    #[inline]
    fn supports_specialized_type<T: TryStaticCast>() -> bool {
        false
    }

    fn into_specialized<T, H>(self, h: H) -> Result<T, H::HandledErr>
    where
        T: TryStaticCast,
        H: DecodeErrorHandler,
    {
        Err(h.handle_error(DecodeError::UNSUPPORTED_OPERATION))
    }

    fn into_nested_buffer(self) -> Self::NestedBuffer;
}

impl TopDecodeInput for Box<[u8]> {
    type NestedBuffer = OwnedBytesNestedDecodeInput;

    fn byte_len(&self) -> usize {
        self.len()
    }

    fn into_boxed_slice_u8(self) -> Box<[u8]> {
        self
    }

    fn into_nested_buffer(self) -> Self::NestedBuffer {
        OwnedBytesNestedDecodeInput::new(self)
    }
}

impl TopDecodeInput for Vec<u8> {
    type NestedBuffer = OwnedBytesNestedDecodeInput;

    fn byte_len(&self) -> usize {
        self.len()
    }

    fn into_boxed_slice_u8(self) -> Box<[u8]> {
        vec_into_boxed_slice(self)
    }

    fn into_nested_buffer(self) -> Self::NestedBuffer {
        OwnedBytesNestedDecodeInput::new(self.into_boxed_slice())
    }
}

impl<'a> TopDecodeInput for &'a [u8] {
    type NestedBuffer = &'a [u8];

    fn byte_len(&self) -> usize {
        self.len()
    }

    fn into_u64(self) -> u64 {
        bytes_to_number(self, false)
    }

    fn into_i64(self) -> i64 {
        bytes_to_number(self, true) as i64
    }

    fn into_boxed_slice_u8(self) -> Box<[u8]> {
        Box::from(self)
    }

    fn into_nested_buffer(self) -> Self::NestedBuffer {
        self
    }
}
