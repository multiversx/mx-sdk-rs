use crate::codec_err::EncodeError;
use crate::nested_ser::NestedEncode;
use crate::top_ser_output::TopEncodeOutput;
use crate::TypeInfo;
use alloc::vec::Vec;

/// Most types will be encoded without any possibility of error.
/// The trait is used to provide these implementations.
/// This is currently not a substitute for implementing a proper TopEncode.
pub trait TopEncodeNoErr: Sized {
    fn top_encode_no_err<O: TopEncodeOutput>(&self, output: O);
}

/// Quick encoding of a type that never fails on encoding.
pub fn top_encode_no_err<T: TopEncodeNoErr>(obj: &T) -> Vec<u8> {
    let mut bytes = Vec::<u8>::new();
    obj.top_encode_no_err(&mut bytes);
    bytes
}

pub trait TopEncode: Sized {
    // !INTERNAL USE ONLY!
    #[doc(hidden)]
    const TYPE_INFO: TypeInfo = TypeInfo::Unknown;

    /// Attempt to serialize the value to ouput.
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError>;

    /// Version of `top_decode` that exits quickly in case of error.
    /// Its purpose is to create smaller bytecode implementations
    /// in cases where the application is supposed to exit directly on decode error.
    fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
        &self,
        output: O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        match self.top_encode(output) {
            Ok(v) => v,
            Err(e) => exit(c, e),
        }
    }
}

pub fn top_encode_from_nested<T, O>(obj: &T, output: O) -> Result<(), EncodeError>
where
    O: TopEncodeOutput,
    T: NestedEncode,
{
    let mut bytes = Vec::<u8>::new();
    obj.dep_encode(&mut bytes)?;
    output.set_slice_u8(&bytes[..]);
    Ok(())
}

pub fn top_encode_from_nested_or_exit<T, O, ExitCtx>(
    obj: &T,
    output: O,
    c: ExitCtx,
    exit: fn(ExitCtx, EncodeError) -> !,
) where
    O: TopEncodeOutput,
    T: NestedEncode,
    ExitCtx: Clone,
{
    let mut bytes = Vec::<u8>::new();
    obj.dep_encode_or_exit(&mut bytes, c, exit);
    output.set_slice_u8(&bytes[..]);
}

pub fn top_encode_to_vec<T: TopEncode>(obj: &T) -> Result<Vec<u8>, EncodeError> {
    let mut bytes = Vec::<u8>::new();
    obj.top_encode(&mut bytes)?;
    Ok(bytes)
}
