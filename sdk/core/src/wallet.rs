extern crate rand;

use std::{
    fs::{self},
    io::{self, Read},
};

use aes::{cipher::KeyIvInit, Aes128};
use anyhow::Result;
use bip39::{Language, Mnemonic};
use ctr::{cipher::StreamCipher, Ctr128BE};
use hmac::{Hmac, Mac};
use pbkdf2::pbkdf2;
use scrypt::{scrypt, Params};
use serde_json::json;
use sha2::{Digest, Sha256, Sha512};
use sha3::Keccak256;
use zeroize::Zeroize;

use crate::{
    crypto::{
        private_key::{PrivateKey, PRIVATE_KEY_LENGTH},
        public_key::PublicKey,
    },
    data::{
        address::Address,
        keystore::{DecryptionParams, Keystore, WalletError},
        transaction::Transaction,
    },
};

const EGLD_COIN_TYPE: u32 = 508;
const HARDENED: u32 = 0x80000000;
const CIPHER_ALGORITHM_AES_128_CTR: &str = "aes-128-ctr";
const KDF_SCRYPT: &str = "scrypt";

type HmacSha512 = Hmac<Sha512>;
type HmacSha256 = Hmac<Sha256>;

#[derive(Copy, Clone, Debug)]
pub struct Wallet {
    priv_key: PrivateKey,
}

impl Wallet {
    // GenerateMnemonic will generate a new mnemonic value using the bip39 implementation
    pub fn generate_mnemonic() -> Mnemonic {
        Mnemonic::generate_in(Language::English, 24).unwrap()
    }

    fn seed_from_mnemonic(mnemonic: Mnemonic, password: &str) -> [u8; 64] {
        let mut salt = String::with_capacity(8 + password.len());
        salt.push_str("mnemonic");
        salt.push_str(password);

        let mut seed = [0u8; 64];

        let _ = pbkdf2::<Hmac<Sha512>>(
            mnemonic.to_string().as_bytes(),
            salt.as_bytes(),
            2048,
            &mut seed,
        );

        salt.zeroize();

        seed
    }

    pub fn get_private_key_from_mnemonic(
        mnemonic: Mnemonic,
        account: u32,
        address_index: u32,
    ) -> PrivateKey {
        let seed = Self::seed_from_mnemonic(mnemonic, "");

        let serialized_key_len = 32;
        let hardened_child_padding: u8 = 0;

        let mut digest =
            HmacSha512::new_from_slice(b"ed25519 seed").expect("HMAC can take key of any size");
        digest.update(&seed);
        let intermediary: Vec<u8> = digest.finalize().into_bytes().into_iter().collect();
        let mut key = intermediary[..serialized_key_len].to_vec();
        let mut chain_code = intermediary[serialized_key_len..].to_vec();

        for child_idx in [
            44 | HARDENED,
            EGLD_COIN_TYPE | HARDENED,
            account | HARDENED, // account
            HARDENED,
            address_index | HARDENED, // addressIndex
        ] {
            let mut buff = [vec![hardened_child_padding], key.clone()].concat();
            buff.push((child_idx >> 24) as u8);
            buff.push((child_idx >> 16) as u8);
            buff.push((child_idx >> 8) as u8);
            buff.push(child_idx as u8);

            digest =
                HmacSha512::new_from_slice(&chain_code).expect("HMAC can take key of any size");
            digest.update(&buff);
            let intermediary: Vec<u8> = digest.finalize().into_bytes().into_iter().collect();
            key = intermediary[..serialized_key_len].to_vec();
            chain_code = intermediary[serialized_key_len..].to_vec();
        }

        PrivateKey::from_bytes(key.as_slice()).unwrap()
    }

    pub fn from_private_key(priv_key: &str) -> Result<Self> {
        let pri_key = PrivateKey::from_hex_str(priv_key)?;
        Ok(Self { priv_key: pri_key })
    }

    pub fn from_pem_file(file_path: &str) -> Result<Self> {
        let contents = std::fs::read_to_string(file_path).unwrap();
        Self::from_pem_file_contents(contents)
    }

    pub fn from_pem_file_contents(contents: String) -> Result<Self> {
        let x = pem::parse(contents)?;
        let x = x.contents()[..PRIVATE_KEY_LENGTH].to_vec();
        let priv_key_str = std::str::from_utf8(x.as_slice())?;
        let pri_key = PrivateKey::from_hex_str(priv_key_str)?;
        Ok(Self { priv_key: pri_key })
    }

