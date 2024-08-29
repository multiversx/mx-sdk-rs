use core::str;

use crate::cli::{WalletAction, WalletArgs, WalletBech32Args, WalletConvertArgs, WalletNewArgs};
use multiversx_sc::types::{self};
use multiversx_sc_snippets::sdk::{
    crypto::public_key::PublicKey, data::address::Address, wallet::Wallet,
};
use multiversx_sc_snippets::{hex, imports::Bech32Address};
use std::{
    fs::{self, File},
    io::{self, Read, Write},
};

pub fn wallet(args: &WalletArgs) {
    let command = &args.command;
    match command {
        WalletAction::New(new_args) => new(new_args),
        WalletAction::Bech32(bech32_args) => bech32_conversion(bech32_args),
        WalletAction::Convert(convert_args) => convert(convert_args),
    }
}

fn convert(convert_args: &WalletConvertArgs) {
    let infile = convert_args.infile.as_ref();
    let outfile = convert_args.outfile.as_ref();
    let in_format = &convert_args.from;
    let out_format = &convert_args.to;

    let mut mnemonic_str = String::new();
    let private_key_str: String;
    let public_key_str: String;

    match (in_format.as_str(), out_format.as_str()) {
        ("mnemonic", "pem") => match infile {
            Some(file) => {
                mnemonic_str = fs::read_to_string(file).unwrap();
                (private_key_str, public_key_str) = Wallet::get_wallet_keys_mnemonic(mnemonic_str);
                write_resulted_pem(&public_key_str, &private_key_str, outfile);
            },
            None => {
                println!("Insert text below. Press 'Ctrl-D' (Linux / MacOS) or 'Ctrl-Z' (Windows) when done.");
                _ = io::stdin().read_to_string(&mut mnemonic_str).unwrap();
                (private_key_str, public_key_str) = Wallet::get_wallet_keys_mnemonic(mnemonic_str);
                write_resulted_pem(&public_key_str, &private_key_str, outfile);
            },
        },
        ("keystore-secret", "pem") => match infile {
            Some(file) => {
                let private_key = Wallet::get_private_key_from_keystore_secret(
                    file,
                    &Wallet::get_keystore_password(),
                )
                .unwrap();
                private_key_str = private_key.to_string();
                let public_key = PublicKey::from(&private_key);
                public_key_str = public_key.to_string();
                write_resulted_pem(&public_key_str, &private_key_str, outfile);
            },
            None => {
                panic!("Input file is required for keystore-secret format");
            },
        },
        ("pem", "keystore-secret") => match infile {
            Some(file) => {
                let pem_decoded_keys = Wallet::get_pem_decoded_content(file);
                (private_key_str, public_key_str) = Wallet::get_wallet_keys_pem(file);

                let address = get_wallet_address(&private_key_str);
                let hex_decoded_keys = hex::decode(pem_decoded_keys).unwrap();

                let json_result = Wallet::encrypt_keystore(
                    hex_decoded_keys.as_slice(),
                    &address,
                    &public_key_str,
                    &Wallet::get_keystore_password(),
                );
                write_resulted_keystore(json_result, outfile);
            },
            None => {
                panic!("Input file is required for pem format");
            },
        },
        _ => {
            println!("Unsupported conversion");
        },
    }
}

fn write_resulted_pem(public_key: &str, private_key: &str, outfile: Option<&String>) {
    let address = get_wallet_address(private_key);
    match outfile {
        Some(outfile) => {
            let pem_content = Wallet::generate_pem_content(&address, private_key, public_key);
            let mut file = File::create(outfile).unwrap();
            file.write_all(pem_content.as_bytes()).unwrap();
        },
        None => {
            let pem_content = Wallet::generate_pem_content(&address, private_key, public_key);
            print!("{}", pem_content);
        },
    }
}

fn write_resulted_keystore(json_result: String, outfile: Option<&String>) {
    match outfile {
        Some(outfile) => {
            let mut file = File::create(outfile).unwrap();
            file.write_all(json_result.as_bytes()).unwrap();
        },
        None => {
            println!("{}", json_result);
        },
    }
}

fn bech32_conversion(bech32_args: &WalletBech32Args) {
    let encode_address = bech32_args.hex_address.as_ref();
    let decode_address = bech32_args.bech32_address.as_ref();

    match (encode_address, decode_address) {
        (Some(hex), None) => {
            let bytes_from_hex = hex::decode(hex).unwrap();
            let bytes_arr: [u8; 32] = bytes_from_hex.try_into().unwrap();

            let addr = types::Address::from(&bytes_arr);
            let bech32_addr = Bech32Address::from(addr).to_bech32_str().to_string();
            println!("{}", bech32_addr);
        },
        (None, Some(bech32)) => {
            let hex_addr = Bech32Address::from_bech32_string(bech32.to_string()).to_hex();
            println!("{}", hex_addr);
        },
        (Some(_), Some(_)) => {
            println!("error: only one of --encode or --decode can be used in the same command");
        },
        _ => {},
    }
}

fn get_wallet_address(private_key: &str) -> Address {
    let wallet = Wallet::from_private_key(private_key).unwrap();
    wallet.address()
}

fn new(new_args: &WalletNewArgs) {
    let format = new_args.wallet_format.as_deref();
    let outfile = new_args.outfile.as_ref(); // Handle outfile as Option<&str> if it's an Option<String>
    let mnemonic = Wallet::generate_mnemonic();
    println!("Mnemonic: {}", mnemonic);

    let (private_key_str, public_key_str) = Wallet::get_wallet_keys_mnemonic(mnemonic.to_string());
    let address = get_wallet_address(private_key_str.as_str());

    println!("Wallet address: {}", address);

    match format {
        Some("pem") => {
            write_resulted_pem(public_key_str.as_str(), private_key_str.as_str(), outfile);
        },
        Some("keystore-secret") => {
            let concatenated_keys = format!("{}{}", private_key_str, public_key_str);
            let hex_decoded_keys = hex::decode(concatenated_keys).unwrap();
            let json_result = Wallet::encrypt_keystore(
                hex_decoded_keys.as_slice(),
                &address,
                &public_key_str,
                &Wallet::get_keystore_password(),
            );
            write_resulted_keystore(json_result, outfile);
        },
        Some(_) => {
            println!("Unsupported format");
        },
        None => {},
    }
}
