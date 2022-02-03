use elrond_wasm::{
    derive::TypeAbi,
    elrond_codec::{
        DecodeError, EncodeError, NestedDecode, NestedDecodeInput, NestedEncode,
        NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
    },
};

/// Helper type to explore encode/decode errors.
#[derive(TypeAbi)]
pub struct CodecErrorTestType;

impl TopEncode for CodecErrorTestType {
    #[inline]
    fn top_encode<O: TopEncodeOutput>(&self, _output: O) -> Result<(), EncodeError> {
        Err(EncodeError::from("deliberate top encode error"))
    }
}

impl NestedEncode for CodecErrorTestType {
    fn dep_encode<O: NestedEncodeOutput>(&self, _dest: &mut O) -> Result<(), EncodeError> {
        Err(EncodeError::from("deliberate nested encode error"))
    }
}

impl TopDecode for CodecErrorTestType {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        Err(h.handle_error(DecodeError::from("deliberate top decode error")))
    }
}

impl NestedDecode for CodecErrorTestType {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        Err(h.handle_error(DecodeError::from("deliberate top decode error")))
    }
}
