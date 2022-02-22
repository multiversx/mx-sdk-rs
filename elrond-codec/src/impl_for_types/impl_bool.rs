use crate::{
    dep_encode_from_no_err, dep_encode_num_mimic, top_encode_from_no_err, DecodeError,
    DecodeErrorHandler, EncodeErrorHandler, NestedDecode, NestedDecodeInput, NestedEncode,
    NestedEncodeNoErr, NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode, TopEncodeNoErr,
    TopEncodeOutput, TypeInfo,
};

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

    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        match input.into_u64() {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(h.handle_error(DecodeError::INPUT_OUT_OF_RANGE)),
        }
    }
}

dep_encode_num_mimic! {bool, u8, TypeInfo::Bool}

impl NestedDecode for bool {
    const TYPE_INFO: TypeInfo = TypeInfo::Bool;

    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        match input.read_byte(h)? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(h.handle_error(DecodeError::INVALID_VALUE)),
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
