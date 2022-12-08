use crate::{
    DecodeErrorHandler, EncodeErrorHandler, NestedDecode, NestedDecodeInput, NestedEncode,
    NestedEncodeOutput,
};

impl NestedEncode for () {
    #[inline]
    fn dep_encode_or_handle_err<O, H>(&self, _: &mut O, _h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        Ok(())
    }
}

impl NestedDecode for () {
    fn dep_decode_or_handle_err<I, H>(_input: &mut I, _h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(())
    }
}
