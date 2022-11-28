use crate::{
    dep_encode_num_mimic, DecodeError, DecodeErrorHandler, EncodeErrorHandler, NestedDecode,
    NestedDecodeInput, NestedEncode, NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode,
    TopEncodeOutput,
};

impl TopEncode for bool {
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, _h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        // only using signed because this one is implemented in Arwen, unsigned is not
        // TODO: change to set_u64
        // true -> 1i64
        // false -> 0i64
        output.set_i64(i64::from(*self));
        Ok(())
    }
}

impl TopDecode for bool {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        match input.into_u64(h)? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(h.handle_error(DecodeError::INPUT_OUT_OF_RANGE)),
        }
    }
}

dep_encode_num_mimic! {bool, u8}

impl NestedDecode for bool {
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
