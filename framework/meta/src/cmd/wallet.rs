use base64;
use core::str;
use multiversx_sc::types;
use std::{
    fs::{self, File},
    io::{self, Read, Write},
};

use crate::cli::{WalletAction, WalletArgs, WalletConvertArgs, WalletNewArgs};
use multiversx_sc_snippets::{hex, imports::Bech32Address};
use multiversx_sdk::{crypto::public_key::PublicKey, data::address::Address, wallet::Wallet};

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
    let infile = convert_args.infile.as_ref();
    let outfile = convert_args.outfile.as_ref();
    let in_format = &convert_args.from;
    let out_format = &convert_args.to;

    let mut in_address = String::new();
    let mut out_address: String = String::new();

    match infile {
        Some(file) => in_address = fs::read_to_string(file).unwrap(),
        None => {
            println!("Insert text below. Press 'Ctrl-D' (Linux / MacOS) or 'Ctrl-Z' (Windows) when done.");
            _ = io::stdin().read_to_string(&mut in_address).unwrap()
        },
    }

    in_address = in_address.replace('\n', "");

    match (in_format.as_str(), out_format.as_str()) {
        ("address-bech32", "address-hex") => {
            out_address = Bech32Address::from_bech32_string(in_address).to_hex();
        },
        ("address-hex", "address-bech32") => {
            let bytes_from_hex = hex::decode(in_address).unwrap();
            let bytes_arr: [u8; 32] = bytes_from_hex.try_into().unwrap();

            let addr = types::Address::from(&bytes_arr);
            out_address = Bech32Address::from(addr).to_bech32_str().to_string();
        },
        ("", _) | (_, "") => {
            println!("error: the following arguments are required: --in-format, --out-format");
        },
        _ => {
            println!("Unsupported conversion");
        },
    }

    match outfile {
        Some(outfile) => {
            let mut file = File::create(outfile).unwrap();
            out_address.push('\n');
            file.write_all(out_address.as_bytes()).unwrap();
        },
        None => {
            println!("{}", out_address);
        },
    }
}

fn new(new_args: &WalletNewArgs) {
    let format = new_args.wallet_format.as_ref();
    let outfile = new_args.outfile.as_ref();
    let mnemonic = Wallet::generate_mnemonic();
    println!("Mnemonic: {}", mnemonic);

    let private_key = Wallet::get_private_key_from_mnemonic(mnemonic, 0u32, 0u32);
    let public_key = PublicKey::from(&private_key);

    let public_key_str: &str = &public_key.to_string();
    let private_key_str: &str = &private_key.to_string();

    let wallet = Wallet::from_private_key(private_key_str).unwrap();
    let address = wallet.address();

    println!("Wallet address: {}", address);

    if let Some(f) = format {
        match (f.as_str(), outfile) {
            ("pem", Some(file)) => {
                generate_pem(&address, private_key_str, public_key_str, file);
            },
            ("pem", None) => {
                println!("Output file is required for PEM format");
            },
            _ => {},
        }
    }
}

fn generate_pem(address: &Address, private_key: &str, public_key: &str, outfile: &String) {
    let concat_keys = format!("{}{}", private_key, public_key);
    let concat_keys_b64 = base64::encode(concat_keys);

    // Split the base64 string into 64-character lines
    let formatted_key = concat_keys_b64
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
