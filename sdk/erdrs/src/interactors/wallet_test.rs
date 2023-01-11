use bip39::Mnemonic;

use crate::{crypto::public_key::PublicKey, data::address::Address, interactors::wallet::Wallet};

#[test]
fn generate_mnemonic() {
    let mnemonic = Wallet::generate_mnemonic();
    println!("mnemonic: {}", mnemonic);

    let private_key = Wallet::get_private_key_from_mnemonic(mnemonic, 0, 0);
    println!("{:?}", private_key.to_string());
}

#[test]
fn get_private_key_from_mnemonic() {
    let mnemonic: Mnemonic = Mnemonic::parse_normalized("acid twice post genre topic observe valid viable gesture fortune funny dawn around blood enemy page update reduce decline van bundle zebra rookie real").unwrap();
    println!("mnemonic: {}", mnemonic);

    let private_key = Wallet::get_private_key_from_mnemonic(mnemonic.clone(), 0, 0);
    let public_key = PublicKey::from(&private_key);
    let address = Address::from(&public_key);
    println!("index: 0; private_key: {:?}", private_key.to_string());
    println!("index: 0; public_key: {:?}", public_key.to_string());
    println!("index: 0; address: {:?}", address.to_string());

    let private_key = Wallet::get_private_key_from_mnemonic(mnemonic, 0, 1);
    let public_key = PublicKey::from(&private_key);
    let address = Address::from(&public_key);
    println!("index: 1; private_key: {:?}", private_key.to_string());
    println!("index: 1; public_key: {:?}", public_key.to_string());
    println!("index: 1; address: {:?}", address.to_string());
}

#[ignore]
#[test]
fn load_from_pem() {
    let wallet = Wallet::from_pem_file("wallet-owner.pem").unwrap();
    let addr = wallet.address();
    println!("{:?}", addr);
}
