use crate::codec::{
    self,
    derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
};

#[derive(TopDecode, TopEncode, NestedDecode, NestedEncode, Clone, Copy, PartialEq, Eq, Debug)]
pub enum EsdtTokenType {
    Fungible = 0,
    NonFungible = 1,
    NonFungibleV2 = 2,
    SemiFungible = 3,
    MetaFungible = 4,
    DynamicNFT = 5,
    DynamicSFT = 6,
    DynamicMeta = 7,
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
            EsdtTokenType::Fungible => 0,
            EsdtTokenType::NonFungible => 1,
            EsdtTokenType::NonFungibleV2 => 2,
            EsdtTokenType::SemiFungible => 3,
            EsdtTokenType::MetaFungible => 4,
            EsdtTokenType::DynamicNFT => 5,
            EsdtTokenType::DynamicSFT => 6,
            EsdtTokenType::DynamicMeta => 7,
            EsdtTokenType::Invalid => 255,
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
            4 => EsdtTokenType::MetaFungible,
            5 => EsdtTokenType::DynamicNFT,
            6 => EsdtTokenType::DynamicSFT,
            7 => EsdtTokenType::DynamicMeta,
            _ => EsdtTokenType::Invalid,
        }
    }
}
