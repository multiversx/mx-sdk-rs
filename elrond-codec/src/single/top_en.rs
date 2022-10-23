use crate::{
    codec_err::EncodeError, DefaultErrorHandler, EncodeErrorHandler, NestedEncode,
    PanicErrorHandler, TopEncodeOutput,
};
use alloc::vec::Vec;

pub trait TopEncode: Sized {
    /// Attempt to serialize the value to ouput.
    fn top_encode<O>(&self, output: O) -> Result<(), EncodeError>
    where
        O: TopEncodeOutput,
    {
        self.top_encode_or_handle_err(output, DefaultErrorHandler)
    }

    /// Version of `top_encode` that can handle errors as soon as they occur.
    /// For instance in can exit immediately and make sure that if it returns, it is a success.
    /// By not deferring error handling, this can lead to somewhat smaller bytecode.
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

pub fn top_encode_to_vec_u8_or_panic<T: TopEncode>(obj: &T) -> Vec<u8> {
    let mut bytes = Vec::<u8>::new();
    let Ok(()) = obj.top_encode_or_handle_err(&mut bytes, PanicErrorHandler);
    bytes
}
