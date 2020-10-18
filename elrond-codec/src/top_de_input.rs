use alloc::boxed::Box;
use crate::num_conv::bytes_to_number;

/// Trait that abstracts away an underlying API for a top-level object deserializer.
/// The underlying API can provide pre-parsed i64/u64 or pre-bundled boxed slices.
pub trait TopDecodeInput: Sized {
    /// Length of the underlying data, in bytes.
    fn byte_len(&self) -> usize;

    /// Retrieves the underlying dat as a byte slice.
    /// 
    /// The mutable self reference is because
    /// some implementations will store the underlying data inside them.
    fn get_slice_u8(&mut self) -> &[u8];

    /// Provides the underlying data as an owned byte slice box,
    /// consuming the input object in the process.
    fn into_boxed_slice_u8(self) -> Box<[u8]>;

    /// Retrieves the underlying data as a pre-parsed u64.
    /// Expected to panic if the conversion is not possible.
    /// 
    /// The mutable self reference is because
    /// some implementations will store the underlying data inside them.
    fn get_u64(&mut self) -> u64 {
        bytes_to_number(self.get_slice_u8(), false)
    }

    /// Retrieves the underlying data as a pre-parsed i64.
    /// Expected to panic if the conversion is not possible.
    /// 
    /// The mutable self reference is because
    /// some implementations will store the underlying data inside them.
    fn get_i64(&mut self) -> i64 {
        bytes_to_number(self.get_slice_u8(), true) as i64
    }
}

impl TopDecodeInput for Box<[u8]> {
    fn byte_len(&self) -> usize {
        self.len()
    }

    fn get_slice_u8(&mut self) -> &[u8] {
        &*self
    }

    fn into_boxed_slice_u8(self) -> Box<[u8]> {
        self
    }
}

impl<'a> TopDecodeInput for &'a [u8] {
    fn byte_len(&self) -> usize {
        self.len()
    }

    fn get_slice_u8(&mut self) -> &[u8] {
        self
    }

    fn into_boxed_slice_u8(self) -> Box<[u8]> {
        Box::from(self)
    }
}
