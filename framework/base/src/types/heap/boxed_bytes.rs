use crate::{
    abi::{TypeAbi, TypeName},
    codec::*,
};
use alloc::{
    alloc::{alloc, alloc_zeroed, realloc, Layout},
    boxed::Box,
    vec::Vec,
};

/// Simple wrapper around a boxed byte slice,
/// but with a lot of optimized methods for manipulating it.
/// The focus is on reducing code size rather improving speed.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BoxedBytes(Box<[u8]>);

impl BoxedBytes {
    pub fn empty() -> Self {
        BoxedBytes(Box::from([0u8; 0]))
    }

    pub fn zeros(len: usize) -> Self {
        unsafe {
            let layout = Layout::from_size_align(len, core::mem::align_of::<u8>()).unwrap();
            let bytes_ptr = alloc_zeroed(layout);
            let bytes_box = Box::from_raw(core::slice::from_raw_parts_mut(bytes_ptr, len));
            BoxedBytes(bytes_box)
        }
    }

    /// Allocates an uninitialized BoxedBytes to heap.
    ///
    /// # Safety
    ///
    /// Should only be called if the contents are initialized immediately afterwards, e.g. via a FFI call.
    pub unsafe fn allocate(len: usize) -> Self {
        let layout = Layout::from_size_align(len, core::mem::align_of::<u8>()).unwrap();
        let bytes_ptr = alloc(layout);
        let bytes_box = Box::from_raw(core::slice::from_raw_parts_mut(bytes_ptr, len));
        BoxedBytes(bytes_box)
    }

    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.0.as_mut_ptr()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub fn into_box(self) -> Box<[u8]> {
        self.0
    }

    #[inline]
    pub fn into_vec(self) -> Vec<u8> {
        self.0.into_vec()
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.0
    }

    /// Create new instance by concatenating several byte slices.
    pub fn from_concat(slices: &[&[u8]]) -> Self {
        let mut result_len = 0usize;
        let mut index = slices.len();
        while index > 0 {
            index -= 1;
            result_len += slices[index].len();
        }
        unsafe {
            let layout = Layout::from_size_align(result_len, core::mem::align_of::<u8>()).unwrap();
            let result_ptr = alloc(layout);
            let mut current_index = 0usize;
            for slice in slices.iter() {
                core::ptr::copy_nonoverlapping(
                    slice.as_ptr(),
                    result_ptr.add(current_index),
                    slice.len(),
                );
                current_index += slice.len();
            }
            let bytes_box = Box::from_raw(core::slice::from_raw_parts_mut(result_ptr, result_len));
            BoxedBytes(bytes_box)
        }
    }

    /// Splits BoxedBytes into 2 others at designated position.
    /// Returns the original and an empty BoxedBytes if position arugment out of range.
    pub fn split(self, at: usize) -> (BoxedBytes, BoxedBytes) {
        if at >= self.len() {
            (self, BoxedBytes::empty())
        } else {
            let other_len = self.len() - at;
            unsafe {
                // breaking down the input into its components
                let self_layout =
                    Layout::from_size_align(self.len(), core::mem::align_of::<u8>()).unwrap();
                let self_ptr = Box::into_raw(self.0) as *mut u8;

                // the data for the second result needs to be copied somewhere else
                let other_layout =
                    Layout::from_size_align(other_len, core::mem::align_of::<u8>()).unwrap();
                let other_ptr = alloc(other_layout);
                core::ptr::copy_nonoverlapping(self_ptr.add(at), other_ptr, other_len);

                // truncating the memory for the first using a realloc
                // got inspiration for this from the RawVec implementation
                let realloc_ptr = realloc(self_ptr, self_layout, at);

                // packaging the resulting parts nicely
                let bytes_box_1 = Box::from_raw(core::slice::from_raw_parts_mut(realloc_ptr, at));
                let bytes_box_2 =
                    Box::from_raw(core::slice::from_raw_parts_mut(other_ptr, other_len));
                (BoxedBytes(bytes_box_1), BoxedBytes(bytes_box_2))
            }
        }
    }
}

