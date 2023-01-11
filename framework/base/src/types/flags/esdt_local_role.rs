use crate::{
    codec,
    codec::derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
};

use super::EsdtLocalRoleFlags;
use crate as multiversx_sc;
use crate::{derive::TypeAbi, types::ManagedVecItem};

static ESDT_ROLE_NONE: &[u8] = &[];
static ESDT_ROLE_LOCAL_MINT: &[u8] = b"ESDTRoleLocalMint";
static ESDT_ROLE_LOCAL_BURN: &[u8] = b"ESDTRoleLocalBurn";
static ESDT_ROLE_NFT_CREATE: &[u8] = b"ESDTRoleNFTCreate";
static ESDT_ROLE_NFT_ADD_QUANTITY: &[u8] = b"ESDTRoleNFTAddQuantity";
static ESDT_ROLE_NFT_BURN: &[u8] = b"ESDTRoleNFTBurn";
static ESDT_ROLE_NFT_ADD_URI: &[u8] = b"ESDTRoleNFTAddURI";
static ESDT_ROLE_NFT_UPDATE_ATTRIBUTES: &[u8] = b"ESDTRoleNFTUpdateAttributes";
static ESDT_ROLE_TRANSFER: &[u8] = b"ESDTTransferRole";

#[derive(
    TopDecode, TopEncode, NestedDecode, NestedEncode, TypeAbi, Clone, PartialEq, Eq, Debug, Copy,
)]
pub enum EsdtLocalRole {
    None,
    Mint,
    Burn,
    NftCreate,
    NftAddQuantity,
    NftBurn,
    NftAddUri,
    NftUpdateAttributes,
    Transfer,
}

impl EsdtLocalRole {
    pub fn as_u16(&self) -> u16 {
        match self {
            Self::None => 0,
            Self::Mint => 1,
            Self::Burn => 2,
            Self::NftCreate => 3,
            Self::NftAddQuantity => 4,
            Self::NftBurn => 5,
            Self::NftAddUri => 6,
            Self::NftUpdateAttributes => 7,
            Self::Transfer => 8,
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
            Self::NftAddUri => ESDT_ROLE_NFT_ADD_URI,
            Self::NftUpdateAttributes => ESDT_ROLE_NFT_UPDATE_ATTRIBUTES,
            Self::Transfer => ESDT_ROLE_TRANSFER,
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
            Self::NftAddUri => EsdtLocalRoleFlags::NFT_ADD_URI,
            Self::NftUpdateAttributes => EsdtLocalRoleFlags::NFT_UPDATE_ATTRIBUTES,
            Self::Transfer => EsdtLocalRoleFlags::TRANSFER,
        }
    }
}

// TODO: can be done with macros, but I didn't find a public library that does it and is no_std
// we can implement it, it's easy
const ALL_ROLES: [EsdtLocalRole; 8] = [
    EsdtLocalRole::Mint,
    EsdtLocalRole::Burn,
    EsdtLocalRole::NftCreate,
    EsdtLocalRole::NftAddQuantity,
    EsdtLocalRole::NftBurn,
    EsdtLocalRole::NftAddUri,
    EsdtLocalRole::NftUpdateAttributes,
    EsdtLocalRole::Transfer,
];

impl EsdtLocalRole {
    pub fn iter_all() -> core::slice::Iter<'static, EsdtLocalRole> {
        ALL_ROLES.iter()
    }
}

impl From<u16> for EsdtLocalRole {
    #[inline]
    fn from(value: u16) -> Self {
        match value {
            1 => Self::Mint,
            2 => Self::Burn,
            3 => Self::NftCreate,
            4 => Self::NftAddQuantity,
            5 => Self::NftBurn,
            6 => Self::NftAddUri,
            7 => Self::NftUpdateAttributes,
            8 => Self::Transfer,
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
        } else if byte_slice == ESDT_ROLE_NFT_ADD_URI {
            Self::NftAddUri
        } else if byte_slice == ESDT_ROLE_NFT_UPDATE_ATTRIBUTES {
            Self::NftUpdateAttributes
        } else if byte_slice == ESDT_ROLE_TRANSFER {
            Self::Transfer
        } else {
            Self::None
        }
    }
}

impl ManagedVecItem for EsdtLocalRole {
    const PAYLOAD_SIZE: usize = 1;
    const SKIPS_RESERIALIZATION: bool = false; // TODO: might be ok to be true, but needs testing
    type Ref<'a> = Self;

    fn from_byte_reader<Reader: FnMut(&mut [u8])>(reader: Reader) -> Self {
        u16::from_byte_reader(reader).into()
    }

    unsafe fn from_byte_reader_as_borrow<'a, Reader: FnMut(&mut [u8])>(
        reader: Reader,
    ) -> Self::Ref<'a> {
        Self::from_byte_reader(reader)
    }

    fn to_byte_writer<R, Writer: FnMut(&[u8]) -> R>(&self, writer: Writer) -> R {
        <u16 as ManagedVecItem>::to_byte_writer(&self.as_u16(), writer)
    }
}
