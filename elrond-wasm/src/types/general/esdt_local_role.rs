use elrond_codec::elrond_codec_derive::{NestedDecode, NestedEncode, TopDecode, TopEncode};

use crate as elrond_wasm; // needed by the TypeAbi generated code
use crate::derive::TypeAbi;

const ESDT_ROLE_NONE: &[u8] = &[];
const ESDT_ROLE_LOCAL_MINT: &[u8] = b"ESDTRoleLocalMint";
const ESDT_ROLE_LOCAL_BURN: &[u8] = b"ESDTRoleLocalBurn";
const ESDT_ROLE_NFT_CREATE: &[u8] = b"ESDTRoleNFTCreate";
const ESDT_ROLE_NFT_ADD_QUANTITY: &[u8] = b"ESDTRoleNFTAddQuantity";
const ESDT_ROLE_NFT_BURN: &[u8] = b"ESDTRoleNFTBurn";

#[derive(TopDecode, TopEncode, NestedDecode, NestedEncode, TypeAbi, Clone, PartialEq, Debug)]
pub enum EsdtLocalRole {
    None,
    Mint,
    Burn,
    NftCreate,
    NftAddQuantity,
    NftBurn,
}

impl EsdtLocalRole {
    pub fn as_u8(&self) -> u8 {
        match self {
            Self::None => 0,
            Self::Mint => 1,
            Self::Burn => 2,
            Self::NftCreate => 3,
            Self::NftAddQuantity => 4,
            Self::NftBurn => 5,
        }
    }

    pub fn as_role_name(&self) -> &'static [u8] {
        match self {
            Self::None => ESDT_ROLE_NONE,
            Self::Mint => ESDT_ROLE_LOCAL_MINT,
            Self::Burn => ESDT_ROLE_LOCAL_BURN,
            Self::NftCreate => ESDT_ROLE_NFT_CREATE,
            Self::NftAddQuantity => ESDT_ROLE_NFT_ADD_QUANTITY,
            Self::NftBurn => ESDT_ROLE_NFT_BURN,
        }
    }
}

impl From<u8> for EsdtLocalRole {
    #[inline]
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Mint,
            2 => Self::Burn,
            3 => Self::NftCreate,
            4 => Self::NftAddQuantity,
            5 => Self::NftBurn,
            _ => Self::None,
        }
    }
}

impl<'a> From<&'a [u8]> for EsdtLocalRole {
    #[inline]
    fn from(byte_slice: &'a [u8]) -> Self {
        if byte_slice == ESDT_ROLE_LOCAL_MINT {
            Self::Mint
        } else if byte_slice == ESDT_ROLE_LOCAL_BURN {
            Self::Burn
        } else if byte_slice == ESDT_ROLE_NFT_CREATE {
            Self::NftCreate
        } else if byte_slice == ESDT_ROLE_NFT_ADD_QUANTITY {
            Self::NftAddQuantity
        } else if byte_slice == ESDT_ROLE_NFT_BURN {
            Self::NftBurn
        } else {
            Self::None
        }
    }
}
