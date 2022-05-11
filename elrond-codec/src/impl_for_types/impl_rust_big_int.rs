use alloc::vec::Vec;
use num_bigint::BigInt;

use crate::{
    DecodeErrorHandler, EncodeErrorHandler, NestedDecode, NestedDecodeInput, NestedEncode,
    NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
};

impl TopEncode for BigInt {
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.to_signed_bytes_be()
            .top_encode_or_handle_err(output, h)
    }
}

impl TopDecode for BigInt {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        let bytes = Vec::<u8>::top_decode_or_handle_err(input, h)?;
        Ok(Self::from_signed_bytes_be(bytes.as_slice()))
    }
}

impl NestedEncode for BigInt {
    #[inline]
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.to_signed_bytes_be().dep_encode_or_handle_err(dest, h)
    }
}

impl NestedDecode for BigInt {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        let bytes = Vec::<u8>::dep_decode_or_handle_err(input, h)?;
        Ok(Self::from_signed_bytes_be(bytes.as_slice()))
    }
}

#[cfg(test)]
pub mod tests {
    use crate::test_util::{check_dep_encode_decode, check_top_encode_decode};
    use num_bigint::BigInt;

    #[test]
    fn test_top() {
        check_top_encode_decode(BigInt::from(5), &[5]);
        check_top_encode_decode(BigInt::from(127), &[127]);
        check_top_encode_decode(BigInt::from(128), &[0, 128]);
        check_top_encode_decode(BigInt::from(-128), &[128]);
    }

    #[test]
    fn test_dep() {
        check_dep_encode_decode(BigInt::from(5), &[0, 0, 0, 1, 5]);
        check_dep_encode_decode(BigInt::from(-5), &[0, 0, 0, 1, 251]);
    }
}
