use crate::codec::{
    self,
    derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
};

#[derive(TopDecode, TopEncode, NestedDecode, NestedEncode, Clone, Copy, PartialEq, Eq, Debug)]
pub enum EsdtTokenType {
    NotSet = 0,
    Fungible = 1,
    NonFungible = 2,
    NonFungibleV2 = 3,
    MetaFungible = 4,
    SemiFungible = 5,
    DynamicNFT = 6,
    DynamicSFT = 7,
    DynamicMeta = 8,
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
            EsdtTokenType::NotSet => 0,
            EsdtTokenType::Fungible => 1,
            EsdtTokenType::NonFungible => 2,
            EsdtTokenType::NonFungibleV2 => 3,
            EsdtTokenType::MetaFungible => 4,
            EsdtTokenType::SemiFungible => 5,
            EsdtTokenType::DynamicNFT => 6,
            EsdtTokenType::DynamicSFT => 7,
            EsdtTokenType::DynamicMeta => 8,
            EsdtTokenType::Invalid => 255,
        }
    }
}

impl From<u8> for EsdtTokenType {
    #[inline]
    fn from(value: u8) -> Self {
        match value {
            0 => EsdtTokenType::NotSet,
            1 => EsdtTokenType::Fungible,
            2 => EsdtTokenType::NonFungible,
            3 => EsdtTokenType::NonFungibleV2,
            4 => EsdtTokenType::MetaFungible,
            5 => EsdtTokenType::SemiFungible,
            6 => EsdtTokenType::DynamicNFT,
            7 => EsdtTokenType::DynamicSFT,
            8 => EsdtTokenType::DynamicMeta,
            _ => EsdtTokenType::Invalid,
        }
    }
}

impl From<Option<u64>> for EsdtTokenType {
    #[inline]
    fn from(opt: Option<u64>) -> Self {
        if let Some(value) = opt {
            EsdtTokenType::from(value as u8)
        } else {
            EsdtTokenType::Invalid
        }
    }
}
