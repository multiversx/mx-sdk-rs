mod keystore;
mod keystore_json;
mod wallet_pem;

pub use keystore::Keystore;
pub use keystore_json::*;
pub use wallet_pem::WalletPem;

use core::str;
use std::{
    io::{self, Write},
    path::Path,
};

use anyhow::Result;
use bip39::Mnemonic;
use multiversx_chain_core::{
    std::{Bech32Address, Bech32Hrp},
    types::Address,
};
use serde_json::json;
use sha2::Digest;
use sha3::Keccak256;

use crate::{
    crypto::{private_key::PrivateKey, public_key::PublicKey},
    data::transaction::Transaction,
};

#[derive(Clone, Debug)]
pub struct Wallet {
    priv_key: PrivateKey,
    pub address: Address,
    pub hrp: Option<Bech32Hrp>,
}

impl From<WalletPem> for Wallet {
    fn from(wallet_pem: WalletPem) -> Self {
        Self::from_private_key(wallet_pem.priv_key, Some(wallet_pem.address.hrp))
    }
}

impl Wallet {
    fn from_private_key(priv_key: PrivateKey, hrp: Option<Bech32Hrp>) -> Self {
        let address = PublicKey::from(&priv_key).to_address();
        Wallet {
            priv_key,
            address,
            hrp,
        }
    }

    pub fn from_mnemonic_string(mnemonic_str: String) -> Wallet {
        let mnemonic = Mnemonic::parse(mnemonic_str.replace('\n', "")).unwrap();
        let private_key = PrivateKey::from_mnemonic(mnemonic, 0u32, 0u32);
        Self::from_private_key(private_key, None)
    }

    pub fn from_private_key_hex(priv_key: &str) -> Result<Self> {
        let priv_key = PrivateKey::from_hex_str(priv_key)?;
        Ok(Self::from_private_key(priv_key, None))
    }

    pub fn from_pem_file<P>(file_path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let contents = std::fs::read_to_string(file_path)?;
        Self::from_pem_file_contents(contents)
    }

    pub fn from_pem_file_contents(contents: String) -> Result<Self> {
        Ok(WalletPem::from_pem_str(&contents)?.into())
    }

    pub fn get_shard(&self) -> u8 {
        let address = self.to_address();
        let address_bytes = address.as_bytes();
        address_bytes[address_bytes.len() - 1] % 3
    }

    pub fn from_keystore_secret<P: AsRef<Path>>(
        file_path: P,
        insert_password: InsertPassword,
    ) -> Result<Self> {
        let keystore = Keystore::from_file(&file_path);
        let decryption_params = match insert_password {
            InsertPassword::Plaintext(password) => {
                keystore.validate_password(&password).unwrap_or_else(|e| {
                    panic!("Error: {:?}", e);
                })
            }
            InsertPassword::StandardInput => keystore
                .validate_password(&Self::get_keystore_password())
                .unwrap_or_else(|e| {
                    panic!("Error: {:?}", e);
                }),
        };
        let priv_key = PrivateKey::from_hex_str(
            hex::encode(Keystore::decrypt_secret_key(decryption_params)).as_str(),
        )?;
        Ok(Self::from_private_key(priv_key, None))
    }

    #[deprecated(
        since = "0.54.0",
        note = "Renamed to `to_address`, type changed to multiversx_chain_core::types::Address"
    )]
    pub fn address(&self) -> Bech32Address {
        self.to_address().to_bech32_default()
    }

    pub fn to_address(&self) -> Address {
        self.address.clone()
    }

    pub fn private_key_hex(&self) -> String {
        self.priv_key.to_string()
    }

    pub fn public_key_hex(&self) -> String {
        PublicKey::from(&self.priv_key).to_string()
    }

    pub fn sign_tx(&self, unsign_tx: &Transaction) -> [u8; 64] {
        let mut unsign_tx = unsign_tx.clone();
        unsign_tx.signature = None;

        let mut tx_bytes = json!(unsign_tx).to_string().as_bytes().to_vec();

        let should_sign_on_tx_hash = unsign_tx.version >= 2 && unsign_tx.options & 1 > 0;
        if should_sign_on_tx_hash {
            let mut h = Keccak256::new();
            h.update(tx_bytes);
            tx_bytes = h.finalize().to_vec();
        }

        self.priv_key.sign(tx_bytes)
    }

    pub fn sign_bytes(&self, data: Vec<u8>) -> [u8; 64] {
        self.priv_key.sign(data)
    }

    pub fn get_keystore_password() -> String {
        print!("Insert password: ");
        io::stdout().flush().unwrap();
        rpassword::read_password().unwrap()
    }

    pub fn to_pem(&self, hrp: Bech32Hrp) -> WalletPem {
        WalletPem {
            priv_key: self.priv_key,
            address: Bech32Address::encode_address(hrp, self.address.clone()),
        }
    }
}
