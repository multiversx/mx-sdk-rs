use crate::{
    DecodeError, DecodeErrorHandler, DefaultErrorHandler, TopDecode, TopDecodeMultiInput, TypeInfo,
};

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
        // const TYPE_INFO: TypeInfo = TypeInfo::Unknown;
        if Self::TYPE_INFO == TypeInfo::Unit {
            // unit type returns without loading anything
            let cast_unit: T = unsafe { core::mem::transmute_copy(&()) };
            return Ok(cast_unit);
        }

        input.next_value(h)
    }
}
