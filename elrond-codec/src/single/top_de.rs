use alloc::boxed::Box;

use crate::{
    codec_err::DecodeError, DecodeErrorHandler, DefaultErrorHandler, NestedDecode,
    NestedDecodeInput, TopDecodeInput,
};

/// Trait that allows zero-copy read of values from an underlying API in big endian format.
///
/// 'Top' stands for the fact that values are deserialized on their own,
/// so we have the benefit of knowing their length.
/// This is useful in many scnearios, such as not having to encode Vec length and others.
///
/// The opther optimization that can be done when deserializing top-level objects
/// is using special functions from the underlying API that do some of the work for the deserializer.
/// These include getting values directly as i64/u64 or wrapping them directly into an owned Box<[u8]>.
///
/// BigInt/BigUint handling is not included here, because these are API-dependent
/// and would overly complicate the trait.
///
pub trait TopDecode: Sized {
    /// Attempt to deserialize the value from input.
    fn top_decode<I>(input: I) -> Result<Self, DecodeError>
    where
        I: TopDecodeInput,
    {
        Self::top_decode_or_handle_err(input, DefaultErrorHandler)
    }

    /// Version of `top_decode` that can handle errors as soon as they occur.
    /// For instance it can exit immediately and make sure that if it returns, it is a success.
    /// By not deferring error handling, this can lead to somewhat smaller bytecode.
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        match Self::top_decode(input) {
            Ok(v) => Ok(v),
            Err(e) => Err(h.handle_error(e)),
        }
    }

    /// Allows types to provide optimized implementations for their boxed version.
    /// Especially designed for byte arrays that can be transmuted directly from the input sometimes.
    /// TODO: switch to the specialized mechanism
    #[doc(hidden)]
    #[inline]
    fn top_decode_boxed_or_handle_err<I, H>(input: I, h: H) -> Result<Box<Self>, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(Box::new(Self::top_decode_or_handle_err(input, h)?))
    }
}

/// Top-decodes the result using the NestedDecode implementation.
pub fn top_decode_from_nested_or_handle_err<I, T, H>(input: I, h: H) -> Result<T, H::HandledErr>
where
    I: TopDecodeInput,
    T: NestedDecode,
    H: DecodeErrorHandler,
{
    let mut nested_buffer = input.into_nested_buffer();
    let result = T::dep_decode_or_handle_err(&mut nested_buffer, h)?;
    if !nested_buffer.is_depleted() {
        return Err(h.handle_error(DecodeError::INPUT_TOO_LONG));
    }
    Ok(result)
}

/// Top-decodes the result using the NestedDecode implementation.
#[inline]
pub fn top_decode_from_nested<T, I>(input: I) -> Result<T, DecodeError>
where
    I: TopDecodeInput,
    T: NestedDecode,
{
    top_decode_from_nested_or_handle_err(input, DefaultErrorHandler)
}
