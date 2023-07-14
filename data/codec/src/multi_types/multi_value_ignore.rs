use crate::{
    CodecFrom, CodecFromSelf, DecodeErrorHandler, EncodeErrorHandler, TopDecodeMulti,
    TopDecodeMultiInput, TopEncodeMulti, TopEncodeMultiOutput,
};

/// Structure that allows taking a variable number of arguments,
/// but does nothing with them, not even deserialization.
#[derive(Default, Clone)]
pub struct IgnoreValue;

impl TopEncodeMulti for IgnoreValue {
    fn multi_encode_or_handle_err<O, H>(&self, _output: &mut O, _h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeMultiOutput,
        H: EncodeErrorHandler,
    {
        Ok(())
    }
}

impl TopDecodeMulti for IgnoreValue {
    fn multi_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeMultiInput,
        H: DecodeErrorHandler,
    {
        input.flush_ignore(h)?;
        Ok(IgnoreValue)
    }
}

impl !CodecFromSelf for IgnoreValue {}
impl<T> CodecFrom<T> for IgnoreValue where T: TopEncodeMulti {}
