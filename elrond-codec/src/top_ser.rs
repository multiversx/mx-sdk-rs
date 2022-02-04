use crate::{
    codec_err::EncodeError, nested_ser::NestedEncode, top_ser_output::TopEncodeOutput,
    DefaultEncodeErrorHandler, EncodeErrorHandler, TypeInfo,
};
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
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        self.top_encode_or_handle_err(output, DefaultEncodeErrorHandler)
    }

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

    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        match self.top_encode(output) {
            Ok(()) => Ok(()),
            Err(e) => Err(h.handle_error(e)),
        }
    }
}

pub fn top_encode_from_nested<T, O, H>(obj: &T, output: O, h: H) -> Result<(), H::HandledErr>
where
    O: TopEncodeOutput,
    T: NestedEncode,
    H: EncodeErrorHandler,
{
    let mut nested_buffer = output.start_nested_encode();
    obj.dep_encode_or_handle_err(&mut nested_buffer, h)?;
    output.finalize_nested_encode(nested_buffer);
    Ok(())
}

pub fn top_encode_to_vec_u8<T: TopEncode>(obj: &T) -> Result<Vec<u8>, EncodeError> {
    let mut bytes = Vec::<u8>::new();
    obj.top_encode(&mut bytes)?;
    Ok(bytes)
}
