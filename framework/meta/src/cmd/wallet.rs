use core::str;
use multiversx_sc::{
    api::{uncallable::UncallableApi, BigIntApiImpl, ExternalViewApi, ManagedTypeApi},
    types::{self, ManagedBuffer},
};
use std::{
    fs::{self, File},
    io::{self, Read, Write},
};

use crate::cli::{WalletAction, WalletArgs, WalletBech32Args, WalletConvertArgs, WalletNewArgs};
use bip39::Mnemonic;
use multiversx_sc_snippets::sdk::{
    crypto::public_key::PublicKey,
    data::address::Address,
    utils::{base64_decode, base64_encode},
    wallet::Wallet,
};
use multiversx_sc_snippets::{hex, imports::Bech32Address};

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
    let mut private_key_str = String::new();
    let mut public_key_str = String::new();

    match (in_format.as_str(), out_format.as_str()) {
        ("mnemonic", "pem") => match infile {
            Some(file) => {
                mnemonic_str = fs::read_to_string(file).unwrap();
                (private_key_str, public_key_str) = get_wallet_keys_mnemonic(mnemonic_str);
                write_resulted_wallet(&public_key_str, &private_key_str, out_format, outfile);
            },
            None => {
                println!("Insert text below. Press 'Ctrl-D' (Linux / MacOS) or 'Ctrl-Z' (Windows) when done.");
                _ = io::stdin().read_to_string(&mut mnemonic_str).unwrap();
                (private_key_str, public_key_str) = get_wallet_keys_mnemonic(mnemonic_str);
                write_resulted_wallet(&public_key_str, &private_key_str, out_format, outfile);
            },
        },
        ("keystore-secret", "pem") => match infile {
            Some(file) => {
                let private_key = Wallet::get_private_key_from_keystore_secret(file).unwrap();
                private_key_str = private_key.to_string();
                let public_key = PublicKey::from(&private_key);
                public_key_str = public_key.to_string();
                write_resulted_wallet(&public_key_str, &private_key_str, out_format, outfile);
            },
            None => {
                panic!("Input file is required for keystore-secret format");
            },
        },
        ("pem", "keystore-secret") => match infile {
            Some(file) => {
                let pem_content = fs::read_to_string(file).unwrap();
                let lines: Vec<&str> = pem_content.split("\n").collect();
                let pem_encoded_keys = format!("{}{}{}", lines[1], lines[2], lines[3]);
                let pem_decoded_keys = base64_decode(pem_encoded_keys);
                let (private_key, public_key) =
                    pem_decoded_keys.split_at(pem_decoded_keys.len() / 2);
                private_key_str = String::from_utf8(private_key.to_vec()).unwrap();
                public_key_str = String::from_utf8(public_key.to_vec()).unwrap();

                let address = get_wallet_address(&private_key_str);
                let hex_decoded_keys = hex::decode(&private_key_str).unwrap();

                Wallet::encrypt_keystore(
                    hex_decoded_keys.as_slice(),
                    &address,
                    &public_key_str,
                    &Wallet::get_keystore_password(),
                );
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

fn write_resulted_wallet(
    public_key: &str,
    private_key: &str,
    format: &str,
    outfile: Option<&String>,
) {
    let address = get_wallet_address(private_key);
    match format {
        "pem" => match outfile {
            Some(outfile) => {
                generate_pem(&address, private_key, public_key, outfile);
            },
            None => {
                let pem_content = generate_pem_content(&address, private_key, public_key);
                print!("{}", pem_content);
            },
        },
        "keystore-secret" => match outfile {
            Some(outfile) => {},
            None => {},
        },
        _ => {},
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

fn get_wallet_keys_mnemonic(mnemonic_str: String) -> (String, String) {
    let mnemonic = Mnemonic::parse(mnemonic_str.replace('\n', "")).unwrap();
    let private_key = Wallet::get_private_key_from_mnemonic(mnemonic, 0u32, 0u32);
    let public_key = PublicKey::from(&private_key);

    let public_key_str: &str = &public_key.to_string();
    let private_key_str: &str = &private_key.to_string();

    (private_key_str.to_string(), public_key_str.to_string())
}

fn get_wallet_address(private_key: &str) -> Address {
    let wallet = Wallet::from_private_key(private_key).unwrap();
    wallet.address()
}

fn new(new_args: &WalletNewArgs) {
    let format = new_args.wallet_format.as_ref();
    let outfile = new_args.outfile.as_ref();
    let mnemonic = Wallet::generate_mnemonic();
    println!("Mnemonic: {}", mnemonic);

    let (private_key_str, public_key_str) = get_wallet_keys_mnemonic(mnemonic.to_string());
    let address = get_wallet_address(private_key_str.as_str());

    println!("Wallet address: {}", address);

    if let Some(f) = format {
        match (f.as_str(), outfile) {
            ("pem", Some(file)) => {
                generate_pem(
                    &address,
                    private_key_str.as_str(),
                    public_key_str.as_str(),
                    file,
                );
            },
            ("pem", None) => {
                println!("Output file is required for PEM format");
            },
            _ => {},
        }
    }
}

fn generate_pem(address: &Address, private_key: &str, public_key: &str, outfile: &String) {
    let pem_content = generate_pem_content(address, private_key, public_key);
    let mut file = File::create(outfile).unwrap();
    file.write_all(pem_content.as_bytes()).unwrap()
}

fn generate_pem_content(address: &Address, private_key: &str, public_key: &str) -> String {
    let concat_keys = format!("{}{}", private_key, public_key);
    let concat_keys_b64 = base64_encode(concat_keys);

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

    pem_content
}
