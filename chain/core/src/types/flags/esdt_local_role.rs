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

/// An ESDT local role that can be granted to or revoked from an account for a specific token.
///
/// Each role has two canonical representations:
/// - A **byte-string name** (e.g. `"ESDTRoleLocalMint"`) used in on-chain role
///   assignment, decoded via `From<&[u8]>`.
/// - A **numeric ID** (`u16`) used for `ManagedVecItem` payload encoding,
///   accessed via [`as_u16`][EsdtLocalRole::as_u16] and decoded via `From<u16>`.
///
/// The numeric IDs and bit-flag positions correspond to the `Role*` iota constants
/// in `vmhost/vmhooks/eei_helpers.go` of `mx-chain-vm-go`.
///
/// The declaration order matches the Go VM `iota` ordering so that the codec
/// discriminant (declaration index) aligns with [`as_u16`][EsdtLocalRole::as_u16].
#[derive(TopDecode, TopEncode, NestedDecode, NestedEncode, Clone, PartialEq, Eq, Debug, Copy)]
pub enum EsdtLocalRole {
    None,
    Mint,
    Burn,
    NftCreate,
    NftAddQuantity,
    NftBurn,
    NftUpdateAttributes,
    NftAddUri,
    NftRecreate,
    ModifyCreator,
    ModifyRoyalties,
    SetNewUri,
    Transfer,
}

impl EsdtLocalRole {
    /// Returns the 1-based ordinal role ID used for `ManagedVecItem` payload encoding.
    ///
    /// This is a sequential index (1..=12), **not** a bitflag value. The
    /// corresponding bitflag in [`EsdtLocalRoleFlags`] is `1 << (id - 1)`, which
    /// aligns with the Go VM's `Role*` iota ordering in
    /// `vmhost/vmhooks/eei_helpers.go`. `Transfer` (12) has no counterpart in
    /// the current Go VM iota. This is the inverse of `From<u16>`.
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

    /// Returns the canonical byte-string name of this role (e.g. `b"ESDTRoleLocalMint"`).
    ///
    /// This is the format used in on-chain ESDT role grant/revoke operations.
    pub fn as_role_name(&self) -> &'static [u8] {
        self.name().as_bytes()
    }

    /// Returns the canonical string name of this role (e.g. `"ESDTRoleLocalMint"`).
    ///
    /// This is the format used in on-chain ESDT role grant/revoke operations and
    /// corresponds to the role-name constants in the Go VM.
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

    /// Converts this role to its corresponding bit-flag in [`EsdtLocalRoleFlags`].
    ///
    /// The bit position matches the Go VM's `Role*` iota constant for the same role.
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
    EsdtLocalRole::ModifyCreator,
    EsdtLocalRole::ModifyRoyalties,
    EsdtLocalRole::SetNewUri,
    EsdtLocalRole::Transfer,
];

impl EsdtLocalRole {
    /// Iterates over all non-`None` roles in canonical numeric-ID order.
    pub fn iter_all() -> core::slice::Iter<'static, EsdtLocalRole> {
        ALL_ROLES.iter()
    }
}

impl From<u16> for EsdtLocalRole {
    /// Decodes a numeric role ID into an [`EsdtLocalRole`].
    ///
    /// This is the inverse of [`EsdtLocalRole::as_u16`]. Unknown values map to
    /// [`EsdtLocalRole::None`].
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
            9 => Self::ModifyCreator,
            10 => Self::ModifyRoyalties,
            11 => Self::SetNewUri,
            12 => Self::Transfer,
            _ => Self::None,
        }
    }
}

impl<'a> From<&'a [u8]> for EsdtLocalRole {
    /// Decodes an ESDT role from its canonical byte-string name
    /// (e.g. `b"ESDTRoleLocalMint"`).
    ///
    /// This is the primary decoding path for on-chain ESDT role data. Unknown
    /// byte strings map to [`EsdtLocalRole::None`].
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
