use crate::codec_err::DecodeError;
use crate::nested_de_input::NestedDecodeInput;
use crate::TypeInfo;

/// Trait that allows zero-copy read of value-references from slices in LE format.
pub trait NestedDecode: Sized {
    // !INTERNAL USE ONLY!
    // This const helps elrond-wasm to optimize the encoding/decoding by doing fake specialization.
    #[doc(hidden)]
    const TYPE_INFO: TypeInfo = TypeInfo::Unknown;

    /// Attempt to deserialise the value from input,
    /// using the format of an object nested inside another structure.
    /// In case of success returns the deserialized value and the number of bytes consumed during the operation.
    fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError>;

    /// Version of `top_decode` that exits quickly in case of error.
    /// Its purpose is to create smaller implementations
    /// in cases where the application is supposed to exit directly on decode error.
    fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
        input: &mut I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        match Self::dep_decode(input) {
            Ok(v) => v,
            Err(e) => exit(c, e),
        }
    }
}

/// Convenience method, to avoid having to specify type when calling `dep_decode`.
/// Especially useful in the macros.
/// Also checks that the entire slice was used.
/// The input doesn't need to be mutable because we are not changing the underlying data.
pub fn dep_decode_from_byte_slice<D: NestedDecode>(input: &[u8]) -> Result<D, DecodeError> {
    let mut_slice = &mut &*input;
    let result = D::dep_decode(mut_slice);
    if !mut_slice.is_empty() {
        return Err(DecodeError::INPUT_TOO_LONG);
    }
    result
}

pub fn dep_decode_from_byte_slice_or_exit<D: NestedDecode, ExitCtx: Clone>(
    input: &[u8],
    c: ExitCtx,
    exit: fn(ExitCtx, DecodeError) -> !,
) -> D {
    let mut_slice = &mut &*input;
    let result = D::dep_decode_or_exit(mut_slice, c.clone(), exit);
    if !mut_slice.is_empty() {
        exit(c, DecodeError::INPUT_TOO_LONG);
    }
    result
}

impl NestedDecode for u8 {
    const TYPE_INFO: TypeInfo = TypeInfo::U8;

    fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
        input.read_byte()
    }

    fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
        input: &mut I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        input.read_byte_or_exit(c, exit)
    }
}
