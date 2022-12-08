pub struct TokenProperties {
    pub num_decimals: usize,
    pub can_freeze: bool,
    pub can_wipe: bool,
    pub can_pause: bool,
    pub can_transfer_create_role: bool,
    pub can_mint: bool,
    pub can_burn: bool,
    pub can_change_owner: bool,
    pub can_upgrade: bool,
    pub can_add_special_roles: bool,
}
pub struct FungibleTokenProperties {
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

pub struct NonFungibleTokenProperties {
    pub can_freeze: bool,
    pub can_wipe: bool,
    pub can_pause: bool,
    pub can_transfer_create_role: bool,
    pub can_change_owner: bool,
    pub can_upgrade: bool,
    pub can_add_special_roles: bool,
}

pub struct SemiFungibleTokenProperties {
    pub can_freeze: bool,
    pub can_wipe: bool,
    pub can_pause: bool,
    pub can_transfer_create_role: bool,
    pub can_change_owner: bool,
    pub can_upgrade: bool,
    pub can_add_special_roles: bool,
}

pub struct MetaTokenProperties {
    pub num_decimals: usize,
    pub can_freeze: bool,
    pub can_wipe: bool,
    pub can_pause: bool,
    pub can_transfer_create_role: bool,
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
            can_transfer_create_role: false,
            can_mint: false,
            can_burn: false,
            can_change_owner: true,
            can_upgrade: true,
            can_add_special_roles: true,
        }
    }
}

impl Default for FungibleTokenProperties {
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
            can_transfer_create_role: true,
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
            can_transfer_create_role: true,
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
            can_transfer_create_role: true,
            can_change_owner: true,
            can_upgrade: true,
            can_add_special_roles: true,
        }
    }
}

/// Represents property arguments to be sent to the system SC.
///
/// Fields set to None will not be mentioned at all. When upgrading tokens, this means leaving the old value in place.
#[derive(Default)]
pub struct TokenPropertyArguments {
    pub can_freeze: Option<bool>,
    pub can_wipe: Option<bool>,
    pub can_pause: Option<bool>,
    pub can_transfer_create_role: Option<bool>,
    pub can_mint: Option<bool>,
    pub can_burn: Option<bool>,
    pub can_change_owner: Option<bool>,
    pub can_upgrade: Option<bool>,
    pub can_add_special_roles: Option<bool>,
}
