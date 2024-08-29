extern crate rand;

use core::str;
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
use rand::RngCore;
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
    data::{address::Address, keystore::*, transaction::Transaction},
    utils::*,
};

use uuid::Uuid;

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
            HmacSha512::new_from_slice(&chain_code).expect("HMAC can take key of any size");
            digest.update(&buff);
            let intermediary: Vec<u8> = digest.finalize().into_bytes().into_iter().collect();
            key = intermediary[..serialized_key_len].to_vec();
            chain_code = intermediary[serialized_key_len..].to_vec();
        }

        PrivateKey::from_bytes(key.as_slice()).unwrap()
    }

    pub fn get_wallet_keys_mnemonic(mnemonic_str: String) -> (String, String) {
        let mnemonic = Mnemonic::parse(mnemonic_str.replace('\n', "")).unwrap();
        let private_key = Self::get_private_key_from_mnemonic(mnemonic, 0u32, 0u32);
        let public_key = PublicKey::from(&private_key);

        let public_key_str: &str = &public_key.to_string();
        let private_key_str: &str = &private_key.to_string();

        (private_key_str.to_string(), public_key_str.to_string())
    }

    pub fn from_private_key(priv_key: &str) -> Result<Self> {
        let priv_key = PrivateKey::from_hex_str(priv_key)?;
        Ok(Self { priv_key })
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

    pub fn get_pem_decoded_content(file: &str) -> Vec<u8> {
        let pem_content = fs::read_to_string(file).unwrap();
        let lines: Vec<&str> = pem_content.split('\n').collect();
        let pem_encoded_keys = format!("{}{}{}", lines[1], lines[2], lines[3]);
        base64_decode(pem_encoded_keys)
    }

    pub fn get_wallet_keys_pem(file: &str) -> (String, String) {
        let pem_decoded_keys = Self::get_pem_decoded_content(file);
        let (private_key, public_key) = pem_decoded_keys.split_at(pem_decoded_keys.len() / 2);
        let private_key_str = String::from_utf8(private_key.to_vec()).unwrap();
        let public_key_str = String::from_utf8(public_key.to_vec()).unwrap();

        (private_key_str, public_key_str)
    }

    pub fn from_keystore_secret(file_path: &str) -> Result<Self> {
        let decyption_params =
            Self::validate_keystore_password(file_path, Self::get_keystore_password())
                .unwrap_or_else(|e| {
                    panic!("Error: {:?}", e);
                });
        let priv_key = PrivateKey::from_hex_str(
            hex::encode(Self::decrypt_secret_key(decyption_params)).as_str(),
        )?;
        Ok(Self { priv_key })
    }

    pub fn get_private_key_from_keystore_secret(
        file_path: &str,
        password: &str,
    ) -> Result<PrivateKey> {
        let decyption_params = Self::validate_keystore_password(file_path, password.to_string())
            .unwrap_or_else(|e| {
                panic!("Error: {:?}", e);
            });
        let priv_key = PrivateKey::from_hex_str(
            hex::encode(Self::decrypt_secret_key(decyption_params)).as_str(),
        )?;
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

    pub fn get_keystore_password() -> String {
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
                data: ciphertext,
            })
        } else {
            println!("Password is incorrect");
            Err(WalletError::InvalidPassword)
        }
    }

    pub fn decrypt_secret_key(decryption_params: DecryptionParams) -> Vec<u8> {
        let mut cipher = Ctr128BE::<Aes128>::new(
            decryption_params.derived_key_first_half.as_slice().into(),
            decryption_params.iv.as_slice().into(),
        );
        let mut decrypted = decryption_params.data.to_vec();
        cipher.apply_keystream(&mut decrypted);

        decrypted
    }

    pub fn encrypt_keystore(
        data: &[u8],
        address: &Address,
        public_key: &str,
        password: &str,
    ) -> String {
        let params = Params::new((KDF_N as f64).log2() as u8, KDF_R, KDF_P, KDF_DKLEN).unwrap();
        let mut rand_salt: [u8; 32] = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut rand_salt);
        let salt_hex = hex::encode(rand_salt);

        let mut rand_iv: [u8; 16] = [0u8; 16];
        rand::thread_rng().fill_bytes(&mut rand_iv);
        let iv_hex = hex::encode(rand_iv);

        let mut derived_key = vec![0u8; 32];
        scrypt(password.as_bytes(), &rand_salt, &params, &mut derived_key).unwrap();

        let derived_key_first_half = derived_key[0..16].to_vec();
        let derived_key_second_half = derived_key[16..32].to_vec();

        let decryption_params = DecryptionParams {
            derived_key_first_half,
            iv: rand_iv.to_vec(),
            data: data.to_vec(),
        };

        let ciphertext = Self::decrypt_secret_key(decryption_params);

        let mut h = HmacSha256::new_from_slice(&derived_key_second_half).unwrap();
        h.update(&ciphertext);
        let mac = h.finalize().into_bytes();
        let keystore = Keystore {
            crypto: Crypto {
                cipher: CIPHER_ALGORITHM_AES_128_CTR.to_string(),
                cipherparams: CryptoParams { iv: iv_hex },
                ciphertext: hex::encode(&ciphertext),
                kdf: KDF_SCRYPT.to_string(),
                kdfparams: KdfParams {
                    salt: salt_hex,
                    n: KDF_N,
                    r: KDF_R,
                    p: KDF_P,
                    dklen: KDF_DKLEN as u32,
                },
                mac: hex::encode(mac),
            },
            id: Uuid::new_v4().to_string(),
            version: KEYSTORE_VERSION,
            kind: "secretKey".to_string(),
            address: public_key.to_string(),
            bech32: address.to_string(),
        };

        let mut keystore_json: String = serde_json::to_string_pretty(&keystore).unwrap();
        keystore_json.push('\n');
        keystore_json
    }

    pub fn generate_pem_content(address: &Address, private_key: &str, public_key: &str) -> String {
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
}
