use crate::{
    dep_encode_from_no_err, nested_ser::NestedEncodeNoErr, top_encode_from_no_err,
    top_ser::TopEncodeNoErr, DecodeError, EncodeError, NestedDecode, NestedDecodeInput,
    NestedEncode, NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
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

    fn top_decode<I: TopDecodeInput>(_: I) -> Result<Self, DecodeError> {
        Ok(())
    }

    fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
        _: I,
        _: ExitCtx,
        _: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
    }
}

impl NestedEncodeNoErr for () {
    fn dep_encode_no_err<O: NestedEncodeOutput>(&self, _: &mut O) {}
}

dep_encode_from_no_err! {(), TypeInfo::Unit}

impl NestedDecode for () {
    const TYPE_INFO: TypeInfo = TypeInfo::Unit;

    fn dep_decode<I: NestedDecodeInput>(_: &mut I) -> Result<(), DecodeError> {
        Ok(())
    }

    fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
        _: &mut I,
        _: ExitCtx,
        _: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
    }
}
