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
