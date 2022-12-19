use crate::{
    DecodeErrorHandler, EncodeErrorHandler, TopDecodeMulti, TopDecodeMultiInput, TopEncodeMulti,
    TopEncodeMultiOutput,
};

impl TopEncodeMulti for () {
    fn multi_encode_or_handle_err<O, H>(&self, _output: &mut O, _h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeMultiOutput,
        H: EncodeErrorHandler,
    {
        Ok(())
    }
}

impl TopDecodeMulti for () {
    fn multi_decode_or_handle_err<I, H>(_input: &mut I, _h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeMultiInput,
        H: DecodeErrorHandler,
    {
        Ok(())
    }
}
