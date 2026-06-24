use std::{
    fs::File,
    io::{self, Write},
};

use anyhow::Result;
use multiversx_sc_meta::cmd::wallet_cmd::generate_mnemonic;
use multiversx_sdk::{chain_core::std::Bech32Hrp, wallet::Wallet};

const SUFFIXES: [&str; 3] = ["0000", "0001", "0002"];

fn main() -> Result<()> {
    let hrp = Bech32Hrp::default();
    let mut mined_count = 0usize;
    let mut retries = 0u64;

    loop {
        let mnemonic = generate_mnemonic();
        let wallet = Wallet::try_from(mnemonic.clone()).unwrap();
        let address_hex = wallet.address.to_hex();

        for suffix in SUFFIXES {
            if !address_hex.ends_with(suffix) {
                continue;
            }

            mined_count += 1;

            if retries >= 1000 {
                println!("\rMining suffixes: found 0x{suffix} after {retries} retries");
            }

            println!("Mined wallet ending in 0x{suffix}:");
            println!("Mnemonic: {}", mnemonic);
            println!("Wallet address:");
            println!("  - bech32: {}", wallet.address.to_bech32(hrp));
            println!("  - hex:    0x{address_hex}");

            let pem_content = wallet.to_pem(hrp).to_pem_str();
            write_pem(suffix, &address_hex, pem_content)?;

            println!();
        }

        retries += 1;
        if retries.is_multiple_of(100) {
            print!("\rMining suffixes forever: {retries} retries, {mined_count} wallets mined");
            io::stdout().flush()?;
        }
    }
}

fn write_pem(suffix: &str, address_hex: &str, contents: String) -> Result<()> {
    let outfile = format!("wallet-{suffix}-{address_hex}.pem");
    let mut file = File::create(&outfile)?;
    file.write_all(contents.as_bytes())?;
    println!("Wallet saved to '{outfile}'");

    Ok(())
}
