use crate::{
    boxed_slice_into_vec, DecodeError, DecodeErrorHandler, EncodeErrorHandler, NestedDecode,
    NestedDecodeInput, NestedEncode, NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode,
    TopEncodeOutput,
};
use alloc::vec::Vec;

impl<T: NestedEncode> TopEncode for Vec<T> {
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.as_slice().top_encode_or_handle_err(output, h)
    }
}

impl<T: NestedDecode> TopDecode for Vec<T> {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        T::if_u8(
            input,
            |input| {
                let bytes = input.into_boxed_slice_u8();
                let bytes_vec = boxed_slice_into_vec(bytes);
                let cast_vec: Vec<T> = unsafe { core::mem::transmute(bytes_vec) };
                Ok(cast_vec)
            },
            |input| {
                let mut result: Vec<T> = Vec::new();
                let mut nested_buffer = input.into_nested_buffer();
                while !nested_buffer.is_depleted() {
                    result.push(T::dep_decode_or_handle_err(&mut nested_buffer, h)?);
                }
                if !nested_buffer.is_depleted() {
                    return Err(h.handle_error(DecodeError::INPUT_TOO_LONG));
                }
                Ok(result)
            },
        )
    }
}

impl<T: NestedEncode> NestedEncode for Vec<T> {
    #[inline]
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.as_slice().dep_encode_or_handle_err(dest, h)
    }
}

impl<T: NestedDecode> NestedDecode for Vec<T> {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        let size = usize::dep_decode_or_handle_err(input, h)?;
        T::if_u8(
            input,
            |input| {
                let mut vec_u8: Vec<u8> = alloc::vec![0; size];
                input.read_into(vec_u8.as_mut_slice(), h)?;
                let cast_vec: Vec<T> = unsafe { core::mem::transmute(vec_u8) };
                Ok(cast_vec)
            },
            |input| {
                let mut result: Vec<T> = Vec::with_capacity(size);
                for _ in 0..size {
                    result.push(T::dep_decode_or_handle_err(input, h)?);
                }
                Ok(result)
            },
        )
    }
}

#[cfg(test)]
pub mod tests {
    use crate::test_util::check_top_encode_decode;

    #[test]
    fn test_top_vec_i32_compacted() {
        let v = [1i32, 2i32, 3i32].to_vec();
        let expected: &[u8] = &[0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3];
        check_top_encode_decode(v, expected);
    }

    #[test]
    fn test_top_vec_u8_compacted() {
        check_top_encode_decode([1u8, 2u8, 3u8].to_vec(), &[1u8, 2u8, 3u8]);
    }
}
