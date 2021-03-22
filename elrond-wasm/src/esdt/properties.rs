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
