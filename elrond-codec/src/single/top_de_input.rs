use crate::{
    num_conv::universal_decode_number, transmute::vec_into_boxed_slice, DecodeError,
    DecodeErrorHandler, NestedDecodeInput, OwnedBytesNestedDecodeInput, TryStaticCast,
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

    /// Puts the underlying data into a fixed size byte buffer
    /// and returns the populated data slice from this buffer.
    ///
    /// Will return an error if the data exceeds the provided buffer.
    fn into_max_size_buffer<H, const MAX_LEN: usize>(
        self,
        buffer: &mut [u8; MAX_LEN],
        h: H,
    ) -> Result<&[u8], H::HandledErr>
    where
        H: DecodeErrorHandler;

    /// Retrieves the underlying data as a pre-parsed u64.
    /// Expected to panic if the conversion is not possible.
    ///
    /// Consumes the input object in the process.
    fn into_u64<H>(self, h: H) -> Result<u64, H::HandledErr>
    where
        H: DecodeErrorHandler,
    {
        let mut buffer = [0u8; 8];
        let slice = self.into_max_size_buffer(&mut buffer, h)?;
        Ok(universal_decode_number(slice, false))
    }

    /// Retrieves the underlying data as a pre-parsed i64.
    /// Expected to panic if the conversion is not possible.
    ///
    /// Consumes the input object in the process.
    fn into_i64<H>(self, h: H) -> Result<i64, H::HandledErr>
    where
        H: DecodeErrorHandler,
    {
        let mut buffer = [0u8; 8];
        let slice = self.into_max_size_buffer(&mut buffer, h)?;
        Ok(universal_decode_number(slice, true) as i64)
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

    fn into_max_size_buffer<H, const MAX_LEN: usize>(
        self,
        buffer: &mut [u8; MAX_LEN],
        h: H,
    ) -> Result<&[u8], H::HandledErr>
    where
        H: DecodeErrorHandler,
    {
        (&*self).into_max_size_buffer(buffer, h)
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

    fn into_max_size_buffer<H, const MAX_LEN: usize>(
        self,
        buffer: &mut [u8; MAX_LEN],
        h: H,
    ) -> Result<&[u8], H::HandledErr>
    where
        H: DecodeErrorHandler,
    {
        self.as_slice().into_max_size_buffer(buffer, h)
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

    fn into_boxed_slice_u8(self) -> Box<[u8]> {
        Box::from(self)
    }

    fn into_max_size_buffer<H, const MAX_LEN: usize>(
        self,
        buffer: &mut [u8; MAX_LEN],
        h: H,
    ) -> Result<&[u8], H::HandledErr>
    where
        H: DecodeErrorHandler,
    {
        let l = self.len();
        if l > MAX_LEN {
            return Err(h.handle_error(DecodeError::INPUT_TOO_LONG));
        }
        buffer[..l].copy_from_slice(self);
        Ok(&buffer[..l])
    }

    #[inline]
    fn into_u64<H>(self, _h: H) -> Result<u64, H::HandledErr>
    where
        H: DecodeErrorHandler,
    {
        Ok(universal_decode_number(self, false))
    }

    #[inline]
    fn into_i64<H>(self, _h: H) -> Result<i64, H::HandledErr>
    where
        H: DecodeErrorHandler,
    {
        Ok(universal_decode_number(self, true) as i64)
    }

    fn into_nested_buffer(self) -> Self::NestedBuffer {
        self
    }
}
