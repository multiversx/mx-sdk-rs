use multiversx_sdk::wallet::PublicKey;

// Alice's known values, consistent with wallet_pem_test.rs constants.
const ALICE_PUBLIC_KEY_HEX: &str =
    "0139472eff6886771a982f3083da5d421f24c29181e63888228dc81ca60d69e1";
const ALICE_BECH32: &str = "erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th";

// ---------------------------------------------------------------------------
// from_hex_str
// ---------------------------------------------------------------------------

#[test]
fn test_from_hex_str_valid() {
    let pk = PublicKey::from_hex_str(ALICE_PUBLIC_KEY_HEX).unwrap();
    assert_eq!(pk.to_hex(), ALICE_PUBLIC_KEY_HEX);
}

#[test]
fn test_from_hex_str_invalid_hex() {
    assert!(PublicKey::from_hex_str("not_valid_hex!!").is_err());
}

#[test]
fn test_from_hex_str_too_short() {
    // 8 bytes — not 32
    assert!(PublicKey::from_hex_str("0102030405060708").is_err());
}

#[test]
fn test_from_hex_str_too_long() {
    // 64 bytes (128 hex chars) — not 32
    let long_hex = "01".repeat(64);
    assert!(PublicKey::from_hex_str(&long_hex).is_err());
}

// ---------------------------------------------------------------------------
// to_hex
// ---------------------------------------------------------------------------

#[test]
fn test_to_hex_matches_input() {
    let pk = PublicKey::from_hex_str(ALICE_PUBLIC_KEY_HEX).unwrap();
    assert_eq!(pk.to_hex(), ALICE_PUBLIC_KEY_HEX);
}

#[test]
fn test_to_hex_from_hex_roundtrip() {
    let pk = PublicKey::from_hex_str(ALICE_PUBLIC_KEY_HEX).unwrap();
    let pk2 = PublicKey::from_hex_str(&pk.to_hex()).unwrap();
    assert_eq!(pk.to_bytes(), pk2.to_bytes());
}

// ---------------------------------------------------------------------------
// to_address
// ---------------------------------------------------------------------------

#[test]
fn test_to_address_bytes_match_public_key() {
    // On MultiversX the address IS the public key bytes.
    let pk = PublicKey::from_hex_str(ALICE_PUBLIC_KEY_HEX).unwrap();
    let address = pk.to_address();
    assert_eq!(address.to_hex(), ALICE_PUBLIC_KEY_HEX);
}

#[test]
fn test_to_address_bech32() {
    let pk = PublicKey::from_hex_str(ALICE_PUBLIC_KEY_HEX).unwrap();
    let address = pk.to_address();
    assert_eq!(address.to_bech32_default().to_bech32_str(), ALICE_BECH32);
}
