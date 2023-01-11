use multiversx_sc::{
    codec::{
        DecodeError, DecodeErrorHandler, EncodeError, EncodeErrorHandler, NestedDecode,
        NestedDecodeInput, NestedEncode, NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode,
        TopEncodeOutput,
    },
    derive::TypeAbi,
};

/// Helper type to explore encode/decode errors.
#[derive(TypeAbi)]
pub struct CodecErrorTestType;

impl TopEncode for CodecErrorTestType {
    fn top_encode_or_handle_err<O, H>(&self, _output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        Err(h.handle_error(EncodeError::from("deliberate top encode error")))
    }
}

impl NestedEncode for CodecErrorTestType {
    fn dep_encode_or_handle_err<O, H>(&self, _dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        Err(h.handle_error(EncodeError::from("deliberate nested encode error")))
    }
}

impl TopDecode for CodecErrorTestType {
    fn top_decode_or_handle_err<I, H>(_input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        Err(h.handle_error(DecodeError::from("deliberate top decode error")))
    }
}

impl NestedDecode for CodecErrorTestType {
    fn dep_decode_or_handle_err<I, H>(_input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        Err(h.handle_error(DecodeError::from("deliberate top decode error")))
    }
}
