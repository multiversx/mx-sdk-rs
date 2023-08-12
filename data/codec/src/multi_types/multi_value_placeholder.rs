use crate::{
    CodecFrom, CodecFromSelf, DecodeError, DecodeErrorHandler, EncodeError, EncodeErrorHandler,
    TopDecodeMulti, TopDecodeMultiInput, TopEncodeMulti, TopEncodeMultiOutput,
};

/// Temporary value used for any kind of templates.
///
/// Can be used for compiling example code, in which it encodes to anything, but will always fail at runtime.
#[derive(Clone, Copy, Debug)]
pub struct PlaceholderInput;

impl TopEncodeMulti for PlaceholderInput {
    fn multi_encode_or_handle_err<O, H>(&self, _output: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeMultiOutput,
        H: EncodeErrorHandler,
    {
        Err(h.handle_error(EncodeError::from("placeholder only, cannot encode")))
    }
}

impl !CodecFromSelf for PlaceholderInput {}
impl<T> CodecFrom<PlaceholderInput> for T where T: TopDecodeMulti + CodecFromSelf {}

/// Temporary value used for any kind of templates.
///
/// Can be used for compiling example code, in which it decodes from anything, but will always fail at runtime.
#[derive(Clone, Copy, Debug)]
pub struct PlaceholderOutput;

impl TopDecodeMulti for PlaceholderOutput {
    fn multi_decode_or_handle_err<I, H>(_input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeMultiInput,
        H: DecodeErrorHandler,
    {
        Err(h.handle_error(DecodeError::from("placeholder only, cannot decode")))
    }
}

impl !CodecFromSelf for PlaceholderOutput {}
impl<T> CodecFrom<T> for PlaceholderOutput where T: TopEncodeMulti + CodecFromSelf {}
