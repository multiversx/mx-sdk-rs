use multiversx_sc_codec::{
    DecodeErrorHandler, TopDecodeInput, TopDecodeMulti, TopDecodeMultiInput,
};

const MAX_BUFFER_SIZE: usize = 62;

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

        if let Some(value) = input.strip_prefix(b"NumDecimals-") {
            self.num_decimals = parse_usize(value);
        } else if let Some(value) = input.strip_prefix(b"IsPaused-") {
            self.is_paused = is_true(value);
        } else if let Some(value) = input.strip_prefix(b"CanUpgrade-") {
            self.can_upgrade = is_true(value);
        } else if let Some(value) = input.strip_prefix(b"CanMint-") {
            self.can_mint = is_true(value);
        } else if let Some(value) = input.strip_prefix(b"CanBurn-") {
            self.can_burn = is_true(value);
        } else if let Some(value) = input.strip_prefix(b"CanChangeOwner-") {
            self.can_change_owner = is_true(value);
        } else if let Some(value) = input.strip_prefix(b"CanPause-") {
            self.can_pause = is_true(value);
        } else if let Some(value) = input.strip_prefix(b"CanFreeze-") {
            self.can_freeze = is_true(value);
        } else if let Some(value) = input.strip_prefix(b"CanWipe-") {
            self.can_wipe = is_true(value);
        } else if let Some(value) = input.strip_prefix(b"CanAddSpecialRoles-") {
            self.can_add_special_roles = is_true(value);
        } else if let Some(value) = input.strip_prefix(b"CanTransferNFTCreateRole-") {
            self.can_transfer_nft_create_role = is_true(value);
        } else if let Some(value) = input.strip_prefix(b"NFTCreateStopped-") {
            self.nft_create_stopped = is_true(value);
        } else if let Some(value) = input.strip_prefix(b"NumWiped-") {
            self.num_wiped = parse_usize(value);
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

#[test]
fn decode_test() {
    use multiversx_sc_codec::Vec;

    fn hex_decode(hex: &[u8]) -> Option<Vec<u8>> {
        if hex.len() % 2 != 0 {
            return None;
        }

        let mut bytes = Vec::new();
        for chunk in hex.chunks(2) {
            if let Ok(hex_str) = core::str::from_utf8(chunk) {
                if let Ok(byte) = u8::from_str_radix(hex_str, 16) {
                    bytes.push(byte);
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }

        Some(bytes)
    }

    let raw_input = b"@6f6b@544553544e4654@44796e616d69634e6f6e46756e6769626c6545534454@0139472eff6886771a982f3083da5d421f24c29181e63888228dc81ca60d69e1@30@30@4e756d446563696d616c732d3138@49735061757365642d66616c7365@43616e557067726164652d74727565@43616e4d696e742d66616c7365@43616e4275726e2d74727565@43616e4368616e67654f776e65722d66616c7365@43616e50617573652d66616c7365@43616e467265657a652d66616c7365@43616e576970652d66616c7365@43616e4164645370656369616c526f6c65732d74727565@43616e5472616e736665724e4654437265617465526f6c652d66616c7365@4e465443726561746553746f707065642d66616c7365@4e756d57697065642d30";
    let mut input: Vec<Vec<u8>> = raw_input
        .split(|&b| b == b'@')
        .filter(|slice| !slice.is_empty())
        .map(|slice| hex_decode(slice).unwrap())
        .collect();

    let token_properties = TokenPropertiesResult::multi_decode(&mut input).unwrap();

    assert!(token_properties.num_decimals == 18usize);
    assert!(!token_properties.is_paused);
    assert!(token_properties.can_upgrade);
    assert!(!token_properties.can_mint);
    assert!(token_properties.can_burn);
    assert!(!token_properties.can_change_owner);
    assert!(!token_properties.can_pause);
    assert!(!token_properties.can_freeze);
    assert!(!token_properties.can_wipe);
    assert!(token_properties.can_add_special_roles);
    assert!(!token_properties.can_transfer_nft_create_role);
    assert!(!token_properties.nft_create_stopped);
    assert!(token_properties.num_wiped == 0usize);
}
