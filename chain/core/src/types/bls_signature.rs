/// BLS signatures have 48 bytes
pub const BLS_SIGNATURE_BYTE_LENGTH: usize = 48;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BLSSignature([u8; BLS_SIGNATURE_BYTE_LENGTH]);

impl From<[u8; BLS_SIGNATURE_BYTE_LENGTH]> for BLSSignature {
    #[inline]
    fn from(value: [u8; BLS_SIGNATURE_BYTE_LENGTH]) -> Self {
        Self(value)
    }
}

impl BLSSignature {
    pub const fn len() -> usize {
        BLS_SIGNATURE_BYTE_LENGTH
    }

    #[inline]
    pub fn as_bytes(self) -> [u8; BLS_SIGNATURE_BYTE_LENGTH] {
        self.0
    }
}

use crate::codec::*;

impl TopEncode for BLSSignature {
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.0.top_encode_or_handle_err(output, h)
    }
}

impl NestedDecode for BLSSignature {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(BLSSignature(
            <[u8; BLS_SIGNATURE_BYTE_LENGTH]>::dep_decode_or_handle_err(input, h)?,
        ))
    }
}

impl TopDecode for BLSSignature {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(BLSSignature(
            <[u8; BLS_SIGNATURE_BYTE_LENGTH]>::top_decode_or_handle_err(input, h)?,
        ))
    }
}
