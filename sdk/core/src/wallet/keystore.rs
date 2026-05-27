use std::fs;
use std::path::Path;

use aes::{Aes128, cipher::KeyIvInit};
use anyhow::Result;
use ctr::{Ctr128BE, cipher::StreamCipher};
use hmac::{Hmac, KeyInit, Mac};
use multiversx_chain_core::std::Bech32Address;
use scrypt::{Params, scrypt};
use sha2::Sha256;

use crate::crypto::private_key::PrivateKey;

use super::{
    Crypto, CryptoParams, DecryptionParams, KDF_DKLEN, KDF_N, KDF_P, KDF_R, KEYSTORE_VERSION,
    KdfParams, KeystoreError, KeystoreJson,
};

const CIPHER_ALGORITHM_AES_128_CTR: &str = "aes-128-ctr";
const KDF_SCRYPT: &str = "scrypt";

type HmacSha256 = Hmac<Sha256>;

/// Groups all randomness inputs required to encrypt a keystore.
///
/// Keeping these separate from [`Keystore::encrypt`] makes the function
/// deterministic and easy to test with fixed values.
pub struct KeystoreRandomness {
    pub salt: [u8; 32],
    pub iv: [u8; 16],
    pub id: String,
}

/// Wraps a keystore JSON payload and provides encryption/decryption helpers.
///
/// The `json` field holds the raw [`KeystoreJson`] that is serialised to disk.
pub struct Keystore {
    pub json: KeystoreJson,
}

impl Keystore {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        let json_body = fs::read_to_string(path).unwrap();
        Keystore {
            json: serde_json::from_str(&json_body).unwrap(),
        }
    }

    pub fn to_json_string(&self) -> String {
        let mut s = serde_json::to_string_pretty(&self.json).unwrap();
        s.push('\n');
        s
    }

    pub fn validate_password(&self, password: &str) -> Result<DecryptionParams, KeystoreError> {
        let ciphertext = hex::decode(&self.json.crypto.ciphertext).unwrap();

        let cipher = &self.json.crypto.cipher;
        if cipher != CIPHER_ALGORITHM_AES_128_CTR {
            return Err(KeystoreError::InvalidCipher);
        }

        let iv = hex::decode(&self.json.crypto.cipherparams.iv).unwrap();
        let salt = hex::decode(&self.json.crypto.kdfparams.salt).unwrap();
        let json_mac = hex::decode(&self.json.crypto.mac).unwrap();

        let kdf = &self.json.crypto.kdf;
        if kdf != KDF_SCRYPT {
            return Err(KeystoreError::InvalidKdf);
        }
        let n = self.json.crypto.kdfparams.n as f64;
        let r = self.json.crypto.kdfparams.r as u64;
        let p = self.json.crypto.kdfparams.p as u64;
        let _dklen = self.json.crypto.kdfparams.dklen as usize;

        let params = Params::new(n.log2() as u8, r as u32, p as u32).unwrap();

        let mut derived_key = vec![0u8; 32];
        scrypt(password.as_bytes(), &salt, &params, &mut derived_key).unwrap();

        let derived_key_first_half = derived_key[0..16].to_vec();
        let derived_key_second_half = derived_key[16..32].to_vec();

        let mut input_mac = HmacSha256::new_from_slice(&derived_key_second_half).unwrap();
        input_mac.update(&ciphertext);
        let computed_mac = input_mac.finalize().into_bytes();

        if computed_mac.to_vec() == json_mac {
            println!("Password is correct");
            Ok(DecryptionParams {
                derived_key_first_half,
                iv,
                data: ciphertext,
            })
        } else {
            println!("Password is incorrect");
            Err(KeystoreError::InvalidPassword)
        }
    }

    pub fn decrypt_secret_key(decryption_params: DecryptionParams) -> Vec<u8> {
        let key: &[u8; 16] = decryption_params
            .derived_key_first_half
            .as_slice()
            .try_into()
            .unwrap();
        let iv: &[u8; 16] = decryption_params.iv.as_slice().try_into().unwrap();
        let mut cipher = Ctr128BE::<Aes128>::new(key.into(), iv.into());
        let mut decrypted = decryption_params.data.to_vec();
        cipher.apply_keystream(&mut decrypted);

        decrypted
    }

    pub fn get_private_key_from_file<P: AsRef<Path>>(
        file_path: P,
        password: &str,
    ) -> Result<PrivateKey> {
        let decryption_params = Self::from_file(file_path)
            .validate_password(password)
            .unwrap_or_else(|e| {
                panic!("Error: {:?}", e);
            });
        let priv_key = PrivateKey::from_hex_str(
            hex::encode(Self::decrypt_secret_key(decryption_params)).as_str(),
        )?;
        Ok(priv_key)
    }

    /// Not available in dapps, since it uses randomness to generate the keystore.
    ///
    /// Only available in the sc-meta standalone CLI.
    pub fn encrypt(
        data: &[u8],
        bech32_address: Bech32Address,
        public_key: &str,
        password: &str,
        randomness: KeystoreRandomness,
    ) -> Self {
        let params = Params::new((KDF_N as f64).log2() as u8, KDF_R, KDF_P).unwrap();
        let salt_hex = hex::encode(randomness.salt);
        let iv_hex = hex::encode(randomness.iv);

        let mut derived_key = vec![0u8; 32];
        scrypt(
            password.as_bytes(),
            &randomness.salt,
            &params,
            &mut derived_key,
        )
        .unwrap();

        let derived_key_first_half = derived_key[0..16].to_vec();
        let derived_key_second_half = derived_key[16..32].to_vec();

        let decryption_params = DecryptionParams {
            derived_key_first_half,
            iv: randomness.iv.to_vec(),
            data: data.to_vec(),
        };

        let ciphertext = Self::decrypt_secret_key(decryption_params);

        let mut h = HmacSha256::new_from_slice(&derived_key_second_half).unwrap();
        h.update(&ciphertext);
        let mac = h.finalize().into_bytes();
        Keystore {
            json: KeystoreJson {
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
                id: randomness.id,
                version: KEYSTORE_VERSION,
                kind: "secretKey".to_string(),
                address: public_key.to_string(),
                bech32: bech32_address.bech32,
            },
        }
    }
}
