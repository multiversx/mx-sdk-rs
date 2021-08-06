use crate::codec_err::{DecodeError, EncodeError};
use crate::nested_de::NestedDecode;
use crate::nested_de_input::NestedDecodeInput;
use crate::nested_ser::{NestedEncode, NestedEncodeNoErr};
use crate::nested_ser_output::NestedEncodeOutput;
use crate::top_de::TopDecode;
use crate::top_de_input::TopDecodeInput;
use crate::top_ser::{TopEncode, TopEncodeNoErr};
use crate::top_ser_output::TopEncodeOutput;
use crate::{dep_encode_from_no_err, encode_num_mimic, top_encode_from_no_err, TypeInfo};

impl TopEncodeNoErr for bool {
    fn top_encode_no_err<O: TopEncodeOutput>(&self, output: O) {
        // only using signed because this one is implemented in Arwen, unsigned is not
        // TODO: change to set_u64
        output.set_i64(if *self { 1i64 } else { 0i64 });
    }
}

top_encode_from_no_err! {bool, TypeInfo::Bool}

impl TopDecode for bool {
    const TYPE_INFO: TypeInfo = TypeInfo::Bool;

    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        match input.into_u64() {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(DecodeError::INPUT_OUT_OF_RANGE),
        }
    }

    fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
        input: I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        match input.into_u64() {
            0 => false,
            1 => true,
            _ => exit(c, DecodeError::INPUT_OUT_OF_RANGE),
        }
    }
}

encode_num_mimic! {bool, u8, TypeInfo::Bool}

impl NestedDecode for bool {
    const TYPE_INFO: TypeInfo = TypeInfo::Bool;

    fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
        match input.read_byte()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(DecodeError::INVALID_VALUE),
        }
    }

    fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
        input: &mut I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        match input.read_byte_or_exit(c.clone(), exit) {
            0 => false,
            1 => true,
            _ => exit(c, DecodeError::INVALID_VALUE),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::test_util::{check_dep_encode_decode, check_top_encode_decode};

    #[test]
    fn test_top() {
        check_top_encode_decode(true, &[1]);
        check_top_encode_decode(false, &[]);
    }
    #[test]
    fn test_dep() {
        check_dep_encode_decode(true, &[1]);
        check_dep_encode_decode(false, &[0]);
    }
}
