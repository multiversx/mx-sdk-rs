use crate::abi::TypeAbi;
use alloc::string::String;
use elrond_codec::*;

const ESDT_TYPE_FUNGIBLE: &[u8] = b"FungibleESDT";
const ESDT_TYPE_NON_FUNGIBLE: &[u8] = b"NonFungibleESDT";
const ESDT_TYPE_SEMI_FUNGIBLE: &[u8] = b"SemiFungibleESDT";
const ESDT_TYPE_INVALID: &[u8] = &[];

// Note: In the current implementation, SemiFungible is never returned
#[derive(Clone, PartialEq, Debug)]
pub enum EsdtTokenType {
    Fungible,
    NonFungible,
    SemiFungible,
    Invalid,
}

impl EsdtTokenType {
    pub fn based_on_token_nonce(token_nonce: u64) -> Self {
        if token_nonce == 0 {
            EsdtTokenType::Fungible
        } else {
            EsdtTokenType::SemiFungible
        }
    }

    pub fn as_u8(&self) -> u8 {
        match self {
            Self::Fungible => 0,
            Self::NonFungible => 1,
            Self::SemiFungible => 2,
            Self::Invalid => 3,
        }
    }

    pub fn as_type_name(&self) -> &'static [u8] {
        match self {
            Self::Fungible => ESDT_TYPE_FUNGIBLE,
            Self::NonFungible => ESDT_TYPE_NON_FUNGIBLE,
            Self::SemiFungible => ESDT_TYPE_SEMI_FUNGIBLE,
            Self::Invalid => ESDT_TYPE_INVALID,
        }
    }
}

impl From<u8> for EsdtTokenType {
    #[inline]
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Fungible,
            1 => Self::NonFungible,
            2 => Self::SemiFungible,
            _ => Self::Invalid,
        }
    }
}

impl<'a> From<&'a [u8]> for EsdtTokenType {
    #[inline]
    fn from(byte_slice: &'a [u8]) -> Self {
        if byte_slice == ESDT_TYPE_FUNGIBLE {
            Self::Fungible
        } else if byte_slice == ESDT_TYPE_NON_FUNGIBLE {
            Self::NonFungible
        } else if byte_slice == ESDT_TYPE_SEMI_FUNGIBLE {
            Self::SemiFungible
        } else {
            Self::Invalid
        }
    }
}

impl NestedEncode for EsdtTokenType {
    #[inline]
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        self.as_u8().dep_encode(dest)
    }

    #[inline]
    fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
        &self,
        dest: &mut O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        self.as_u8().dep_encode_or_exit(dest, c, exit);
    }
}

impl TopEncode for EsdtTokenType {
    #[inline]
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        self.as_u8().top_encode(output)
    }

    #[inline]
    fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
        &self,
        output: O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        self.as_u8().top_encode_or_exit(output, c, exit);
    }
}

impl NestedDecode for EsdtTokenType {
    fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
        Ok(Self::from(u8::dep_decode(input)?))
    }

    fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
        input: &mut I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        Self::from(u8::dep_decode_or_exit(input, c, exit))
    }
}

impl TopDecode for EsdtTokenType {
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        Ok(Self::from(u8::top_decode(input)?))
    }

    fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
        input: I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        Self::from(u8::top_decode_or_exit(input, c, exit))
    }
}

impl TypeAbi for EsdtTokenType {
    fn type_name() -> String {
        "EsdtTokenType".into()
    }
}
