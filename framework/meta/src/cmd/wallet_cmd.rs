use core::str;

use bip39::{Language, Mnemonic};
use rand::Rng;
use std::{
    fs::{self, File},
    io::{self, Read, Write},
};

use multiversx_sc::types::Address;
use multiversx_sc_snippets::sdk::chain_core::std::Bech32Hrp;
use multiversx_sc_snippets::sdk::wallet::Keystore;
use multiversx_sc_snippets::sdk::wallet::KeystoreRandomness;
use multiversx_sc_snippets::sdk::wallet::Wallet;
use multiversx_sc_snippets::{hex, imports::Bech32Address};
use multiversx_sdk::crypto::private_key::PrivateKey;

use crate::cli::cli_args_sender::get_keystore_password;
use crate::cli::{
    WalletAction, WalletArgs, WalletBech32Args, WalletConvertArgs, WalletNewArgs,
    WalletTestWalletArgs,
};

pub fn wallet(args: &WalletArgs) {
    let command = &args.command;
    match command {
        WalletAction::New(new_args) => new(new_args),
        WalletAction::Bech32(bech32_args) => bech32_conversion(bech32_args),
        WalletAction::Convert(convert_args) => convert(convert_args),
        WalletAction::TestWallet(test_wallet_args) => test_wallet_cmd(test_wallet_args),
    }
}

fn convert(convert_args: &WalletConvertArgs) {
    let infile = convert_args.infile.as_ref();
    let outfile = convert_args.outfile.as_ref();
    let in_format = &convert_args.from;
    let out_format = &convert_args.to;
    let hrp = convert_args
        .hrp
        .as_deref()
        .map(|hrp| Bech32Hrp::try_from(hrp).expect("invalid HRP"))
        .unwrap_or_default();

    let mut mnemonic_str = String::new();

    match (in_format.as_str(), out_format.as_str()) {
        ("mnemonic", "pem") => match infile {
            Some(file) => {
                mnemonic_str = fs::read_to_string(file).unwrap();
                let wallet = Wallet::from_mnemonic_string(mnemonic_str);
                write_resulted_pem(hrp, &wallet.private_key_hex(), outfile);
            }
            None => {
                println!(
                    "Insert text below. Press 'Ctrl-D' (Linux / MacOS) or 'Ctrl-Z' (Windows) when done."
                );
                _ = io::stdin().read_to_string(&mut mnemonic_str).unwrap();
                let wallet = Wallet::from_mnemonic_string(mnemonic_str);
                write_resulted_pem(hrp, &wallet.private_key_hex(), outfile);
            }
        },
        ("keystore-secret", "pem") => match infile {
            Some(file) => {
                let private_key = Keystore::from_file(file)
                    .unwrap()
                    .extract_private_key(&get_keystore_password())
                    .unwrap();
                write_resulted_pem(hrp, &private_key.to_string(), outfile);
            }
            None => {
                panic!("Input file is required for keystore-secret format");
            }
        },
        ("pem", "keystore-secret") => match infile {
            Some(file) => {
                let wallet = Wallet::from_pem_file(file).expect("error reading PEM file");
                let randomness = new_keystore_randomness();
                let json_result = Keystore::encrypt(
                    wallet.priv_key,
                    wallet.address.to_bech32(hrp),
                    &get_keystore_password(),
                    randomness,
                )
                .to_json_string();
                write_resulted_keystore(json_result, outfile);
            }
            None => {
                panic!("Input file is required for pem format");
            }
        },
        _ => {
            println!("Unsupported conversion");
        }
    }
}

fn write_resulted_pem(hrp: Bech32Hrp, private_key: &str, outfile: Option<&String>) {
    let wallet = Wallet::from_private_key_hex(private_key).unwrap();
    let pem_content = wallet.to_pem(hrp).to_pem_str();
    match outfile {
        Some(outfile) => {
            let mut file = File::create(outfile).unwrap();
            file.write_all(pem_content.as_bytes()).unwrap();
        }
        None => {
            print!("{}", pem_content);
        }
    }
}

fn write_resulted_keystore(json_result: String, outfile: Option<&String>) {
    match outfile {
        Some(outfile) => {
            let mut file = File::create(outfile).unwrap();
            file.write_all(json_result.as_bytes()).unwrap();
        }
        None => {
            println!("{}", json_result);
        }
    }
}

pub fn new_keystore_randomness() -> KeystoreRandomness {
    let mut salt = [0u8; 32];
    let mut iv = [0u8; 16];
    rand::rng().fill_bytes(&mut salt);
    rand::rng().fill_bytes(&mut iv);
    KeystoreRandomness {
        salt,
        iv,
        id: uuid::Uuid::new_v4().to_string(),
    }
}

