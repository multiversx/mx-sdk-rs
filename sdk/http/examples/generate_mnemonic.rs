use multiversx_sdk::wallet::Wallet;

fn main() {
    let mnemonic = Wallet::generate_mnemonic();
    println!("mnemonic: {mnemonic}");

    let private_key = Wallet::get_private_key_from_mnemonic(mnemonic, 0, 0);
    println!("private key: {:?}", private_key.to_string());
}
