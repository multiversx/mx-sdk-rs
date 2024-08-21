use std::fs;

use multiversx_sc_meta::cli::{WalletAction, WalletArgs, WalletConvertArgs};
use multiversx_sc_meta::cmd::wallet::wallet;
use multiversx_sc_snippets::imports::Wallet;

const ALICE_PEM_PATH: &str = "../snippets/src/test_wallets/alice.pem";
const ALICE_KEYSTORE_PATH: &str = "alice.json";

// Insert a password for your keystore, followed by (CTRL+D) for Linux/Mac or (CTRL+Z) for Windows
fn create_keystore_from_pem() {
    let wallet_convert_args = WalletConvertArgs {
        infile: Some(ALICE_PEM_PATH.to_string()),
        outfile: Some(ALICE_KEYSTORE_PATH.to_string()),
        from: "pem".to_string(),
        to: "keystore-secret".to_string(),
    };
    let wallet_args = WalletArgs {
        command: WalletAction::Convert(wallet_convert_args),
    };
    wallet(&wallet_args);
}

// Insert "1234" as password for your keystore, followed by (CTRL+D) for Linux/Mac or (CTRL+Z) for Windows
#[test]
fn test_wallet_convert_pem_to_keystore() {
    let keystore_password = "1234";
    create_keystore_from_pem();
    let (private_key_pem, _public_key_pem) = Wallet::get_wallet_keys_pem(ALICE_PEM_PATH);
    assert_eq!(
        Wallet::get_private_key_from_keystore_secret(ALICE_KEYSTORE_PATH, keystore_password)
            .unwrap()
            .to_string(),
        private_key_pem
    );
    fs::remove_file(ALICE_KEYSTORE_PATH).unwrap();
}

// Insert the same password twice, each followed by (CTRL+D) for Linux/Mac or (CTRL+Z) for Windows
#[test]
fn test_wallet_convert_keystore_to_pem() {
    create_keystore_from_pem();
    let wallet_convert_args = WalletConvertArgs {
        infile: Some(ALICE_KEYSTORE_PATH.to_string()),
        outfile: Some("alice_test.pem".to_string()),
        from: "keystore-secret".to_string(),
        to: "pem".to_string(),
    };
    let wallet_args = WalletArgs {
        command: WalletAction::Convert(wallet_convert_args),
    };
    wallet(&wallet_args);
    assert_eq!(
        Wallet::get_pem_decoded_content(ALICE_PEM_PATH),
        Wallet::get_pem_decoded_content("alice_test.pem")
    );
    fs::remove_file("alice_test.pem").unwrap();
    fs::remove_file(ALICE_KEYSTORE_PATH).unwrap();
}
