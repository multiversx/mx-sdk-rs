use std::fs::{self, File};
use std::io::Write;

use multiversx_sc_meta::cmd::wallet::generate_pem_content;
use multiversx_sc_snippets::sdk::{crypto::public_key::PublicKey, data::address::Address};
use multiversx_sc_snippets::{hex, imports::Wallet};

const ALICE_PEM_PATH: &str = "../snippets/src/test_wallets/alice.pem";
const ALICE_KEYSTORE_PATH_TEST: &str = "alice.json";
const ALICE_PEM_PATH_TEST: &str = "alice_test.pem";
const KEYSTORE_PASSWORD: &str = "abcd";

fn create_keystore_from_pem(file: &str) {
    let pem_decoded_keys = Wallet::get_pem_decoded_content(file);
    let (private_key_str, public_key_str) = Wallet::get_wallet_keys_pem(file);

    let address = Wallet::from_private_key(&private_key_str)
        .unwrap()
        .address();
    let hex_decoded_keys = hex::decode(pem_decoded_keys).unwrap();

    let json_result = Wallet::encrypt_keystore(
        hex_decoded_keys.as_slice(),
        &address,
        &public_key_str,
        KEYSTORE_PASSWORD,
    );

    write_to_file(&json_result, ALICE_KEYSTORE_PATH_TEST);
}

fn write_to_file(content: &str, file: &str) {
    let mut file = File::create(file).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

fn create_keystore_file_from_scratch() -> Address {
    let mnemonic = Wallet::generate_mnemonic();
    let (private_key_str, public_key_str) = Wallet::get_wallet_keys_mnemonic(mnemonic.to_string());
    let wallet = Wallet::from_private_key(&private_key_str).unwrap();
    let address = wallet.address();

    let concatenated_keys = format!("{}{}", private_key_str, public_key_str);
    let hex_decoded_keys = hex::decode(concatenated_keys).unwrap();
    let json_result = Wallet::encrypt_keystore(
        hex_decoded_keys.as_slice(),
        &address,
        &public_key_str,
        KEYSTORE_PASSWORD,
    );
    write_to_file(&json_result, ALICE_KEYSTORE_PATH_TEST);
    address
}

#[test]
fn test_wallet_convert_pem_to_keystore() {
    create_keystore_from_pem(ALICE_PEM_PATH);
    let (private_key_pem, _public_key_pem) = Wallet::get_wallet_keys_pem(ALICE_PEM_PATH);
    assert_eq!(
        Wallet::get_private_key_from_keystore_secret(ALICE_KEYSTORE_PATH_TEST, KEYSTORE_PASSWORD)
            .unwrap()
            .to_string(),
        private_key_pem
    );
    fs::remove_file(ALICE_KEYSTORE_PATH_TEST).unwrap();
}

#[test]
fn test_wallet_convert_keystore_to_pem() {
    let address = create_keystore_file_from_scratch();

    let private_key =
        Wallet::get_private_key_from_keystore_secret(ALICE_KEYSTORE_PATH_TEST, KEYSTORE_PASSWORD)
            .unwrap();
    let private_key_str = private_key.to_string();
    let public_key = PublicKey::from(&private_key);
    let public_key_str = public_key.to_string();

    let pem_content = generate_pem_content(&address, &private_key_str, &public_key_str);
    write_to_file(&pem_content, ALICE_PEM_PATH_TEST);

    assert_eq!(
        Wallet::get_private_key_from_keystore_secret(ALICE_KEYSTORE_PATH_TEST, KEYSTORE_PASSWORD)
            .unwrap()
            .to_string(),
        Wallet::get_wallet_keys_pem(ALICE_PEM_PATH_TEST).0
    );

    fs::remove_file(ALICE_PEM_PATH_TEST).unwrap();
    fs::remove_file(ALICE_KEYSTORE_PATH_TEST).unwrap();
}
