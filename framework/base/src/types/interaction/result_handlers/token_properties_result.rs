use multiversx_sc_codec::{
    DecodeErrorHandler, TopDecodeInput, TopDecodeMulti, TopDecodeMultiInput,
};

// const MAX_BUFFER_SIZE: usize = 62;

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
        if let Ok(string) = core::str::from_utf8(input) {
            if let Some((key, value)) = string.split_once('-') {
                match key {
                    "NumDecimals" => {
                        if let Ok(parsed) = value.parse::<usize>() {
                            self.num_decimals = parsed;
                        }
                    },
                    "IsPaused" => self.is_paused = value == "true",
                    "CanUpgrade" => self.can_upgrade = value == "true",
                    "CanMint" => self.can_mint = value == "true",
                    "CanBurn" => self.can_burn = value == "true",
                    "CanChangeOwner" => self.can_change_owner = value == "true",
                    "CanPause" => self.can_pause = value == "true",
                    "CanFreeze" => self.can_freeze = value == "true",
                    "CanWipe" => self.can_wipe = value == "true",
                    "CanAddSpecialRoles" => self.can_add_special_roles = value == "true",
                    "CanTransferNFTCreateRole" => {
                        self.can_transfer_nft_create_role = value == "true"
                    },
                    "NFTCreateStopped" => self.nft_create_stopped = value == "true",
                    "NumWiped" => {
                        if let Ok(parsed) = value.parse::<usize>() {
                            self.num_wiped = parsed;
                        }
                    },
                    _ => {},
                }
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
            token_properties.fetch_struct_field(&input.next_value_input(h)?.into_boxed_slice_u8());
        }

        Ok(token_properties)
    }
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
    assert!(!token_properties.can_freeze);
    assert!(token_properties.can_burn)
}
