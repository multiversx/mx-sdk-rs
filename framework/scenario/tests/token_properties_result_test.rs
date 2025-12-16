use multiversx_sc::{
    codec::TopDecodeMulti,
    imports::{Bech32Address, TokenPropertiesResult},
    types::EsdtTokenType,
};

fn hex_decode(hex: &[u8]) -> Option<Vec<u8>> {
    if !hex.len().is_multiple_of(2) {
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

#[test]
fn multi_decode_from_api_response() {
    let raw_input = b"@544553544e4654@44796e616d69634e6f6e46756e6769626c6545534454@0139472eff6886771a982f3083da5d421f24c29181e63888228dc81ca60d69e1@30@30@4e756d446563696d616c732d3138@49735061757365642d66616c7365@43616e557067726164652d74727565@43616e4d696e742d66616c7365@43616e4275726e2d74727565@43616e4368616e67654f776e65722d66616c7365@43616e50617573652d66616c7365@43616e467265657a652d66616c7365@43616e576970652d66616c7365@43616e4164645370656369616c526f6c65732d74727565@43616e5472616e736665724e4654437265617465526f6c652d66616c7365@4e465443726561746553746f707065642d66616c7365@4e756d57697065642d30";
    let mut input: Vec<Vec<u8>> = raw_input
        .split(|&b| b == b'@')
        .filter(|slice| !slice.is_empty())
        .map(|slice| hex_decode(slice).unwrap())
        .collect();

    let token_properties = TokenPropertiesResult::multi_decode(&mut input).unwrap();
    assert!(token_properties.token_name == "TESTNFT");
    assert!(token_properties.token_type == EsdtTokenType::DynamicNFT);
    assert!(
        Bech32Address::from(token_properties.owner_address).to_bech32_str()
            == "erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th"
    );
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
