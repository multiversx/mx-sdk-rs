use crate::{
    boxed_slice_into_vec,
    codec_err::{DecodeError, EncodeError},
    nested_de::NestedDecode,
    nested_de_input::NestedDecodeInput,
    nested_ser::NestedEncode,
    nested_ser_output::NestedEncodeOutput,
    top_de::TopDecode,
    top_de_input::TopDecodeInput,
    top_ser::TopEncode,
    top_ser_output::TopEncodeOutput,
    TypeInfo,
};
use alloc::vec::Vec;

impl<T: NestedEncode> TopEncode for Vec<T> {
    #[inline]
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        self.as_slice().top_encode(output)
    }

    #[inline]
    fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
        &self,
        output: O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        self.as_slice().top_encode_or_exit(output, c, exit);
    }
}

impl<T: NestedDecode> TopDecode for Vec<T> {
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        if let TypeInfo::U8 = T::TYPE_INFO {
            let bytes = input.into_boxed_slice_u8();
            let bytes_vec = boxed_slice_into_vec(bytes);
            let cast_vec: Vec<T> = unsafe { core::mem::transmute(bytes_vec) };
            Ok(cast_vec)
        } else {
            let mut result: Vec<T> = Vec::new();
            let mut nested_buffer = input.into_nested_buffer();
            while !nested_buffer.is_depleted() {
                result.push(T::dep_decode(&mut nested_buffer)?);
            }
            if !nested_buffer.is_depleted() {
                return Err(DecodeError::INPUT_TOO_LONG);
            }
            Ok(result)
        }
    }

    fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
        input: I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        if let TypeInfo::U8 = T::TYPE_INFO {
            let bytes = input.into_boxed_slice_u8();
            let bytes_vec = boxed_slice_into_vec(bytes);
            let cast_vec: Vec<T> = unsafe { core::mem::transmute(bytes_vec) };
            cast_vec
        } else {
            let mut result: Vec<T> = Vec::new();
            let mut nested_buffer = input.into_nested_buffer();
            while !nested_buffer.is_depleted() {
                result.push(T::dep_decode_or_exit(&mut nested_buffer, c.clone(), exit));
            }
            if !nested_buffer.is_depleted() {
                exit(c, DecodeError::INPUT_TOO_LONG);
            }
            result
        }
    }
}

impl<T: NestedEncode> NestedEncode for Vec<T> {
    #[inline]
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        self.as_slice().dep_encode(dest)
    }

    #[inline]
    fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
        &self,
        dest: &mut O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        self.as_slice().dep_encode_or_exit(dest, c, exit);
    }
}

impl<T: NestedDecode> NestedDecode for Vec<T> {
    fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
        let size = usize::dep_decode(input)?;
        match T::TYPE_INFO {
            TypeInfo::U8 => {
                let mut vec_u8: Vec<u8> = alloc::vec![0; size];
                input.read_into(vec_u8.as_mut_slice())?;
                let cast_vec: Vec<T> = unsafe { core::mem::transmute(vec_u8) };
                Ok(cast_vec)
            },
            _ => {
                let mut result: Vec<T> = Vec::with_capacity(size);
                for _ in 0..size {
                    result.push(T::dep_decode(input)?);
                }
                Ok(result)
            },
        }
    }

    fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
        input: &mut I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        let size = usize::dep_decode_or_exit(input, c.clone(), exit);
        match T::TYPE_INFO {
            TypeInfo::U8 => {
                let mut vec_u8: Vec<u8> = alloc::vec![0; size];
                input.read_into_or_exit(vec_u8.as_mut_slice(), c, exit);
                let cast_vec: Vec<T> = unsafe { core::mem::transmute(vec_u8) };
                cast_vec
            },
            _ => {
                let mut result: Vec<T> = Vec::with_capacity(size);
                for _ in 0..size {
                    result.push(T::dep_decode_or_exit(input, c.clone(), exit));
                }
                result
            },
        }
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
