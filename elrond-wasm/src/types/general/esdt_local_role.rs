use elrond_codec::elrond_codec_derive::{NestedDecode, NestedEncode, TopDecode, TopEncode};

use super::EsdtLocalRoleFlags;
use crate as elrond_wasm;
use crate::{derive::TypeAbi, types::ManagedVecItem};

const ESDT_ROLE_NONE: &[u8] = &[];
const ESDT_ROLE_LOCAL_MINT: &[u8] = b"ESDTRoleLocalMint";
const ESDT_ROLE_LOCAL_BURN: &[u8] = b"ESDTRoleLocalBurn";
const ESDT_ROLE_NFT_CREATE: &[u8] = b"ESDTRoleNFTCreate";
const ESDT_ROLE_NFT_ADD_QUANTITY: &[u8] = b"ESDTRoleNFTAddQuantity";
const ESDT_ROLE_NFT_BURN: &[u8] = b"ESDTRoleNFTBurn";

#[derive(
    TopDecode, TopEncode, NestedDecode, NestedEncode, TypeAbi, Clone, PartialEq, Debug, Copy,
)]
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

    pub fn to_flag(&self) -> EsdtLocalRoleFlags {
        match self {
            Self::None => EsdtLocalRoleFlags::NONE,
            Self::Mint => EsdtLocalRoleFlags::MINT,
            Self::Burn => EsdtLocalRoleFlags::BURN,
            Self::NftCreate => EsdtLocalRoleFlags::NFT_CREATE,
            Self::NftAddQuantity => EsdtLocalRoleFlags::NFT_ADD_QUANTITY,
            Self::NftBurn => EsdtLocalRoleFlags::NFT_BURN,
        }
    }
}

// TODO: can be done with macros, but I didn't find a public library that does it and is no_std
// we can implement it, it's easy
const ALL_ROLES: [EsdtLocalRole; 5] = [
    EsdtLocalRole::Mint,
    EsdtLocalRole::Burn,
    EsdtLocalRole::NftCreate,
    EsdtLocalRole::NftAddQuantity,
    EsdtLocalRole::NftBurn,
];

impl EsdtLocalRole {
    pub fn iter_all() -> core::slice::Iter<'static, EsdtLocalRole> {
        ALL_ROLES.iter()
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

impl ManagedVecItem for EsdtLocalRole {
    const PAYLOAD_SIZE: usize = 1;
    const SKIPS_RESERIALIZATION: bool = false; // TODO: might be ok to be true, but needs testing

    fn from_byte_reader<Reader: FnMut(&mut [u8])>(reader: Reader) -> Self {
        u8::from_byte_reader(reader).into()
    }

    fn to_byte_writer<R, Writer: FnMut(&[u8]) -> R>(&self, writer: Writer) -> R {
        <u8 as ManagedVecItem>::to_byte_writer(&self.as_u8(), writer)
    }
}
