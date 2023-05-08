// use core::ops::Try;

use crate::{codec_err::DecodeError, DecodeErrorHandler, DefaultErrorHandler, NestedDecodeInput};

/// Trait that allows zero-copy read of value-references from slices in LE format.
pub trait NestedDecode: Sized {
    /// Attempt to deserialise the value from input,
    /// using the format of an object nested inside another structure.
    /// In case of success returns the deserialized value and the number of bytes consumed during the operation.
    fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
        Self::dep_decode_or_handle_err(input, DefaultErrorHandler)
    }

    /// Version of `dep_decode` that can handle errors as soon as they occur.
    /// For instance in can exit immediately and make sure that if it returns, it is a success.
    /// By not deferring error handling, this can lead to somewhat smaller bytecode.
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        match Self::dep_decode(input) {
            Ok(v) => Ok(v),
            Err(e) => Err(h.handle_error(e)),
        }
    }

    /// Allows the framework to do monomorphisation of special cases where the data is of type `u8`.
    ///
    /// Especially useful for deserializing byte arrays.
    ///
    /// Working with this also involves transmuting low-level data. Only use if you really know what you are doing!
    #[doc(hidden)]
    #[allow(unused_variables)]
    fn if_u8<Input, If, Else, R>(input: Input, if_branch: If, else_branch: Else) -> R
    where
        If: FnOnce(Input) -> R,
        Else: FnOnce(Input) -> R,
    {
        else_branch(input)
    }
}
