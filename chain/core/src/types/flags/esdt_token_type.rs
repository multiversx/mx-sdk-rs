use crate::codec::{
    self,
    derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
};

// Note: In the current implementation, SemiFungible is never returned

#[derive(TopDecode, TopEncode, NestedDecode, NestedEncode, Clone, Copy, PartialEq, Eq, Debug)]
pub enum EsdtTokenType {
    Fungible,
    NonFungible,
    NonFungibleV2,
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
            Self::NonFungibleV2 => 2,
            Self::SemiFungible => 3,
            Self::Meta => 4,
            Self::DynamicNFT => 5,
            Self::DynamicSFT => 6,
            Self::DynamicMeta => 7,
            Self::Invalid => 255,
        }
    }
}

impl From<u8> for EsdtTokenType {
    #[inline]
    fn from(value: u8) -> Self {
        match value {
            0 => EsdtTokenType::Fungible,
            1 => EsdtTokenType::NonFungible,
            2 => EsdtTokenType::NonFungibleV2,
            3 => EsdtTokenType::SemiFungible,
            4 => EsdtTokenType::Meta,
            5 => EsdtTokenType::DynamicNFT,
            6 => EsdtTokenType::DynamicSFT,
            7 => EsdtTokenType::DynamicMeta,
            _ => EsdtTokenType::Invalid,
        }
    }
}

impl From<Option<u64>> for EsdtTokenType {
    #[inline]
    fn from(value: Option<u64>) -> Self {
        match value {
            Some(0) => Self::Fungible,
            Some(1) => Self::NonFungible,
            Some(2) => Self::SemiFungible,
            Some(3) => Self::Meta,
            Some(4) => Self::DynamicNFT,
            Some(5) => Self::DynamicSFT,
            Some(6) => Self::DynamicMeta,
            _ => Self::Invalid,
        }
    }
}
