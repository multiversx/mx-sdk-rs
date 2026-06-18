use multiversx_chain_core::std::Bech32Hrp;
use multiversx_sdk::wallet::{Wallet, WalletPem, WalletSource};

const ALICE_PEM_PATH: &str = "tests/alice.pem";
const ALICE_BECH32: &str = "erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th";
const ALICE_PRIVATE_KEY_HEX: &str =
    "413f42575f7f26fad3317a778771212fdb80245850981e48b58a4f25e344e8f9";
const ALICE_PUBLIC_KEY_HEX: &str =
    "0139472eff6886771a982f3083da5d421f24c29181e63888228dc81ca60d69e1";

// ---------------------------------------------------------------------------
// WalletPem parsing
// ---------------------------------------------------------------------------

#[test]
fn test_pem_from_file_address() {
    let pem = WalletPem::from_pem_file(ALICE_PEM_PATH).unwrap();
    assert_eq!(pem.address.to_bech32_str(), ALICE_BECH32);
}

#[test]
fn test_pem_from_file_private_key() {
    let pem = WalletPem::from_pem_file(ALICE_PEM_PATH).unwrap();
    assert_eq!(pem.private_key_hex(), ALICE_PRIVATE_KEY_HEX);
}

#[test]
fn test_pem_from_file_public_key() {
    let pem = WalletPem::from_pem_file(ALICE_PEM_PATH).unwrap();
    assert_eq!(pem.public_key_hex(), ALICE_PUBLIC_KEY_HEX);
}

#[test]
fn test_pem_from_str_parses_correctly() {
    let pem_str = std::fs::read_to_string(ALICE_PEM_PATH).unwrap();
    let pem = WalletPem::from_pem_str(&pem_str).unwrap();
    assert_eq!(pem.address.to_bech32_str(), ALICE_BECH32);
    assert_eq!(pem.private_key_hex(), ALICE_PRIVATE_KEY_HEX);
    assert_eq!(pem.public_key_hex(), ALICE_PUBLIC_KEY_HEX);
}

// ---------------------------------------------------------------------------
// WalletPem serialization
// ---------------------------------------------------------------------------

#[test]
fn test_pem_str_roundtrip() {
    let pem = WalletPem::from_pem_file(ALICE_PEM_PATH).unwrap();
    let serialized = pem.to_pem_str();
    let reparsed = WalletPem::from_pem_str(&serialized).unwrap();
    assert_eq!(reparsed.private_key_hex(), ALICE_PRIVATE_KEY_HEX);
    assert_eq!(reparsed.address.to_bech32_str(), ALICE_BECH32);
}

// ---------------------------------------------------------------------------
// WalletPem <-> Wallet conversion
// ---------------------------------------------------------------------------

#[test]
fn test_wallet_from_pem_fields() {
    let pem = WalletPem::from_pem_file(ALICE_PEM_PATH).unwrap();
    let wallet = Wallet::from(pem);
    assert_eq!(wallet.private_key.to_seed_hex(), ALICE_PRIVATE_KEY_HEX);
    assert_eq!(wallet.address.to_bech32_default().bech32, ALICE_BECH32);
}

#[test]
fn test_wallet_from_pem_source() {
    let pem = WalletPem::from_pem_file(ALICE_PEM_PATH).unwrap();
    let hrp = pem.address.hrp;
    let wallet = Wallet::from(pem);
    assert_eq!(wallet.source, WalletSource::PemFile(hrp));
}

#[test]
fn test_wallet_to_pem_roundtrip() {
    let wallet = Wallet::from_pem_file(ALICE_PEM_PATH).unwrap();
    let hrp = Bech32Hrp::try_from("erd").unwrap();
    let pem = wallet.to_pem(hrp);
    assert_eq!(pem.private_key_hex(), ALICE_PRIVATE_KEY_HEX);
    assert_eq!(pem.address.to_bech32_str(), ALICE_BECH32);
}

#[test]
fn test_wallet_to_pem_str_roundtrip() {
    let wallet = Wallet::from_pem_file(ALICE_PEM_PATH).unwrap();
    let hrp = Bech32Hrp::try_from("erd").unwrap();
    let pem_str = wallet.to_pem(hrp).to_pem_str();
    let reparsed = Wallet::from(WalletPem::from_pem_str(&pem_str).unwrap());
    assert_eq!(reparsed.private_key.to_seed_hex(), ALICE_PRIVATE_KEY_HEX);
}