fn bech32_conversion(bech32_args: &WalletBech32Args) {
    let encode_address = bech32_args.hex_address.as_ref();
    let decode_address = bech32_args.bech32_address.as_ref();

    match (encode_address, decode_address) {
        (Some(hex), None) => {
            let bytes_from_hex = hex::decode(hex).unwrap();
            let bytes_arr: [u8; 32] = bytes_from_hex.try_into().unwrap();

            let addr = Address::from(&bytes_arr);
            let bech32_addr = Bech32Address::from(addr).to_bech32_str().to_string();
            println!("{}", bech32_addr);
        }
        (None, Some(bech32)) => {
            let bech32_address = Bech32Address::from_bech32_string(bech32.to_string());
            let hex_addr = hex::encode(&bech32_address.address);
            println!("{}", hex_addr);
        }
        (Some(_), Some(_)) => {
            println!("error: only one of --encode or --decode can be used in the same command");
        }
        _ => {}
    }
}

pub fn generate_mnemonic() -> Mnemonic {
    Mnemonic::generate_in(Language::English, 24).unwrap()
}

struct NewWalletInfo {
    mnemonic: Mnemonic,
    wallet: Wallet,
}

impl NewWalletInfo {
    fn generate() -> Self {
        let mnemonic = generate_mnemonic();
        let wallet = Wallet::from_mnemonic_string(mnemonic.to_string());
        NewWalletInfo { mnemonic, wallet }
    }

    fn generate_for_shard(shard: u8) -> Self {
        assert!(shard < 3, "Shard must be between 0 and 2");
        loop {
            let wallet_info = Self::generate();
            if wallet_info.wallet.address.shard_of_3().as_u32() == shard as u32 {
                return wallet_info;
            }
        }
    }
}

fn new(new_args: &WalletNewArgs) {
    let format = new_args.wallet_format.as_deref();
    let outfile = new_args.outfile.as_ref();
    let hrp = new_args
        .hrp
        .as_deref()
        .map(|hrp| Bech32Hrp::try_from(hrp).expect("invalid HRP"))
        .unwrap_or_default();

    let new_wallet_info = if let Some(shard) = new_args.shard {
        NewWalletInfo::generate_for_shard(shard)
    } else {
        NewWalletInfo::generate()
    };

    println!("Mnemonic: {}", new_wallet_info.mnemonic);

    println!("Wallet address:");
    println!(
        "  - bech32: {}",
        new_wallet_info.wallet.address.to_bech32(hrp)
    );
    println!("  - hex:    0x{}", new_wallet_info.wallet.address.to_hex());

    match format {
        Some("pem") => {
            write_resulted_pem(hrp, &new_wallet_info.wallet.private_key_hex(), outfile);
            if let Some(outfile) = outfile {
                println!("Wallet saved to '{outfile}'");
            }
        }
        Some("keystore-secret") => {
            let randomness = new_keystore_randomness();
            let json_result = Keystore::encrypt(
                new_wallet_info.wallet.priv_key,
                new_wallet_info.wallet.address.to_bech32(hrp),
                &get_keystore_password(),
                randomness,
            )
            .to_json_string();
            write_resulted_keystore(json_result, outfile);
            if let Some(outfile) = outfile {
                println!("Wallet saved to '{outfile}'");
            }
        }
        Some(_) => {
            println!("Unsupported format");
        }
        None => {}
    }
}

fn test_wallet_cmd(args: &WalletTestWalletArgs) {
    let name = &args.name;
    let pem = match multiversx_sc_snippets::test_wallets::pem_contents(name) {
        Some(pem) => pem,
        None => {
            let valid = multiversx_sc_snippets::test_wallets::valid_names().join(", ");
            eprintln!("Unknown test wallet name: '{name}'. Valid names: {valid}");
            std::process::exit(1);
        }
    };
    let path = args.path.clone().unwrap_or_else(|| format!("{name}.pem"));
    let mut file = File::create(&path).unwrap();
    file.write_all(pem.as_bytes()).unwrap();
    println!("Saved test wallet '{name}' to '{path}'");
}

/// Currently not in use.
#[allow(unused)]
pub fn generate_random_private_key<T>(r: &mut T) -> PrivateKey
where
    T: rand::CryptoRng + rand::Rng,
{
    let mut secret_key = PrivateKey([0u8; 64]);

    r.fill_bytes(&mut secret_key.0);

    secret_key
}