    pub fn from_keystore_secret(file_path: &str) -> Result<Self> {
        let decyption_params =
            Self::validate_keystore_password(file_path, Self::get_keystore_password())
                .unwrap_or_else(|e| {
                    panic!("Error: {:?}", e);
                });
        let priv_key =
            PrivateKey::from_hex_str(Self::decrypt_secret_key(decyption_params).as_str())?;
        Ok(Self { priv_key })
    }

    pub fn get_private_key_from_keystore_secret(file_path: &str) -> Result<PrivateKey> {
        let decyption_params =
            Self::validate_keystore_password(file_path, Self::get_keystore_password())
                .unwrap_or_else(|e| {
                    panic!("Error: {:?}", e);
                });
        let priv_key =
            PrivateKey::from_hex_str(Self::decrypt_secret_key(decyption_params).as_str())?;
        Ok(priv_key)
    }

    pub fn address(&self) -> Address {
        let public_key = PublicKey::from(&self.priv_key);
        Address::from(&public_key)
    }

    pub fn sign_tx(&self, unsign_tx: &Transaction) -> [u8; 64] {
        let mut unsign_tx = unsign_tx.clone();
        unsign_tx.signature = None;

        let mut tx_bytes = json!(unsign_tx).to_string().as_bytes().to_vec();

        let should_sign_on_tx_hash = unsign_tx.version >= 2 && unsign_tx.options & 1 > 0;
        if should_sign_on_tx_hash {
            let mut h = Keccak256::new();
            h.update(tx_bytes);
            tx_bytes = h.finalize().as_slice().to_vec();
        }

        self.priv_key.sign(tx_bytes)
    }

    fn get_keystore_password() -> String {
        println!(
            "Insert password. Press 'Ctrl-D' (Linux / MacOS) or 'Ctrl-Z' (Windows) when done."
        );
        let mut password = String::new();
        io::stdin().read_to_string(&mut password).unwrap();
        password = password.trim().to_string();
        password
    }

    pub fn validate_keystore_password(
        path: &str,
        password: String,
    ) -> Result<DecryptionParams, WalletError> {
        let json_body = fs::read_to_string(path).unwrap();
        let keystore: Keystore = serde_json::from_str(&json_body).unwrap();
        let ciphertext = hex::decode(&keystore.crypto.ciphertext).unwrap();

        let cipher = &keystore.crypto.cipher;
        if cipher != CIPHER_ALGORITHM_AES_128_CTR {
            return Err(WalletError::InvalidCipher);
        }

        let iv = hex::decode(&keystore.crypto.cipherparams.iv).unwrap();
        let salt = hex::decode(&keystore.crypto.kdfparams.salt).unwrap();
        let json_mac = hex::decode(&keystore.crypto.mac).unwrap();

        let kdf = &keystore.crypto.kdf;
        if kdf != KDF_SCRYPT {
            return Err(WalletError::InvalidKdf);
        }
        let n = keystore.crypto.kdfparams.n as f64;
        let r = keystore.crypto.kdfparams.r as u64;
        let p = keystore.crypto.kdfparams.p as u64;
        let dklen = keystore.crypto.kdfparams.dklen as usize;

        let params = Params::new(n.log2() as u8, r as u32, p as u32, dklen).unwrap();

        let mut derived_key = vec![0u8; 32];
        scrypt(password.as_bytes(), &salt, &params, &mut derived_key).unwrap();

        let derived_key_first_half = derived_key[0..16].to_vec();
        let derived_key_second_half = derived_key[16..32].to_vec();

        let mut input_mac = HmacSha256::new_from_slice(&derived_key_second_half).unwrap();
        input_mac.update(&ciphertext);
        let computed_mac = input_mac.finalize().into_bytes();

        if computed_mac.as_slice() == json_mac.as_slice() {
            println!("Password is correct");
            Ok(DecryptionParams {
                derived_key_first_half,
                iv,
                ciphertext,
            })
        } else {
            println!("Password is incorrect");
            Err(WalletError::InvalidPassword)
        }
    }

    pub fn decrypt_secret_key(decryption_params: DecryptionParams) -> String {
        let mut cipher = Ctr128BE::<Aes128>::new(
            decryption_params.derived_key_first_half.as_slice().into(),
            decryption_params.iv.as_slice().into(),
        );
        let mut decrypted = decryption_params.ciphertext.to_vec();
        cipher.apply_keystream(&mut decrypted);

        hex::encode(decrypted).to_string()
    }
}
