use multiversx_sc::types::Address;
use multiversx_sdk::{crypto::public_key::PublicKey, wallet::Wallet};
use std::fs::{self, File};
use std::io::Write;

const ALICE_PEM_PATH: &str = "tests/alice.pem";
const ALICE_KEYSTORE_PATH_TEST_1: &str = "tests/alice1.json";
const ALICE_KEYSTORE_PATH_TEST_2: &str = "tests/alice2.json";
const ALICE_PEM_PATH_TEST: &str = "tests/alice_test.pem";
const KEYSTORE_PASSWORD: &str = "abcd";
const ALICE_PUBLIC_KEY: &str = "0139472eff6886771a982f3083da5d421f24c29181e63888228dc81ca60d69e1";
const ALICE_PRIVATE_KEY: &str = "413f42575f7f26fad3317a778771212fdb80245850981e48b58a4f25e344e8f9";

fn create_keystore_file_from_scratch(hrp: &str, file: &str) -> Address {
    let wallet = Wallet::from_private_key(ALICE_PRIVATE_KEY).unwrap();
    let address = wallet.to_address();

    let concatenated_keys = format!("{}{}", ALICE_PRIVATE_KEY, ALICE_PUBLIC_KEY);
    let hex_decoded_keys = hex::decode(concatenated_keys).unwrap();
    let json_result = Wallet::encrypt_keystore(
        hex_decoded_keys.as_slice(),
        hrp,
        &address,
        ALICE_PUBLIC_KEY,
        KEYSTORE_PASSWORD,
    );
    write_to_file(&json_result, file);
    address
}

#[test]
fn test_wallet_convert_pem_to_keystore() {
    let _ = create_keystore_file_from_scratch("erd", ALICE_KEYSTORE_PATH_TEST_1);
    let (private_key_pem, _public_key_pem) = Wallet::get_wallet_keys_pem(ALICE_PEM_PATH);
    assert_eq!(
        Wallet::get_private_key_from_keystore_secret(ALICE_KEYSTORE_PATH_TEST_1, KEYSTORE_PASSWORD)
            .unwrap()
            .to_string(),
        private_key_pem
    );
    fs::remove_file(ALICE_KEYSTORE_PATH_TEST_1).unwrap();
}

#[test]
fn test_wallet_convert_keystore_to_pem() {
    let address = create_keystore_file_from_scratch("erd", ALICE_KEYSTORE_PATH_TEST_2);

    let private_key =
        Wallet::get_private_key_from_keystore_secret(ALICE_KEYSTORE_PATH_TEST_2, KEYSTORE_PASSWORD)
            .unwrap();
    let private_key_str = private_key.to_string();
    let public_key = PublicKey::from(&private_key);
    let public_key_str = public_key.to_string();

    let pem_content =
        Wallet::generate_pem_content("erd", &address, &private_key_str, &public_key_str);
    write_to_file(&pem_content, ALICE_PEM_PATH_TEST);
    assert_eq!(
        private_key_str,
        Wallet::get_wallet_keys_pem(ALICE_PEM_PATH_TEST).0
    );

    fs::remove_file(ALICE_PEM_PATH_TEST).unwrap();
    fs::remove_file(ALICE_KEYSTORE_PATH_TEST_2).unwrap();
}

fn write_to_file(content: &str, file: &str) {
    let mut file = File::create(file).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}
