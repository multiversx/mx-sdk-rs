use super::EsdtLocalRole;
use bitflags::bitflags;

bitflags! {
    /// Bit-flag representation of ESDT local roles.
    ///
    /// Each flag corresponds to a `Role*` constant defined in the Go VM at
    /// `vmhost/vmhooks/eei_helpers.go` (`mx-chain-vm-go`), where the same
    /// values are produced via `1 << iota` starting from `RoleMint = 1`.
    ///
    /// Correspondence table (Rust flag → Go constant → bit):
    ///
    /// | Rust flag              | Go constant             | Bit |
    /// |------------------------|--------------------------|----|
    /// | `MINT`                 | `RoleMint`               |  0 |
    /// | `BURN`                 | `RoleBurn`               |  1 |
    /// | `NFT_CREATE`           | `RoleNFTCreate`          |  2 |
    /// | `NFT_ADD_QUANTITY`     | `RoleNFTAddQuantity`     |  3 |
    /// | `NFT_BURN`             | `RoleNFTBurn`            |  4 |
    /// | `NFT_UPDATE_ATTRIBUTES`| `RoleNFTUpdateAttributes`|  5 |
    /// | `NFT_ADD_URI`          | `RoleNFTAddURI`          |  6 |
    /// | `NFT_RECREATE`         | `RoleNFTRecreate`        |  7 |
    /// | `MODIFY_CREATOR`       | `RoleModifyCreator`      |  8 |
    /// | `MODIFY_ROYALTIES`     | `RoleModifyRoyalties`    |  9 |
    /// | `SET_NEW_URI`          | `RoleSetNewURI`          | 10 |
    /// | `TRANSFER`             | *(not yet in Go VM)*     | 11 |
    #[derive(PartialEq, Clone, Copy, Debug)]
    pub struct EsdtLocalRoleFlags: u64 {
        const NONE                  = 0b00000000_00000000;
        const MINT                  = 0b00000000_00000001;
        const BURN                  = 0b00000000_00000010;
        const NFT_CREATE            = 0b00000000_00000100;
        const NFT_ADD_QUANTITY      = 0b00000000_00001000;
        const NFT_BURN              = 0b00000000_00010000;
        const NFT_UPDATE_ATTRIBUTES = 0b00000000_00100000;
        const NFT_ADD_URI           = 0b00000000_01000000;
        const NFT_RECREATE          = 0b00000000_10000000;
        const MODIFY_CREATOR        = 0b00000001_00000000;
        const MODIFY_ROYALTIES      = 0b00000010_00000000;
        const SET_NEW_URI           = 0b00000100_00000000;
        //TODO: check this flag after barnard
        const TRANSFER              = 0b00001000_00000000;
    }
}

impl EsdtLocalRoleFlags {
    /// Returns `true` if this flag set contains the bit corresponding to `role`.
    pub fn has_role(&self, role: &EsdtLocalRole) -> bool {
        *self & role.to_flag() != EsdtLocalRoleFlags::NONE
    }

    /// Iterates over all [`EsdtLocalRole`] variants whose bit is set in this flag set.
    ///
    /// Roles are yielded in canonical numeric-ID order (see [`EsdtLocalRole::iter_all`]).
    pub fn iter_roles(&self) -> impl Iterator<Item = &EsdtLocalRole> {
        EsdtLocalRole::iter_all().filter(move |role| self.has_role(role))
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use alloc::vec::Vec;

    #[test]
    fn test_flags_has_role() {
        let flags = EsdtLocalRoleFlags::MINT;
        assert!(flags.has_role(&EsdtLocalRole::Mint));
        let flags = EsdtLocalRoleFlags::MINT | EsdtLocalRoleFlags::BURN;
        assert!(flags.has_role(&EsdtLocalRole::Mint));
        let flags = EsdtLocalRoleFlags::NONE;
        assert!(!flags.has_role(&EsdtLocalRole::Mint));
        let flags = EsdtLocalRoleFlags::BURN;
        assert!(!flags.has_role(&EsdtLocalRole::Mint));
    }

    #[test]
    fn test_flags_iter_role() {
        let flags = EsdtLocalRoleFlags::MINT;
        assert_eq!(
            flags.iter_roles().collect::<Vec<&EsdtLocalRole>>(),
            alloc::vec![&EsdtLocalRole::Mint],
        );

        let flags = EsdtLocalRoleFlags::MINT | EsdtLocalRoleFlags::BURN;
        assert_eq!(
            flags.iter_roles().collect::<Vec<&EsdtLocalRole>>(),
            alloc::vec![&EsdtLocalRole::Mint, &EsdtLocalRole::Burn],
        );

        let flags = EsdtLocalRoleFlags::NONE;
        assert_eq!(
            flags.iter_roles().collect::<Vec<&EsdtLocalRole>>(),
            Vec::<&EsdtLocalRole>::new(),
        );
    }
}
