use alloc::boxed::Box;

use crate::{
    codec_err::DecodeError, nested_de::*, top_de_input::TopDecodeInput, NestedDecodeInput, TypeInfo,
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
    #[doc(hidden)]
    const TYPE_INFO: TypeInfo = TypeInfo::Unknown;

    /// Attempt to deserialize the value from input.
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError>;

    /// Version of `top_decode` that exits quickly in case of error.
    /// Its purpose is to create smaller implementations
    /// in cases where the application is supposed to exit directly on decode error.
    #[inline]
    fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
        input: I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        match Self::top_decode(input) {
            Ok(v) => v,
            Err(e) => exit(c, e),
        }
    }

    /// Allows types to provide optimized implementations for their boxed version.
    /// Especially designed for byte arrays that can be transmuted directly from the input sometimes.
    #[doc(hidden)]
    #[inline]
    fn top_decode_boxed<I: TopDecodeInput>(input: I) -> Result<Box<Self>, DecodeError> {
        Ok(Box::new(Self::top_decode(input)?))
    }

    #[doc(hidden)]
    #[inline]
    fn top_decode_boxed_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
        input: I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Box<Self> {
        Box::new(Self::top_decode_or_exit(input, c, exit))
    }
}

/// Top-decodes the result using the NestedDecode implementation.
pub fn top_decode_from_nested<T, I>(input: I) -> Result<T, DecodeError>
where
    I: TopDecodeInput,
    T: NestedDecode,
{
    let mut nested_buffer = input.into_nested_buffer();
    let result = T::dep_decode(&mut nested_buffer)?;
    if !nested_buffer.is_depleted() {
        return Err(DecodeError::INPUT_TOO_LONG);
    }
    Ok(result)
}

/// Top-decodes the result using the NestedDecode implementation.
/// Uses the fast-exit mechanism in case of error.
pub fn top_decode_from_nested_or_exit<T, I, ExitCtx: Clone>(
    input: I,
    c: ExitCtx,
    exit: fn(ExitCtx, DecodeError) -> !,
) -> T
where
    I: TopDecodeInput,
    T: NestedDecode,
{
    let mut nested_buffer = input.into_nested_buffer();
    let result = T::dep_decode_or_exit(&mut nested_buffer, c.clone(), exit);
    if !nested_buffer.is_depleted() {
        exit(c, DecodeError::INPUT_TOO_LONG);
    }
    result
}
