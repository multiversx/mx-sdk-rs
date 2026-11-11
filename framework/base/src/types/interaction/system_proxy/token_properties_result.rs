use alloc::string::String;
use multiversx_chain_core::types::{Address, EsdtTokenType};
use multiversx_sc_codec::{DecodeErrorHandler, TopDecodeMulti, TopDecodeMultiInput};

/// The decoded result of the `getTokenProperties` view of the ESDT system SC.
///
/// The system SC returns, in order:
/// - token name,
/// - token type name (e.g. `FungibleESDT`, `DynamicNonFungibleESDT`),
/// - owner address,
/// - minted value (base 10 string),
/// - burnt value (base 10 string),
/// - a list of `Name-value` properties (e.g. `NumDecimals-18`, `IsPaused-false`).
#[derive(Debug, Clone)]
pub struct TokenPropertiesResult {
    pub token_name: String,
    pub token_type: EsdtTokenType,
    pub owner_address: Address,
    /// Total minted value, as a base 10 string.
    pub minted_value: String,
    /// Total burnt value, as a base 10 string.
    pub burnt_value: String,
    pub num_decimals: usize,
    pub is_paused: bool,
    pub can_upgrade: bool,
    pub can_mint: bool,
    pub can_burn: bool,
    pub can_change_owner: bool,
    pub can_pause: bool,
    pub can_freeze: bool,
    pub can_wipe: bool,
    pub can_add_special_roles: bool,
    pub can_transfer_nft_create_role: bool,
    pub nft_create_stopped: bool,
    pub num_wiped: usize,
}

impl TokenPropertiesResult {
    fn fetch_struct_field(&mut self, input: &str) {
        let Some((name, value)) = input.split_once('-') else {
            return;
        };
        let is_true = value == "true";

        match name {
            "NumDecimals" => self.num_decimals = parse_usize(value),
            "IsPaused" => self.is_paused = is_true,
            "CanUpgrade" => self.can_upgrade = is_true,
            "CanMint" => self.can_mint = is_true,
            "CanBurn" => self.can_burn = is_true,
            "CanChangeOwner" => self.can_change_owner = is_true,
            "CanPause" => self.can_pause = is_true,
            "CanFreeze" => self.can_freeze = is_true,
            "CanWipe" => self.can_wipe = is_true,
            "CanAddSpecialRoles" => self.can_add_special_roles = is_true,
            "CanTransferNFTCreateRole" => self.can_transfer_nft_create_role = is_true,
            "NFTCreateStopped" => self.nft_create_stopped = is_true,
            "NumWiped" => self.num_wiped = parse_usize(value),
            _ => {}
        }
    }
}

impl TopDecodeMulti for TokenPropertiesResult {
    fn multi_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeMultiInput,
        H: DecodeErrorHandler,
    {
        let token_name = String::multi_decode_or_handle_err(input, h)?;
        let token_type_name = String::multi_decode_or_handle_err(input, h)?;
        let owner_address = Address::multi_decode_or_handle_err(input, h)?;
        let minted_value = String::multi_decode_or_handle_err(input, h)?;
        let burnt_value = String::multi_decode_or_handle_err(input, h)?;

        let mut token_properties = TokenPropertiesResult {
            token_name,
            token_type: EsdtTokenType::collection_from_string(token_type_name.as_str()),
            owner_address,
            minted_value,
            burnt_value,
            num_decimals: 0,
            is_paused: false,
            can_upgrade: false,
            can_mint: false,
            can_burn: false,
            can_change_owner: false,
            can_pause: false,
            can_freeze: false,
            can_wipe: false,
            can_add_special_roles: false,
            can_transfer_nft_create_role: false,
            nft_create_stopped: false,
            num_wiped: 0,
        };

        while input.has_next() {
            let property = String::multi_decode_or_handle_err(input, h)?;
            token_properties.fetch_struct_field(property.as_str());
        }

        Ok(token_properties)
    }
}

fn parse_usize(input: &str) -> usize {
    input.parse::<usize>().unwrap_or(0)
}
