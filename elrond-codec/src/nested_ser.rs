use crate::{codec_err::EncodeError, nested_ser_output::NestedEncodeOutput, TypeInfo};
use alloc::vec::Vec;

/// Most types will be encoded without any possibility of error.
/// The trait is used to provide these implementations.
/// This is currently not a substitute for implementing a proper TopEncode.
pub trait NestedEncodeNoErr: Sized {
    fn dep_encode_no_err<O: NestedEncodeOutput>(&self, dest: &mut O);
}

/// Trait that allows zero-copy write of value-references to slices in LE format.
///
/// Implementations should override `using_top_encoded` for value types and `dep_encode` and `size_hint` for allocating types.
/// Wrapper types should override all methods.
pub trait NestedEncode: Sized {
    // !INTERNAL USE ONLY!
    // This const helps SCALE to optimize the encoding/decoding by doing fake specialization.
    #[doc(hidden)]
    const TYPE_INFO: TypeInfo = TypeInfo::Unknown;

    /// NestedEncode to output, using the format of an object nested inside another structure.
    /// Does not provide compact version.
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError>;

    /// Version of `top_decode` that exits quickly in case of error.
    /// Its purpose is to create smaller implementations
    /// in cases where the application is supposed to exit directly on decode error.
    fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
        &self,
        dest: &mut O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        match self.dep_encode(dest) {
            Ok(v) => v,
            Err(e) => exit(c, e),
        }
    }
}

/// Convenience function for getting an object nested-encoded to a Vec<u8> directly.
pub fn dep_encode_to_vec<T: NestedEncode>(obj: &T) -> Result<Vec<u8>, EncodeError> {
    let mut bytes = Vec::<u8>::new();
    obj.dep_encode(&mut bytes)?;
    Ok(bytes)
}
