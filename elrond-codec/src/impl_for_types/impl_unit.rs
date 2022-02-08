use crate::{
    dep_encode_from_no_err, top_encode_from_no_err, DecodeError, DecodeErrorHandler,
    EncodeErrorHandler, NestedDecode, NestedDecodeInput, NestedEncode, NestedEncodeNoErr,
    NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode, TopEncodeNoErr, TopEncodeOutput,
    TypeInfo,
};

impl TopEncodeNoErr for () {
    #[inline]
    fn top_encode_no_err<O: TopEncodeOutput>(&self, output: O) {
        output.set_unit();
    }
}

top_encode_from_no_err! {(), TypeInfo::Unit}

impl TopDecode for () {
    const TYPE_INFO: TypeInfo = TypeInfo::Unit;

    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        if input.byte_len() == 0 {
            Ok(())
        } else {
            Err(h.handle_error(DecodeError::INPUT_TOO_LONG))
        }
    }
}

impl NestedEncodeNoErr for () {
    fn dep_encode_no_err<O: NestedEncodeOutput>(&self, _: &mut O) {}
}

dep_encode_from_no_err! {(), TypeInfo::Unit}

impl NestedDecode for () {
    const TYPE_INFO: TypeInfo = TypeInfo::Unit;

    fn dep_decode_or_handle_err<I, H>(_input: &mut I, _h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(())
    }
}
