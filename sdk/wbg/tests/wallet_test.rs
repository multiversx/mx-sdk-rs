use bip39::Mnemonic;

use multiversx_sdk_wbg::{crypto::public_key::PublicKey, data::address::Address, wallet::Wallet};
use std::fs::{self, File};
use std::io::Write;

const ALICE_PEM_PATH: &str = "tests/alice.pem";
const ALICE_KEYSTORE_PATH_TEST_1: &str = "tests/alice1.json";
const ALICE_KEYSTORE_PATH_TEST_2: &str = "tests/alice2.json";
const ALICE_PEM_PATH_TEST: &str = "tests/alice_test.pem";
const KEYSTORE_PASSWORD: &str = "abcd";
const ALICE_PUBLIC_KEY: &str = "0139472eff6886771a982f3083da5d421f24c29181e63888228dc81ca60d69e1";
const ALICE_PRIVATE_KEY: &str = "413f42575f7f26fad3317a778771212fdb80245850981e48b58a4f25e344e8f9";

#[test]
fn test_private_key_from_mnemonic() {
    let mnemonic: Mnemonic = Mnemonic::parse_normalized("acid twice post genre topic observe valid viable gesture fortune funny dawn around blood enemy page update reduce decline van bundle zebra rookie real").unwrap();

    let private_key = Wallet::get_private_key_from_mnemonic(mnemonic.clone(), 0, 0);
    let public_key = PublicKey::from(&private_key);
    let address = Address::from(&public_key);
    assert_eq!(
        "0b7966138e80b8f3bb64046f56aea4250fd7bacad6ed214165cea6767fd0bc2c",
        private_key.to_string()
    );
    assert_eq!(
        "dfefe0453840e5903f2bd519de9b0ed6e9621e57e28ba0b4c1b15115091dd72f",
        public_key.to_string()
    );
    assert_eq!(
        "erd1mlh7q3fcgrjeq0et65vaaxcw6m5ky8jhu296pdxpk9g32zga6uhsemxx2a",
        address.to_string()
    );

    let private_key = Wallet::get_private_key_from_mnemonic(mnemonic, 0, 1);
    let public_key = PublicKey::from(&private_key);
    let address = Address::from(&public_key);
    assert_eq!(
        "1648ad209d6b157a289884933e3bb30f161ec7113221ec16f87c3578b05830b0",
        private_key.to_string()
    );
    assert_eq!(
        "af8fef070a581873912ccbafb6a78bb9eb4e003085ac43dbbdfa3e20eb93cede",
        public_key.to_string()
    );
    assert_eq!(
        "erd147877pc2tqv88yfvewhmdfuth845uqpsskky8kaalglzp6unem0qpwh982",
        address.to_string()
    );
}

#[test]
fn test_load_from_pem() {
    let wallet = Wallet::from_pem_file("tests/alice.pem").unwrap();
    let addr = wallet.address();
    assert_eq!(
        addr.to_bech32_string().unwrap(),
        "erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th"
    );
}

fn write_to_file(content: &str, file: &str) {
    let mut file = File::create(file).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

fn create_keystore_file_from_scratch(file: &str) -> Address {
    let wallet = Wallet::from_private_key(ALICE_PRIVATE_KEY).unwrap();
    let address = wallet.address();

    let concatenated_keys = format!("{}{}", ALICE_PRIVATE_KEY, ALICE_PUBLIC_KEY);
    let hex_decoded_keys = hex::decode(concatenated_keys).unwrap();
    let json_result = Wallet::encrypt_keystore(
        hex_decoded_keys.as_slice(),
        &address,
        ALICE_PUBLIC_KEY,
        KEYSTORE_PASSWORD,
    );
    write_to_file(&json_result, file);
    address
}

#[test]
fn test_wallet_convert_pem_to_keystore() {
    let _ = create_keystore_file_from_scratch(ALICE_KEYSTORE_PATH_TEST_1);
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
    let address = create_keystore_file_from_scratch(ALICE_KEYSTORE_PATH_TEST_2);

    let private_key =
        Wallet::get_private_key_from_keystore_secret(ALICE_KEYSTORE_PATH_TEST_2, KEYSTORE_PASSWORD)
            .unwrap();
    let private_key_str = private_key.to_string();
    let public_key = PublicKey::from(&private_key);
    let public_key_str = public_key.to_string();

    let pem_content = Wallet::generate_pem_content(&address, &private_key_str, &public_key_str);
    write_to_file(&pem_content, ALICE_PEM_PATH_TEST);
    assert_eq!(
        private_key_str,
        Wallet::get_wallet_keys_pem(ALICE_PEM_PATH_TEST).0
    );

    fs::remove_file(ALICE_PEM_PATH_TEST).unwrap();
    fs::remove_file(ALICE_KEYSTORE_PATH_TEST_2).unwrap();
}
