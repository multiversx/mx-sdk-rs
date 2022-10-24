use crate::{codec_err::EncodeError, DefaultErrorHandler, EncodeErrorHandler, NestedEncodeOutput};
use alloc::vec::Vec;

/// Trait that allows zero-copy write of value-references to slices in LE format.
///
/// Implementations should override `using_top_encoded` for value types and `dep_encode` and `size_hint` for allocating types.
/// Wrapper types should override all methods.
pub trait NestedEncode: Sized {
    /// NestedEncode to output, using the format of an object nested inside another structure.
    /// Does not provide compact version.
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        self.dep_encode_or_handle_err(dest, DefaultErrorHandler)
    }

    /// Version of `dep_encode` that can handle errors as soon as they occur.
    /// For instance in can exit immediately and make sure that if it returns, it is a success.
    /// By not deferring error handling, this can lead to somewhat smaller bytecode.
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        match self.dep_encode(dest) {
            Ok(()) => Ok(()),
            Err(e) => Err(h.handle_error(e)),
        }
    }

    /// Allows the framework to do monomorphisation of special cases where the data is of type `u8`.
    ///
    /// Especially useful for serializing byte arrays.
    ///
    /// Working with this also involves transmuting low-level data. Only use if you really know what you are doing!
    #[doc(hidden)]
    #[allow(unused_variables)]
    fn if_u8<Output, If, Else, R>(output: Output, if_branch: If, else_branch: Else) -> R
    where
        If: FnOnce(Output) -> R,
        Else: FnOnce(Output) -> R,
    {
        else_branch(output)
    }
}

/// Convenience function for getting an object nested-encoded to a Vec<u8> directly.
pub fn dep_encode_to_vec<T: NestedEncode>(obj: &T) -> Result<Vec<u8>, EncodeError> {
    let mut bytes = Vec::<u8>::new();
    obj.dep_encode(&mut bytes)?;
    Ok(bytes)
}
