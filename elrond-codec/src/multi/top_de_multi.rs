use crate::{DecodeError, DecodeErrorHandler, DefaultErrorHandler, TopDecode, TopDecodeMultiInput};

pub trait TopDecodeMulti: Sized {
    fn multi_decode<I>(input: &mut I) -> Result<Self, DecodeError>
    where
        I: TopDecodeMultiInput,
    {
        Self::multi_decode_or_handle_err(input, DefaultErrorHandler)
    }

    fn multi_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeMultiInput,
        H: DecodeErrorHandler,
    {
        match Self::multi_decode(input) {
            Ok(v) => Ok(v),
            Err(e) => Err(h.handle_error(e)),
        }
    }
}

/// All single top decode types also work as multi-value decode types.
impl<T> TopDecodeMulti for T
where
    T: TopDecode,
{
    fn multi_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeMultiInput,
        H: DecodeErrorHandler,
    {
        input.next_value(h)
    }
}
