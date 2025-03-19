use multiversx_sc_codec::{
    DecodeErrorHandler, TopDecodeInput, TopDecodeMulti, TopDecodeMultiInput,
};

const MAX_BUFFER_SIZE: usize = 62;

const PREFIXES: &[&[u8]] = &[
    b"NumDecimals-",
    b"IsPaused-",
    b"CanUpgrade-",
    b"CanMint-",
    b"CanBurn-",
    b"CanChangeOwner-",
    b"CanPause-",
    b"CanFreeze-",
    b"CanWipe-",
    b"CanAddSpecialRoles-",
    b"CanTransferNFTCreateRole-",
    b"NFTCreateStopped-",
    b"NumWiped-",
];

#[derive(Debug, Clone, Default)]
pub struct TokenPropertiesResult {
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
    fn fetch_struct_field(&mut self, input: &[u8]) {
        let is_true = |value: &[u8]| value == b"true";

        if let Some((prefix, value)) = PREFIXES
            .iter()
            .find_map(|&prefix| input.strip_prefix(prefix).map(|value| (prefix, value)))
        {
            match prefix {
                b"NumDecimals-" => self.num_decimals = parse_usize(value),
                b"IsPaused-" => self.is_paused = is_true(value),
                b"CanUpgrade-" => self.can_upgrade = is_true(value),
                b"CanMint-" => self.can_mint = is_true(value),
                b"CanBurn-" => self.can_burn = is_true(value),
                b"CanChangeOwner-" => self.can_change_owner = is_true(value),
                b"CanPause-" => self.can_pause = is_true(value),
                b"CanFreeze-" => self.can_freeze = is_true(value),
                b"CanWipe-" => self.can_wipe = is_true(value),
                b"CanAddSpecialRoles-" => self.can_add_special_roles = is_true(value),
                b"CanTransferNFTCreateRole-" => self.can_transfer_nft_create_role = is_true(value),
                b"NFTCreateStopped-" => self.nft_create_stopped = is_true(value),
                b"NumWiped-" => self.num_wiped = parse_usize(value),
                _ => {},
            }
        }
    }
}

impl TopDecodeMulti for TokenPropertiesResult {
    fn multi_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeMultiInput,
        H: DecodeErrorHandler,
    {
        let mut token_properties = TokenPropertiesResult::default();
        while input.has_next() {
            let value = input.next_value_input(h)?;
            let mut buffer = [0u8; MAX_BUFFER_SIZE];
            let len = value.into_max_size_buffer_align_right(&mut buffer, h)?;
            token_properties.fetch_struct_field(&buffer[MAX_BUFFER_SIZE - len..]);
        }

        Ok(token_properties)
    }
}

fn parse_usize(input: &[u8]) -> usize {
    core::str::from_utf8(input)
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(0)
}
