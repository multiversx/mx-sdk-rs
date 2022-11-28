use crate::{
    DecodeError, DecodeErrorHandler, EncodeErrorHandler, NestedDecode, NestedDecodeInput,
    NestedEncode, NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
};
use arrayvec::ArrayVec;

impl<T: NestedEncode, const CAP: usize> TopEncode for ArrayVec<T, CAP> {
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.as_slice().top_encode_or_handle_err(output, h)
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
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        let mut result: ArrayVec<T, CAP> = ArrayVec::new();
        let mut nested_buffer = input.into_nested_buffer();
        while !nested_buffer.is_depleted() {
            if let Err(capacity_error) =
                result.try_push(T::dep_decode_or_handle_err(&mut nested_buffer, h)?)
            {
                return Err(h.handle_error(DecodeError::from(capacity_error)));
            }
        }
        if !nested_buffer.is_depleted() {
            return Err(h.handle_error(DecodeError::INPUT_TOO_LONG));
        }
        Ok(result)
    }
}

impl<T: NestedEncode, const CAP: usize> NestedEncode for ArrayVec<T, CAP> {
    #[inline]
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.as_slice().dep_encode_or_handle_err(dest, h)
    }
}

impl<T: NestedDecode, const CAP: usize> NestedDecode for ArrayVec<T, CAP> {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        let size = usize::dep_decode_or_handle_err(input, h)?;
        if size > CAP {
            return Err(h.handle_error(DecodeError::CAPACITY_EXCEEDED_ERROR));
        }
        let mut result: ArrayVec<T, CAP> = ArrayVec::new();
        for _ in 0..size {
            unsafe {
                result.push_unchecked(T::dep_decode_or_handle_err(input, h)?);
            }
        }
        Ok(result)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{test_util::check_top_encode_decode, DecodeError, PanicErrorHandler, TopDecode};
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
        let _ = <ArrayVec<i32, 2>>::top_decode_or_handle_err(TOP_BYTES, PanicErrorHandler);
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
        let _ =
            Option::<ArrayVec<i32, 2>>::top_decode_or_handle_err(NESTED_BYTES, PanicErrorHandler);
    }
}
