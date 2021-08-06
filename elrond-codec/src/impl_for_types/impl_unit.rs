use crate::codec_err::{DecodeError, EncodeError};
use crate::nested_de::NestedDecode;
use crate::nested_de_input::NestedDecodeInput;
use crate::nested_ser::NestedEncodeNoErr;
use crate::nested_ser_output::NestedEncodeOutput;
use crate::top_de::TopDecode;
use crate::top_de_input::TopDecodeInput;
use crate::top_ser::{TopEncode, TopEncodeNoErr};
use crate::top_ser_output::TopEncodeOutput;
use crate::{top_encode_from_no_err, TypeInfo};

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
