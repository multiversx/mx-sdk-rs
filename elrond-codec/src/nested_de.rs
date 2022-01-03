use crate::{codec_err::DecodeError, nested_de_input::NestedDecodeInput, TypeInfo};

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
