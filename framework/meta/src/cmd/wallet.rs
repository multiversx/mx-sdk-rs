use std::{fs::File, io::Write};

use crate::cli::{WalletAction, WalletArgs, WalletConvertArgs, WalletFormat, WalletNewArgs};
use multiversx_sdk::{data::address::Address, wallet::Wallet};

pub fn wallet(args: &WalletArgs) {
    let command = args
        .command
        .as_ref()
        .expect("command expected after `wallet`");
    match command {
        WalletAction::New(new_args) => new(new_args),
        WalletAction::Convert(convert_args) => convert(convert_args),
    }
}

fn convert(convert_args: &WalletConvertArgs) {
    todo!()
}

fn new(new_args: &WalletNewArgs) {
    let format = new_args
        .wallet_format
        .as_ref()
        .expect("wallet format is required");
    let outfile = new_args.outfile.as_ref().expect("output file is required");

    match format {
        WalletFormat::Pem => {
            let mnemonic = Wallet::generate_mnemonic();
            let private_key = Wallet::get_private_key_from_mnemonic(mnemonic, 0u32, 0u32);
            let pk_str: &str = &private_key.to_string();
            let wallet = Wallet::from_private_key(pk_str).unwrap();
            let address = wallet.address();

            println!("Wallet address: {}", address);

            generate_pem(&address, pk_str, outfile);
        },
    }
}

fn generate_pem(address: &Address, private_key: &str, outfile: &String) {
    let pem_content = format!(
        "-----BEGIN PRIVATE KEY for {}-----\n{}\n-----END PRIVATE KEY for {}-----",
        address.to_bech32_string().unwrap(),
        private_key,
        address.to_bech32_string().unwrap()
    );

    let mut file = File::create(outfile).unwrap();
    file.write_all(pem_content.as_bytes()).unwrap()
}
