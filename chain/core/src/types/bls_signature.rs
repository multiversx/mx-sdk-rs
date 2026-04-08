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

    pub fn to_vec(self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn from_vec(v: Vec<u8>) -> Option<Self> {
        match v.try_into() {
            Ok(arr) => Some(Self(arr)),
            Err(_) => None,
        }
    }

    #[cfg(feature = "std")]
    pub fn parse_hex(hex_key: &str) -> Option<Self> {
        let Ok(v) = hex::decode(hex_key) else {
            return None;
        };
        Self::from_vec(v)
    }

    pub fn dummy(name: &str) -> Self {
        let name_bytes = name.as_bytes();
        assert!(name_bytes.len() < BLS_SIGNATURE_BYTE_LENGTH);
        let mut arr = [0u8; BLS_SIGNATURE_BYTE_LENGTH];
        arr[..name_bytes.len()].copy_from_slice(name_bytes);
        Self(arr)
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
