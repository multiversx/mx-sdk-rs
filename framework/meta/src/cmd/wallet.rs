use base64;
use core::str;
use std::{
    fs::{self, File},
    io::Write,
};

use crate::cli::{WalletAction, WalletArgs, WalletConvertArgs, WalletNewArgs};
use multiversx_sc_snippets::{hex, imports::Bech32Address};
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
    let infile = convert_args
        .infile
        .as_ref()
        .expect("input file is required");
    let outfile = convert_args.outfile.as_ref();
    let in_format = convert_args
        .from
        .as_ref()
        .expect("input format is required");
    let out_format = convert_args.to.as_ref().expect("output format is required");

    let in_address = fs::read_to_string(infile).unwrap();
    let mut out_addr: String = String::from("");

    match (in_format.as_str(), out_format.as_str()) {
        ("address-bech32", "address-hex") => {
            out_addr = Bech32Address::from_bech32_string(in_address).to_hex();
        },
        ("address-hex", "address-bech32") => {
            // out_addr = Bech32Address::from(in_address).to_bech32_string();
            let bytes_from_hex: [u8; 64] = hex::decode(in_address).unwrap().try_into().unwrap();
            out_addr = Bech32Address::from_bech32_string(
                str::from_utf8(&bytes_from_hex).unwrap().to_string(),
            )
            .to_bech32_string();
        },
        _ => {
            println!("Unsupported conversion");
        },
    }

    match outfile {
        Some(outfile) => {
            let mut file = File::create(outfile).unwrap();
            file.write_all(out_addr.as_bytes()).unwrap();
        },
        None => {
            println!("{}", out_addr);
        },
    }
}

fn new(new_args: &WalletNewArgs) {
    let format = new_args
        .wallet_format
        .as_ref()
        .expect("wallet format is required");
    let outfile = new_args.outfile.as_ref().expect("output file is required");

    match format.as_str() {
        "pem" => {
            let mnemonic = Wallet::generate_mnemonic();
            let private_key = Wallet::get_private_key_from_mnemonic(mnemonic, 0u32, 0u32);
            let pk_str: &str = &private_key.to_string();
            let wallet = Wallet::from_private_key(pk_str).unwrap();
            let address = wallet.address();

            println!("Wallet address: {}", address);

            generate_pem(&address, pk_str, outfile);
        },
        _ => {
            println!("Unsupported wallet format");
        },
    }
}

fn generate_pem(address: &Address, private_key: &str, outfile: &String) {
    println!("{private_key}");
    let private_key_hex_encoded = hex::encode(private_key.as_bytes());
    println!("HEX {private_key_hex_encoded}");
    // let priv_key_bytes = private_key_hex_encoded.as_bytes();
    let private_key_base64 = base64::encode(&private_key_hex_encoded.as_bytes());
    println!("B64 {private_key_base64}");

    // Split the base64 string into 64-character lines
    let formatted_key = private_key_base64
        .as_bytes()
        .chunks(64)
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .collect::<Vec<&str>>()
        .join("\n");

    let pem_content = format!(
        "-----BEGIN PRIVATE KEY for {}-----\n{}\n-----END PRIVATE KEY for {}-----\n",
        address.to_bech32_string().unwrap(),
        formatted_key,
        address.to_bech32_string().unwrap()
    );

    let mut file = File::create(outfile).unwrap();
    file.write_all(pem_content.as_bytes()).unwrap()
}
