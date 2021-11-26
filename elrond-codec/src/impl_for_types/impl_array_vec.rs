use crate::{
    codec_err::{DecodeError, EncodeError},
    nested_de::NestedDecode,
    nested_de_input::NestedDecodeInput,
    nested_ser::NestedEncode,
    nested_ser_output::NestedEncodeOutput,
    top_de::TopDecode,
    top_de_input::TopDecodeInput,
    top_ser::TopEncode,
    top_ser_output::TopEncodeOutput,
};
use arrayvec::ArrayVec;

impl<T: NestedEncode, const CAP: usize> TopEncode for ArrayVec<T, CAP> {
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

/// Allows us to use `?` from the `try_push` to return our `DecodeError`.
impl<T> From<arrayvec::CapacityError<T>> for DecodeError {
    #[inline]
    fn from(_: arrayvec::CapacityError<T>) -> Self {
        DecodeError::CAPACITY_EXCEEDED_ERROR
    }
}

impl<T: NestedDecode, const CAP: usize> TopDecode for ArrayVec<T, CAP> {
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        let mut result: ArrayVec<T, CAP> = ArrayVec::new();
        let mut nested_buffer = input.into_nested_buffer();
        while !nested_buffer.is_depleted() {
            result.try_push(T::dep_decode(&mut nested_buffer)?)?;
        }
        if !nested_buffer.is_depleted() {
            return Err(DecodeError::INPUT_TOO_LONG);
        }
        Ok(result)
    }

    fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
        input: I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        let mut result: ArrayVec<T, CAP> = ArrayVec::new();
        let mut nested_buffer = input.into_nested_buffer();
        while !nested_buffer.is_depleted() {
            let elem = T::dep_decode_or_exit(&mut nested_buffer, c.clone(), exit);
            if result.try_push(elem).is_err() {
                exit(c, DecodeError::CAPACITY_EXCEEDED_ERROR)
            }
        }
        if !nested_buffer.is_depleted() {
            exit(c, DecodeError::INPUT_TOO_LONG);
        }
        result
    }
}

impl<T: NestedEncode, const CAP: usize> NestedEncode for ArrayVec<T, CAP> {
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

impl<T: NestedDecode, const CAP: usize> NestedDecode for ArrayVec<T, CAP> {
    fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
        let size = usize::dep_decode(input)?;
        if size > CAP {
            return Err(DecodeError::CAPACITY_EXCEEDED_ERROR);
        }
        let mut result: ArrayVec<T, CAP> = ArrayVec::new();
        for _ in 0..size {
            unsafe {
                result.push_unchecked(T::dep_decode(input)?);
            }
        }
        Ok(result)
    }

    fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
        input: &mut I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        let size = usize::dep_decode_or_exit(input, c.clone(), exit);
        if size > CAP {
            exit(c, DecodeError::CAPACITY_EXCEEDED_ERROR);
        }
        let mut result: ArrayVec<T, CAP> = ArrayVec::new();
        for _ in 0..size {
            unsafe {
                result.push_unchecked(T::dep_decode_or_exit(input, c.clone(), exit));
            }
        }
        result
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{
        test_util::{check_top_encode_decode, top_decode_from_byte_slice_or_panic},
        DecodeError, TopDecode,
    };
    use arrayvec::ArrayVec;

    /// [1, 2, 3]
    const TOP_BYTES: &[u8] = &[0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3];

    /// Some([1, 2, 3])
    const NESTED_BYTES: &[u8] = &[1, 0, 0, 0, 3, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3];

    #[test]
    fn test_top_arrayvec_i32() {
        let v = ArrayVec::from([1i32, 2i32, 3i32]);
        check_top_encode_decode(v, TOP_BYTES);
    }

    #[test]
    fn test_top_arrayvec_capacity_exceeded() {
        let result = ArrayVec::<i32, 2>::top_decode(TOP_BYTES);
        assert_eq!(result, Err(DecodeError::CAPACITY_EXCEEDED_ERROR));
    }

    #[test]
    #[should_panic]
    fn test_top_arrayvec_capacity_exceeded_panic() {
        let _ = top_decode_from_byte_slice_or_panic::<ArrayVec<i32, 2>>(TOP_BYTES);
    }

    #[test]
    fn test_nested_arrayvec_i32() {
        let v = Some(ArrayVec::from([1i32, 2i32, 3i32]));
        check_top_encode_decode(v, NESTED_BYTES);
    }

    #[test]
    fn test_nested_arrayvec_capacity_exceeded() {
        let result = Option::<ArrayVec<i32, 2>>::top_decode(NESTED_BYTES);
        assert_eq!(result, Err(DecodeError::CAPACITY_EXCEEDED_ERROR));
    }

    #[test]
    #[should_panic]
    fn test_nested_arrayvec_capacity_exceeded_panic() {
        let _ = top_decode_from_byte_slice_or_panic::<Option<ArrayVec<i32, 2>>>(NESTED_BYTES);
    }
}
