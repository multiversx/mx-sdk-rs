use hex::FromHex;
use multiversx_chain_vm::crypto_functions;

#[test]
fn test_verify_ed25519_basic() {
    let public_key: &[u8] = b"832c7d42d05a69647887961552c172b5dd3da1b253ca823d96df2576bf218c61";
    let message: &[u8] = b"033085998ec7c0fa980ed924ab6ea6bf465053d896040aa0f9887ccbca7ecdac";
    let signature: &[u8] = b"40e03623649f6929908a1051d99ef95907f405ce4c04c99598e67373d2e88be03c59ce36d767255e290b91bdd174e71803a222b74d8c138b4650f1e70c26930c";

    let pub_bytes: Vec<u8> = FromHex::from_hex(public_key).unwrap();
    let msg_bytes: Vec<u8> = FromHex::from_hex(message).unwrap();
    let sig_bytes: Vec<u8> = FromHex::from_hex(signature).unwrap();

    let success = crypto_functions::verify_ed25519(&pub_bytes, &msg_bytes, &sig_bytes);
    assert!(success);
}

#[test]
fn test_verify_ed25519_bad_sig() {
    let public_key: &[u8] = b"832c7d42d05a69647887961552c172b5dd3da1b253ca823d96df2576bf218c61";
    let message: &[u8] = b"033085998ec7c0fa980ed924ab6ea6bf465053d896040aa0f9887ccbca7ecdac";
    let signature: &[u8] = b"40e03623649f6929908a1051d99ef95907f405ce4c04c99598e67373d2e88be03c59ce36d767255e290b91bdd174e71803a222b74d8c138b4650f1e70c26930d";

    let pub_bytes: Vec<u8> = FromHex::from_hex(public_key).unwrap();
    let msg_bytes: Vec<u8> = FromHex::from_hex(message).unwrap();
    let sig_bytes: Vec<u8> = FromHex::from_hex(signature).unwrap();

    let success = crypto_functions::verify_ed25519(&pub_bytes, &msg_bytes, &sig_bytes);
    assert!(!success);
}

#[test]
fn test_verify_ed25519_invalid_args() {
    let public_key: &[u8] = b"832c7d42d05a69647887961552c172b5dd3da1b253ca823d96df2576bf218c";
    let message: &[u8] = b"033085998ec7c0fa980ed924ab6ea6bf465053d896040aa0f9887ccbca7ecd";
    let signature: &[u8] = b"40e03623649f6929908a1051d99ef95907f405ce4c04c99598e67373d2e88be03c59ce36d767255e290b91bdd174e71803a222b74d8c138b4650f1e70c2693";

    let pub_bytes: Vec<u8> = FromHex::from_hex(public_key).unwrap();
    let msg_bytes: Vec<u8> = FromHex::from_hex(message).unwrap();
    let sig_bytes: Vec<u8> = FromHex::from_hex(signature).unwrap();

    let success = crypto_functions::verify_ed25519(&pub_bytes, &msg_bytes, &sig_bytes);
    assert!(!success);
}
