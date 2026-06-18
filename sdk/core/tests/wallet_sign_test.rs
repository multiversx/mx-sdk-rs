use multiversx_sdk::{test_wallets, wallet::WalletSignature};

#[test]
fn test_sign_bytes_snapshot() {
    let alice = test_wallets::alice();
    let bob = test_wallets::bob();

    // Signatures are deterministic: same key + same message always produces the same bytes.
    let sig_alice = alice.sign_bytes(b"hello multiversx");
    let sig_bob = bob.sign_bytes(b"hello multiversx");

    assert_eq!(sig_alice.to_hex(), SIG_ALICE_HELLO);
    assert_eq!(sig_bob.to_hex(), SIG_BOB_HELLO);
}

#[test]
fn test_sign_bytes_deterministic() {
    let alice = test_wallets::alice();
    assert_eq!(
        alice.sign_bytes(b"deterministic"),
        alice.sign_bytes(b"deterministic"),
    );
}

#[test]
fn test_sign_bytes_different_messages_differ() {
    let alice = test_wallets::alice();
    assert_ne!(
        alice.sign_bytes(b"message one"),
        alice.sign_bytes(b"message two"),
    );
}

#[test]
fn test_sign_bytes_different_keys_differ() {
    assert_ne!(
        test_wallets::alice().sign_bytes(b"same message"),
        test_wallets::bob().sign_bytes(b"same message"),
    );
}

#[test]
fn test_verify_valid_signature() {
    let alice = test_wallets::alice();
    let message = b"hello multiversx";
    let sig = alice.sign_bytes(message);
    assert!(alice.public_key().verify(message, &sig));
}

#[test]
fn test_verify_wrong_message_fails() {
    let alice = test_wallets::alice();
    let sig = alice.sign_bytes(b"hello multiversx");
    assert!(!alice.public_key().verify(b"different message", &sig));
}

#[test]
fn test_verify_wrong_key_fails() {
    let alice = test_wallets::alice();
    let bob = test_wallets::bob();
    let sig = alice.sign_bytes(b"hello multiversx");
    assert!(!bob.public_key().verify(b"hello multiversx", &sig));
}

#[test]
fn test_verify_snapshot_signatures() {
    let alice = test_wallets::alice();
    let bob = test_wallets::bob();
    let alice_sig = WalletSignature::from_hex_str(SIG_ALICE_HELLO).unwrap();
    let bob_sig = WalletSignature::from_hex_str(SIG_BOB_HELLO).unwrap();
    assert!(alice.public_key().verify(b"hello multiversx", &alice_sig));
    assert!(bob.public_key().verify(b"hello multiversx", &bob_sig));
}

// Snapshots captured with fixed test-wallet keys.
const SIG_ALICE_HELLO: &str = "c32f811c809a02d3548f190b134d5fa542b028935f4e300556e29b3164f146d54aa7e37ba97080e9f72402af67f6f6f070cbbd1b496c9779e75038a6aee54c07";
const SIG_BOB_HELLO: &str = "09e1b11a87e47bb0c1bebc71f653daabb966c583219588d4b80e396a966b91d41767af8f4704bbc50f8e8fd34cc6fb0da210329ea19437a7e07c0fc3f5cdf50e";

#[test]
fn test_wallet_signature_to_hex_roundtrip() {
    let raw = [0xabu8; 64];
    let sig = WalletSignature::from_bytes(raw);
    let hex = sig.to_hex();
    assert_eq!(hex.len(), 128);
    let sig2 = WalletSignature::from_hex_str(&hex).unwrap();
    assert_eq!(sig2, sig);
}

#[test]
fn test_wallet_signature_from_hex_str_invalid_hex() {
    assert!(WalletSignature::from_hex_str("not-hex").is_err());
}

#[test]
fn test_wallet_signature_from_hex_str_wrong_length() {
    // 63 bytes (126 hex chars) — too short
    assert!(WalletSignature::from_hex_str(&"ab".repeat(63)).is_err());
    // 65 bytes (130 hex chars) — too long
    assert!(WalletSignature::from_hex_str(&"ab".repeat(65)).is_err());
}

#[test]
fn test_wallet_signature_json_serialize_as_hex() {
    let raw = [0x01u8; 64];
    let sig = WalletSignature::from_bytes(raw);
    let json = serde_json::to_string(&sig).unwrap();
    // JSON value should be a quoted hex string of length 128 chars
    assert_eq!(json, format!("\"{}\"", sig.to_hex()));
}

#[test]
fn test_wallet_signature_json_deserialize_from_hex() {
    let hex = "ab".repeat(64);
    let json = format!("\"{}\"", hex);
    let sig: WalletSignature = serde_json::from_str(&json).unwrap();
    assert_eq!(sig.to_hex(), hex);
}

#[test]
fn test_wallet_signature_json_roundtrip() {
    let raw = [0xdeu8; 64];
    let sig = WalletSignature::from_bytes(raw);
    let json = serde_json::to_string(&sig).unwrap();
    let sig2: WalletSignature = serde_json::from_str(&json).unwrap();
    assert_eq!(sig, sig2);
}
