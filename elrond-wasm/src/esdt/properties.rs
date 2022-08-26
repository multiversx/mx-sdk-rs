use elrond_codec::elrond_codec_derive::TopDecode;

pub struct TokenProperties {
    pub num_decimals: usize,
    pub can_freeze: bool,
    pub can_wipe: bool,
    pub can_pause: bool,
    pub can_mint: bool,
    pub can_burn: bool,
    pub can_change_owner: bool,
    pub can_upgrade: bool,
    pub can_add_special_roles: bool,
}

#[derive(TopDecode)]
pub enum Properties {
    CanFreeze,
    CanWipe,
    CanPause,
    CanMint,
    CanBurn,
    CanChangeOwner,
    CanUpgrade,
    CanAddSpecialRoles
}

pub type FungibleTokenProperties = TokenProperties;

pub struct NonFungibleTokenProperties {
    pub can_freeze: bool,
    pub can_wipe: bool,
    pub can_pause: bool,
    pub can_change_owner: bool,
    pub can_upgrade: bool,
    pub can_add_special_roles: bool,
}

pub struct SemiFungibleTokenProperties {
    pub can_freeze: bool,
    pub can_wipe: bool,
    pub can_pause: bool,
    pub can_change_owner: bool,
    pub can_upgrade: bool,
    pub can_add_special_roles: bool,
}

pub struct MetaTokenProperties {
    pub num_decimals: usize,
    pub can_freeze: bool,
    pub can_wipe: bool,
    pub can_pause: bool,
    pub can_change_owner: bool,
    pub can_upgrade: bool,
    pub can_add_special_roles: bool,
}

impl Default for TokenProperties {
    fn default() -> Self {
        Self {
            num_decimals: 0,
            can_freeze: true,
            can_wipe: true,
            can_pause: true,
            can_mint: false,
            can_burn: false,
            can_change_owner: true,
            can_upgrade: true,
            can_add_special_roles: true,
        }
    }
}

impl Default for NonFungibleTokenProperties {
    fn default() -> Self {
        Self {
            can_freeze: true,
            can_wipe: true,
            can_pause: true,
            can_change_owner: true,
            can_upgrade: true,
            can_add_special_roles: true,
        }
    }
}

impl Default for SemiFungibleTokenProperties {
    fn default() -> Self {
        Self {
            can_freeze: true,
            can_wipe: true,
            can_pause: true,
            can_change_owner: true,
            can_upgrade: true,
            can_add_special_roles: true,
        }
    }
}

impl Default for MetaTokenProperties {
    fn default() -> Self {
        Self {
            num_decimals: 0,
            can_freeze: true,
            can_wipe: true,
            can_pause: true,
            can_change_owner: true,
            can_upgrade: true,
            can_add_special_roles: true,
        }
    }
}

impl Properties {
    pub fn as_bytes(&self) -> &'static [u8] {
        match self {
            Properties::CanFreeze => b"canFreeze",
            Properties::CanWipe => b"canWipe",
            Properties::CanPause => b"canPause",
            Properties::CanMint => b"canMint",
            Properties::CanBurn => b"canBurn",
            Properties::CanChangeOwner => b"canChangeOwner",
            Properties::CanUpgrade => b"canUpgrade",
            Properties::CanAddSpecialRoles => b"canAddSpecialRoles",
        }
    }
}
