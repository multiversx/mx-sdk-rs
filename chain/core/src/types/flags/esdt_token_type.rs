use crate::codec::{
    self,
    derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
};

const ESDT_TYPE_FUNGIBLE: &[u8] = b"FungibleESDT";
const ESDT_TYPE_NON_FUNGIBLE: &[u8] = b"NonFungibleESDT";
const ESDT_TYPE_SEMI_FUNGIBLE: &[u8] = b"SemiFungibleESDT";
const ESDT_TYPE_META: &[u8] = b"MetaESDT";
const ESDT_TYPE_DYNAMIC_NON_FUNGIBLE: &[u8] = b"DynamicNonFungibleESDT";
const ESDT_TYPE_DYNAMIC_SEMI_FUNGIBLE: &[u8] = b"DynamicSemiFungibleESDT";
const ESDT_TYPE_DYNAMIC_META: &[u8] = b"DynamicMetaESDT";
const ESDT_TYPE_INVALID: &[u8] = &[];

// Note: In the current implementation, SemiFungible is never returned

#[derive(TopDecode, TopEncode, NestedDecode, NestedEncode, Clone, Copy, PartialEq, Eq, Debug)]
pub enum EsdtTokenType {
    Fungible,
    NonFungible,
    SemiFungible,
    Meta,
    DynamicNFT,
    DynamicSFT,
    DynamicMeta,
    Invalid,
}

impl EsdtTokenType {
    pub fn based_on_token_nonce(token_nonce: u64) -> Self {
        if token_nonce == 0 {
            EsdtTokenType::Fungible
        } else {
            EsdtTokenType::NonFungible
        }
    }

    pub fn as_u8(&self) -> u8 {
        match self {
            Self::Fungible => 0,
            Self::NonFungible => 1,
            Self::SemiFungible => 2,
            Self::Meta => 3,
            Self::DynamicNFT => 4,
            Self::DynamicSFT => 5,
            Self::DynamicMeta => 6,
            Self::Invalid => 7,
        }
    }

    pub fn as_type_name(&self) -> &'static [u8] {
        match self {
            Self::Fungible => ESDT_TYPE_FUNGIBLE,
            Self::NonFungible => ESDT_TYPE_NON_FUNGIBLE,
            Self::SemiFungible => ESDT_TYPE_SEMI_FUNGIBLE,
            Self::Meta => ESDT_TYPE_META,
            Self::DynamicNFT => ESDT_TYPE_DYNAMIC_NON_FUNGIBLE,
            Self::DynamicSFT => ESDT_TYPE_DYNAMIC_SEMI_FUNGIBLE,
            Self::DynamicMeta => ESDT_TYPE_DYNAMIC_META,
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
            3 => Self::Meta,
            4 => Self::DynamicNFT,
            5 => Self::DynamicSFT,
            6 => Self::DynamicMeta,
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
        } else if byte_slice == ESDT_TYPE_META {
            Self::Meta
        } else if byte_slice == ESDT_TYPE_DYNAMIC_NON_FUNGIBLE {
            Self::DynamicNFT
        } else if byte_slice == ESDT_TYPE_DYNAMIC_SEMI_FUNGIBLE {
            Self::DynamicSFT
        } else if byte_slice == ESDT_TYPE_DYNAMIC_META {
            Self::DynamicMeta
        } else {
            Self::Invalid
        }
    }
}

impl From<Option<u64>> for EsdtTokenType {
    #[inline]
    fn from(value: Option<u64>) -> Self {
        if value.is_none() {
            return Self::Invalid;
        }

        match value.unwrap() {
            0 => Self::Fungible,
            1 => Self::NonFungible,
            2 => Self::SemiFungible,
            3 => Self::Meta,
            4 => Self::DynamicNFT,
            5 => Self::DynamicSFT,
            6 => Self::DynamicMeta,
            _ => Self::Invalid,
        }
    }
}
