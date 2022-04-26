use crate::{
    DecodeError, DecodeErrorHandler, DefaultErrorHandler, TopDecode, TopDecodeMultiInput,
    TopEncode, TypeInfo,
};

pub trait TopDecodeMulti: Sized {
    /// Used to optimize single value loading of endpoint arguments.
    const IS_SINGLE_VALUE: bool = false;

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
pub trait TopDecodeMultiLength {
    const LEN: usize;
    fn get_len() -> usize {
        Self::LEN
    }
}

/// All single top decode types also work as multi-value decode types.
impl<T> TopDecodeMulti for T
where
    T: TopDecode,
{
    const IS_SINGLE_VALUE: bool = true;

    fn multi_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeMultiInput,
        H: DecodeErrorHandler,
    {
        // const TYPE_INFO: TypeInfo = TypeInfo::Unknown;
        if Self::TYPE_INFO == TypeInfo::Unit {
            // unit type returns without loading anything
            let cast_unit: T = unsafe { core::mem::transmute_copy(&()) };
            return Ok(cast_unit);
        }

        input.next_value(h)
    }
}

impl<T> TopDecodeMultiLength for T
where
    T: TopEncode + TopDecode,
{
    const LEN: usize = 1;
}
