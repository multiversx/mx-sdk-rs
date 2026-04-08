// BLS keys have 96 bytes
const BLS_KEY_BYTE_LENGTH: usize = 96;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BLSKey([u8; BLS_KEY_BYTE_LENGTH]);

impl From<[u8; BLS_KEY_BYTE_LENGTH]> for BLSKey {
    #[inline]
    fn from(value: [u8; BLS_KEY_BYTE_LENGTH]) -> Self {
        Self(value)
    }
}

impl BLSKey {
    pub const fn len() -> usize {
        BLS_KEY_BYTE_LENGTH
    }

    #[inline]
    pub fn as_bytes(self) -> [u8; BLS_KEY_BYTE_LENGTH] {
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
}

use crate::codec::*;

impl NestedEncode for BLSKey {
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.0.dep_encode_or_handle_err(dest, h)
    }
}

impl TopEncode for BLSKey {
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.0.top_encode_or_handle_err(output, h)
    }
}

impl NestedDecode for BLSKey {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(BLSKey(
            <[u8; BLS_KEY_BYTE_LENGTH]>::dep_decode_or_handle_err(input, h)?,
        ))
    }
}

impl TopDecode for BLSKey {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(BLSKey(
            <[u8; BLS_KEY_BYTE_LENGTH]>::top_decode_or_handle_err(input, h)?,
        ))
    }
}
