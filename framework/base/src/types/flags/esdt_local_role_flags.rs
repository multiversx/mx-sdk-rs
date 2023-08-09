#![allow(clippy::bad_bit_mask)]

use super::EsdtLocalRole;
use bitflags::bitflags;

bitflags! {
    pub struct EsdtLocalRoleFlags: u64 {
        const NONE                  = 0b00000000;
        const MINT                  = 0b00000001;
        const BURN                  = 0b00000010;
        const NFT_CREATE            = 0b00000100;
        const NFT_ADD_QUANTITY      = 0b00001000;
        const NFT_BURN              = 0b00010000;
        const NFT_ADD_URI           = 0b00100000;
        const NFT_UPDATE_ATTRIBUTES = 0b01000000;
        const TRANSFER              = 0b10000000;
    }
}

impl EsdtLocalRoleFlags {
    pub fn has_role(&self, role: &EsdtLocalRole) -> bool {
        *self & role.to_flag() != EsdtLocalRoleFlags::NONE
    }

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
