use multiversx_sdk::test_wallets;

#[test]
fn test_sign_bytes_snapshot() {
    let alice = test_wallets::alice();
    let bob = test_wallets::bob();

    // Signatures are deterministic: same key + same message always produces the same bytes.
    let sig_alice = alice.sign_bytes(b"hello multiversx");
    let sig_bob = bob.sign_bytes(b"hello multiversx");

    assert_eq!(hex::encode(sig_alice), SIG_ALICE_HELLO);
    assert_eq!(hex::encode(sig_bob), SIG_BOB_HELLO);
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

// Snapshots captured with fixed test-wallet keys.
const SIG_ALICE_HELLO: &str = "c32f811c809a02d3548f190b134d5fa542b028935f4e300556e29b3164f146d54aa7e37ba97080e9f72402af67f6f6f070cbbd1b496c9779e75038a6aee54c07";
const SIG_BOB_HELLO: &str = "09e1b11a87e47bb0c1bebc71f653daabb966c583219588d4b80e396a966b91d41767af8f4704bbc50f8e8fd34cc6fb0da210329ea19437a7e07c0fc3f5cdf50e";
