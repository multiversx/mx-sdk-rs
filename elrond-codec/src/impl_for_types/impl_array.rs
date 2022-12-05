use crate::{
    top_decode_from_nested_or_handle_err, DecodeError, DecodeErrorHandler, EncodeErrorHandler,
    NestedDecode, NestedDecodeInput, NestedEncode, NestedEncodeOutput, TopDecode, TopDecodeInput,
    TopEncode, TopEncodeOutput,
};
use alloc::boxed::Box;
use arrayvec::ArrayVec;

impl<T: NestedEncode, const N: usize> NestedEncode for [T; N] {
    #[inline]
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        super::impl_slice::dep_encode_slice_contents(&self[..], dest, h)
    }
}

impl<T: NestedEncode, const N: usize> TopEncode for [T; N] {
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        // the top encoded slice does not serialize its length, so just like the array
        (&self[..]).top_encode_or_handle_err(output, h)
    }
}

impl<T: NestedDecode, const N: usize> NestedDecode for [T; N] {
    #[allow(clippy::reversed_empty_ranges)]
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        let mut r = ArrayVec::new();
        for _ in 0..N {
            r.push(T::dep_decode_or_handle_err(input, h)?);
        }
        let i = r.into_inner();

        match i {
            Ok(a) => Ok(a),
            Err(_) => Err(h.handle_error(DecodeError::ARRAY_DECODE_ERROR)),
        }
    }
}

impl<T: NestedDecode, const N: usize> TopDecode for [T; N] {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        top_decode_from_nested_or_handle_err(input, h)
    }

    fn top_decode_boxed_or_handle_err<I, H>(input: I, h: H) -> Result<Box<Self>, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        T::if_u8(
            input,
            |input| {
                // transmute directly
                let bs = input.into_boxed_slice_u8();
                if bs.len() != N {
                    return Err(h.handle_error(DecodeError::ARRAY_DECODE_ERROR));
                }
                let raw = Box::into_raw(bs);
                let array_box = unsafe { Box::<[T; N]>::from_raw(raw as *mut [T; N]) };
                Ok(array_box)
            },
            |input| Ok(Box::new(Self::top_decode_or_handle_err(input, h)?)),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::test_util::check_top_encode;

    use super::*;
    use alloc::vec::Vec;

    #[test]
    fn test_array_16384() {
        let arr = [7i32; 16384];
        let mut expected_bytes = Vec::<u8>::with_capacity(16384 * 4);
        for _ in 0..16384 {
            expected_bytes.push(0);
            expected_bytes.push(0);
            expected_bytes.push(0);
            expected_bytes.push(7);
        }

        // serialize
        let serialized_bytes = check_top_encode(&arr);
        assert_eq!(serialized_bytes, expected_bytes);

        // deserialize
        let deserialized = <[i32; 16384]>::top_decode(&serialized_bytes[..]).unwrap();
        for byte in deserialized {
            assert_eq!(byte, 7i32);
        }
    }
}