impl AsRef<[u8]> for BoxedBytes {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<'a> From<&'a [u8]> for BoxedBytes {
    #[inline]
    fn from(byte_slice: &'a [u8]) -> Self {
        BoxedBytes(Box::from(byte_slice))
    }
}

impl From<Box<[u8]>> for BoxedBytes {
    #[inline]
    fn from(b: Box<[u8]>) -> Self {
        BoxedBytes(b)
    }
}

impl From<Vec<u8>> for BoxedBytes {
    #[inline]
    fn from(v: Vec<u8>) -> Self {
        BoxedBytes(v.into_boxed_slice())
    }
}

impl From<&Vec<u8>> for BoxedBytes {
    #[inline]
    fn from(v: &Vec<u8>) -> Self {
        BoxedBytes::from(v.as_slice())
    }
}

/// This allows us to use a mutable BoxedBytes as top encode output.
impl TopEncodeOutput for &mut BoxedBytes {
    type NestedBuffer = Vec<u8>;

    fn set_slice_u8(self, bytes: &[u8]) {
        *self = BoxedBytes::from(bytes);
    }

    fn start_nested_encode(&self) -> Self::NestedBuffer {
        Vec::<u8>::new()
    }

    fn finalize_nested_encode(self, nb: Self::NestedBuffer) {
        self.set_slice_u8(nb.as_slice());
    }
}

impl NestedEncode for BoxedBytes {
    #[inline]
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.len().dep_encode_or_handle_err(dest, h)?;
        dest.write(self.as_ref());
        Ok(())
    }
}

impl TopEncode for BoxedBytes {
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, _h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        output.set_slice_u8(self.as_ref());
        Ok(())
    }
}

impl NestedDecode for BoxedBytes {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        let size = usize::dep_decode_or_handle_err(input, h)?;
        unsafe {
            let mut result = BoxedBytes::allocate(size);
            input.read_into(result.as_mut_slice(), h)?;
            Ok(result)
        }
    }
}

impl TopDecode for BoxedBytes {
    fn top_decode_or_handle_err<I, H>(input: I, _h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(BoxedBytes(input.into_boxed_slice_u8()))
    }
}

impl TypeAbi for BoxedBytes {
    fn type_name() -> TypeName {
        "bytes".into()
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concat_1() {
        let bb = BoxedBytes::from_concat(&[&b"abc"[..], &b"def"[..]]);
        assert_eq!(bb, BoxedBytes::from(&b"abcdef"[..]));
    }

    #[test]
    fn test_concat_2() {
        let bb = BoxedBytes::from_concat(&[&b"abc"[..], &b""[..], &b"def"[..]]);
        assert_eq!(bb, BoxedBytes::from(&b"abcdef"[..]));
    }

    #[test]
    fn test_concat_empty_1() {
        let bb = BoxedBytes::from_concat(&[&b""[..], &b""[..], &b""[..]]);
        assert_eq!(bb, BoxedBytes::from(&b""[..]));
    }

    #[test]
    fn test_concat_empty_2() {
        let bb = BoxedBytes::from_concat(&[]);
        assert_eq!(bb, BoxedBytes::from(&b""[..]));
    }

    #[test]
    fn test_is_empty() {
        assert!(BoxedBytes::empty().is_empty());
    }

    #[test]
    fn test_size_of() {
        use core::mem::size_of;
        assert_eq!(size_of::<BoxedBytes>(), 2 * size_of::<usize>());
        assert_eq!(size_of::<Option<BoxedBytes>>(), 2 * size_of::<usize>());
    }

    #[test]
    fn test_split_1() {
        let (bb1, bb2) = BoxedBytes::from(&b"abcdef"[..]).split(3);
        assert_eq!(bb1, BoxedBytes::from(&b"abc"[..]));
        assert_eq!(bb2, BoxedBytes::from(&b"def"[..]));
    }

    #[test]
    fn test_split_2() {
        let (bb1, bb2) = BoxedBytes::from(&b"abcdef"[..]).split(0);
        assert_eq!(bb1, BoxedBytes::from(&b""[..]));
        assert_eq!(bb2, BoxedBytes::from(&b"abcdef"[..]));
    }

    #[test]
    fn test_split_over() {
        let (bb1, bb2) = BoxedBytes::from(&b"abcdef"[..]).split(6);
        assert_eq!(bb1, BoxedBytes::from(&b"abcdef"[..]));
        assert_eq!(bb2, BoxedBytes::from(&b""[..]));

        let (bb1, bb2) = BoxedBytes::from(&b"abcdef"[..]).split(7);
        assert_eq!(bb1, BoxedBytes::from(&b"abcdef"[..]));
        assert_eq!(bb2, BoxedBytes::from(&b""[..]));
    }
}
