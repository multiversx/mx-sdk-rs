use multiversx_sdk::crypto::private_key::{PRIVATE_KEY_LENGTH, PrivateKey, SEED_LENGTH};

// Alice's well-known key material (consistent with wallet_pem_test.rs).
const ALICE_SEED_HEX: &str = "413f42575f7f26fad3317a778771212fdb80245850981e48b58a4f25e344e8f9";
const ALICE_PUBLIC_KEY_HEX: &str =
    "0139472eff6886771a982f3083da5d421f24c29181e63888228dc81ca60d69e1";

fn alice_seed_bytes() -> Vec<u8> {
    hex::decode(ALICE_SEED_HEX).unwrap()
}

fn alice_64byte_hex() -> String {
    format!("{}{}", ALICE_SEED_HEX, ALICE_PUBLIC_KEY_HEX)
}

// ---------------------------------------------------------------------------
// from_bytes — 32-byte (seed) branch
// ---------------------------------------------------------------------------

#[test]
fn test_from_bytes_seed_length_ok() {
    let seed = alice_seed_bytes();
    assert_eq!(seed.len(), SEED_LENGTH);

    let pk = PrivateKey::from_seed_bytes(seed.as_slice().try_into().unwrap());

    // to_seed_hex() encodes only the first 32 bytes (the seed).
    assert_eq!(pk.to_seed_hex(), ALICE_SEED_HEX);
}

#[test]
fn test_from_bytes_seed_derives_correct_public_key() {
    let seed = alice_seed_bytes();
    let pk = PrivateKey::from_seed_bytes(seed.as_slice().try_into().unwrap());

    // The upper 32 bytes of the stored key must be the verifying key.
    let stored = pk.to_bytes();
    let derived_pubkey_hex = hex::encode(&stored[32..]);
    assert_eq!(derived_pubkey_hex, ALICE_PUBLIC_KEY_HEX);
}

#[test]
fn test_from_bytes_seed_roundtrip() {
    let seed = alice_seed_bytes();
    let pk = PrivateKey::from_seed_bytes(seed.as_slice().try_into().unwrap());

    // Re-constructing from the returned 64-byte array must yield an equal key.
    let pk2 = PrivateKey::from_keypair_bytes(&pk.to_bytes()).unwrap();
    assert_eq!(pk, pk2);
}

// ---------------------------------------------------------------------------
// from_bytes — 64-byte (full key) branch
// ---------------------------------------------------------------------------

#[test]
fn test_from_bytes_full_length_ok() {
    let full = hex::decode(alice_64byte_hex()).unwrap();
    assert_eq!(full.len(), PRIVATE_KEY_LENGTH);

    let pk = PrivateKey::from_keypair_bytes(full.as_slice().try_into().unwrap()).unwrap();

    // Seed portion must be preserved verbatim.
    assert_eq!(pk.to_seed_hex(), ALICE_SEED_HEX);
}

#[test]
fn test_from_bytes_full_preserves_all_64_bytes() {
    let full = hex::decode(alice_64byte_hex()).unwrap();
    let pk = PrivateKey::from_keypair_bytes(full.as_slice().try_into().unwrap()).unwrap();

    assert_eq!(pk.to_bytes().as_slice(), full.as_slice());
}

#[test]
fn test_from_bytes_full_roundtrip() {
    let full = hex::decode(alice_64byte_hex()).unwrap();
    let pk = PrivateKey::from_keypair_bytes(full.as_slice().try_into().unwrap()).unwrap();
    let pk2 = PrivateKey::from_keypair_bytes(&pk.to_bytes()).unwrap();
    assert_eq!(pk, pk2);
}

// ---------------------------------------------------------------------------
// from_bytes — error cases
// ---------------------------------------------------------------------------

#[test]
fn test_from_bytes_empty_is_err() {
    assert!(PrivateKey::from_bytes(&[]).is_err());
}

#[test]
fn test_from_bytes_wrong_length_is_err() {
    // 16 bytes — neither 32 nor 64.
    assert!(PrivateKey::from_bytes(&[0u8; 16]).is_err());
}

#[test]
fn test_from_bytes_65_bytes_is_err() {
    assert!(PrivateKey::from_bytes(&[0u8; 65]).is_err());
}
