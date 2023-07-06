use super::{esdt_local_role_names::*, EsdtLocalRoleFlags};

const ESDT_ROLE_NONE: &str = "";

/// The VM implementation for EsdtLocalRole, used internally in builtin functions.
///
/// There is another near-identical implementation in the framework, used for communicating with the VM.
///
/// It might be a good idea to move it to some "common ground" crate, between the framework and the VM.
#[derive(Clone, PartialEq, Eq, Debug, Copy)]
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
            Self::None => ESDT_ROLE_NONE.as_bytes(),
            Self::Mint => ESDT_ROLE_LOCAL_MINT.as_bytes(),
            Self::Burn => ESDT_ROLE_LOCAL_BURN.as_bytes(),
            Self::NftCreate => ESDT_ROLE_NFT_CREATE.as_bytes(),
            Self::NftAddQuantity => ESDT_ROLE_NFT_ADD_QUANTITY.as_bytes(),
            Self::NftBurn => ESDT_ROLE_NFT_BURN.as_bytes(),
            Self::NftAddUri => ESDT_ROLE_NFT_ADD_URI.as_bytes(),
            Self::NftUpdateAttributes => ESDT_ROLE_NFT_UPDATE_ATTRIBUTES.as_bytes(),
            Self::Transfer => ESDT_ROLE_TRANSFER.as_bytes(),
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
        } else if byte_slice == ESDT_ROLE_NFT_ADD_URI.as_bytes() {
            Self::NftAddUri
        } else if byte_slice == ESDT_ROLE_NFT_UPDATE_ATTRIBUTES.as_bytes() {
            Self::NftUpdateAttributes
        } else if byte_slice == ESDT_ROLE_TRANSFER.as_bytes() {
            Self::Transfer
        } else {
            Self::None
        }
    }
}
