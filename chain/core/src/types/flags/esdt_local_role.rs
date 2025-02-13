use super::EsdtLocalRoleFlags;
use crate::codec::{
    self,
    derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
};

const ESDT_ROLE_NONE: &str = "";
const ESDT_ROLE_LOCAL_MINT: &str = "ESDTRoleLocalMint";
const ESDT_ROLE_LOCAL_BURN: &str = "ESDTRoleLocalBurn";
const ESDT_ROLE_NFT_CREATE: &str = "ESDTRoleNFTCreate";
const ESDT_ROLE_NFT_ADD_QUANTITY: &str = "ESDTRoleNFTAddQuantity";
const ESDT_ROLE_NFT_BURN: &str = "ESDTRoleNFTBurn";
const ESDT_ROLE_NFT_ADD_URI: &str = "ESDTRoleNFTAddURI";
const ESDT_ROLE_NFT_UPDATE_ATTRIBUTES: &str = "ESDTRoleNFTUpdateAttributes";
const ESDT_ROLE_SET_NEW_URI: &str = "ESDTRoleSetNewURI";
const ESDT_ROLE_MODIFY_ROYALTIES: &str = "ESDTRoleModifyRoyalties";
const ESDT_ROLE_MODIFY_CREATOR: &str = "ESDTRoleModifyCreator";
const ESDT_ROLE_NFT_RECREATE: &str = "ESDTRoleNFTRecreate";
const ESDT_ROLE_TRANSFER: &str = "ESDTTransferRole";

#[derive(
    TopDecode, TopEncode, NestedDecode, NestedEncode, Clone, PartialEq, Eq, Debug, Copy, Default,
)]
pub enum EsdtLocalRole {
    #[default]
    None,
    Mint,
    Burn,
    NftCreate,
    NftAddQuantity,
    NftBurn,
    NftUpdateAttributes,
    NftAddUri,
    NftRecreate,
    ModifyRoyalties,
    ModifyCreator,
    SetNewUri,
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
            Self::NftUpdateAttributes => 6,
            Self::NftAddUri => 7,
            Self::NftRecreate => 8,
            Self::ModifyCreator => 9,
            Self::ModifyRoyalties => 10,
            Self::SetNewUri => 11,
            Self::Transfer => 12,
        }
    }

    pub fn as_role_name(&self) -> &'static [u8] {
        self.name().as_bytes()
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::None => ESDT_ROLE_NONE,
            Self::Mint => ESDT_ROLE_LOCAL_MINT,
            Self::Burn => ESDT_ROLE_LOCAL_BURN,
            Self::NftCreate => ESDT_ROLE_NFT_CREATE,
            Self::NftAddQuantity => ESDT_ROLE_NFT_ADD_QUANTITY,
            Self::NftBurn => ESDT_ROLE_NFT_BURN,
            Self::NftUpdateAttributes => ESDT_ROLE_NFT_UPDATE_ATTRIBUTES,
            Self::NftAddUri => ESDT_ROLE_NFT_ADD_URI,
            Self::NftRecreate => ESDT_ROLE_NFT_RECREATE,
            Self::ModifyRoyalties => ESDT_ROLE_MODIFY_ROYALTIES,
            Self::ModifyCreator => ESDT_ROLE_MODIFY_CREATOR,
            Self::SetNewUri => ESDT_ROLE_SET_NEW_URI,
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
            Self::NftUpdateAttributes => EsdtLocalRoleFlags::NFT_UPDATE_ATTRIBUTES,
            Self::NftAddUri => EsdtLocalRoleFlags::NFT_ADD_URI,
            Self::NftRecreate => EsdtLocalRoleFlags::NFT_RECREATE,
            Self::ModifyRoyalties => EsdtLocalRoleFlags::MODIFY_ROYALTIES,
            Self::ModifyCreator => EsdtLocalRoleFlags::MODIFY_CREATOR,
            Self::SetNewUri => EsdtLocalRoleFlags::SET_NEW_URI,
            Self::Transfer => EsdtLocalRoleFlags::TRANSFER,
        }
    }
}

// TODO: can be done with macros, but I didn't find a public library that does it and is no_std
// we can implement it, it's easy
const ALL_ROLES: [EsdtLocalRole; 12] = [
    EsdtLocalRole::Mint,
    EsdtLocalRole::Burn,
    EsdtLocalRole::NftCreate,
    EsdtLocalRole::NftAddQuantity,
    EsdtLocalRole::NftBurn,
    EsdtLocalRole::NftUpdateAttributes,
    EsdtLocalRole::NftAddUri,
    EsdtLocalRole::NftRecreate,
    EsdtLocalRole::ModifyRoyalties,
    EsdtLocalRole::ModifyCreator,
    EsdtLocalRole::SetNewUri,
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
            6 => Self::NftUpdateAttributes,
            7 => Self::NftAddUri,
            8 => Self::NftRecreate,
            9 => Self::ModifyRoyalties,
            10 => Self::ModifyCreator,
            11 => Self::SetNewUri,
            12 => Self::Transfer,
            _ => Self::None,
        }
    }
}

impl<'a> From<&'a [u8]> for EsdtLocalRole {
    #[inline]
    fn from(byte_slice: &'a [u8]) -> Self {
        if byte_slice == ESDT_ROLE_LOCAL_MINT.as_bytes() {
            Self::Mint
        } else if byte_slice == ESDT_ROLE_LOCAL_BURN.as_bytes() {
            Self::Burn
        } else if byte_slice == ESDT_ROLE_NFT_CREATE.as_bytes() {
            Self::NftCreate
        } else if byte_slice == ESDT_ROLE_NFT_ADD_QUANTITY.as_bytes() {
            Self::NftAddQuantity
        } else if byte_slice == ESDT_ROLE_NFT_BURN.as_bytes() {
            Self::NftBurn
        } else if byte_slice == ESDT_ROLE_NFT_UPDATE_ATTRIBUTES.as_bytes() {
            Self::NftUpdateAttributes
        } else if byte_slice == ESDT_ROLE_NFT_ADD_URI.as_bytes() {
            Self::NftAddUri
        } else if byte_slice == ESDT_ROLE_NFT_RECREATE.as_bytes() {
            Self::NftRecreate
        } else if byte_slice == ESDT_ROLE_MODIFY_ROYALTIES.as_bytes() {
            Self::ModifyRoyalties
        } else if byte_slice == ESDT_ROLE_MODIFY_CREATOR.as_bytes() {
            Self::ModifyCreator
        } else if byte_slice == ESDT_ROLE_SET_NEW_URI.as_bytes() {
            Self::SetNewUri
        } else if byte_slice == ESDT_ROLE_TRANSFER.as_bytes() {
            Self::Transfer
        } else {
            Self::None
        }
    }
}
