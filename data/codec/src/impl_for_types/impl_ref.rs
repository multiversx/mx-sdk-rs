use crate::{
    DecodeErrorHandler, EncodeErrorHandler, NestedDecode, NestedDecodeInput, NestedEncode,
    NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
};
use alloc::boxed::Box;

impl<T: TopEncode> TopEncode for &T {
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        (*self).top_encode_or_handle_err(output, h)
    }
}

impl<T: TopEncode> TopEncode for Box<T> {
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.as_ref().top_encode_or_handle_err(output, h)
    }
}

impl<T: TopDecode> TopDecode for Box<T> {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        T::top_decode_boxed_or_handle_err(input, h)
    }
}

impl<T: NestedEncode> NestedEncode for &T {
    #[inline]
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        (*self).dep_encode_or_handle_err(dest, h)
    }
}

impl<T: NestedEncode> NestedEncode for Box<T> {
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.as_ref().dep_encode_or_handle_err(dest, h)
    }
}

impl<T: NestedDecode> NestedDecode for Box<T> {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(Box::new(T::dep_decode_or_handle_err(input, h)?))
    }
}

#[cfg(test)]
mod tests {
    use alloc::boxed::Box;

    use crate::test_util::{
        check_dep_encode, check_dep_encode_decode, check_top_encode, check_top_encode_decode,
    };

    #[test]
    fn test_top_box_u32() {
        check_top_encode_decode(Box::new(5u32), &[5]);
        check_top_encode_decode(Box::new(0u32), &[]);
    }

    #[test]
    fn test_dep_box_u32() {
        check_dep_encode_decode(Box::new(5u32), &[0, 0, 0, 5]);
        check_dep_encode_decode(Box::new(0u32), &[0, 0, 0, 0]);
    }

    #[test]
    fn test_top_encode_ref() {
        let val = 42u32;
        let bytes = check_top_encode(&&val);
        assert_eq!(bytes, &[42]);
    }

    #[test]
    fn test_dep_encode_ref() {
        let val = 42u32;
        let bytes = check_dep_encode(&&val);
        assert_eq!(bytes, &[0, 0, 0, 42]);
    }
}
