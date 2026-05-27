use multiversx_chain_core::std::Bech32Hrp;
use multiversx_sc::types::Address;
use multiversx_sc_meta::cmd::wallet_cmd::new_keystore_randomness;
use multiversx_sdk::wallet::Keystore;
use multiversx_sdk::wallet::Wallet;
use std::fs::{self, File};
use std::io::Write;

const ALICE_PEM_PATH: &str = "tests/alice.pem";
const ALICE_KEYSTORE_PATH_TEST_1: &str = "tests/alice1.json";
const ALICE_KEYSTORE_PATH_TEST_2: &str = "tests/alice2.json";
const ALICE_PEM_PATH_TEST: &str = "tests/alice_test.pem";
const KEYSTORE_PASSWORD: &str = "abcd";
const ALICE_PRIVATE_KEY: &str = "413f42575f7f26fad3317a778771212fdb80245850981e48b58a4f25e344e8f9";

fn create_keystore_file_from_scratch(hrp: &str, file: &str) -> Address {
    let wallet = Wallet::from_private_key_hex(ALICE_PRIVATE_KEY).unwrap();
    let json_result = Keystore::encrypt(
        wallet.private_key,
        hrp.try_into().expect("invalid HRP"),
        KEYSTORE_PASSWORD,
        new_keystore_randomness(),
    )
    .to_json_string();
    write_to_file(&json_result, file);
    wallet.address
}

#[test]
fn test_wallet_convert_pem_to_keystore() {
    let _ = create_keystore_file_from_scratch("erd", ALICE_KEYSTORE_PATH_TEST_1);
    let wallet_pem = Wallet::from_pem_file(ALICE_PEM_PATH).unwrap();
    assert_eq!(
        Keystore::from_file(ALICE_KEYSTORE_PATH_TEST_1)
            .unwrap()
            .decrypt_wallet(KEYSTORE_PASSWORD)
            .unwrap()
            .private_key,
        wallet_pem.private_key
    );
    fs::remove_file(ALICE_KEYSTORE_PATH_TEST_1).unwrap();
}

#[test]
fn test_wallet_convert_keystore_to_pem() {
    create_keystore_file_from_scratch("erd", ALICE_KEYSTORE_PATH_TEST_2);

    let wallet = Keystore::from_file(ALICE_KEYSTORE_PATH_TEST_2)
        .unwrap()
        .decrypt_wallet(KEYSTORE_PASSWORD)
        .unwrap();
    let pem_content = wallet.to_pem(Bech32Hrp::default()).to_pem_str();
    write_to_file(&pem_content, ALICE_PEM_PATH_TEST);
    assert_eq!(
        wallet.private_key,
        Wallet::from_pem_file(ALICE_PEM_PATH_TEST)
            .unwrap()
            .private_key
    );

    fs::remove_file(ALICE_PEM_PATH_TEST).unwrap();
    fs::remove_file(ALICE_KEYSTORE_PATH_TEST_2).unwrap();
}

fn write_to_file(content: &str, file: &str) {
    let mut file = File::create(file).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}
