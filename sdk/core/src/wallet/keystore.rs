use std::fs;
use std::path::Path;

use aes::{Aes128, cipher::KeyIvInit};
use ctr::{Ctr128BE, cipher::StreamCipher};
use hmac::{Hmac, KeyInit, Mac};
use multiversx_chain_core::std::Bech32Address;
use scrypt::{Params, scrypt};
use sha2::Sha256;

use crate::crypto::private_key::PrivateKey;

use super::{Crypto, CryptoParams, KdfParams, KeystoreError, KeystoreJson};

const KDF_N: u32 = 4096;
const KDF_R: u32 = 8;
const KDF_P: u32 = 1;
const KDF_DKLEN: usize = 32;
const KEYSTORE_VERSION: u32 = 4;
const CIPHER_ALGORITHM_AES_128_CTR: &str = "aes-128-ctr";
const KDF_SCRYPT: &str = "scrypt";
const KIND_SECRET_KEY: &str = "secretKey";

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

    pub fn extract_private_key(&self, password: &str) -> Result<PrivateKey, KeystoreError> {
        let ciphertext = hex::decode(&self.json.crypto.ciphertext)?;

        let cipher = &self.json.crypto.cipher;
        if cipher != CIPHER_ALGORITHM_AES_128_CTR {
            return Err(KeystoreError::InvalidCipher);
        }

        let kdf = &self.json.crypto.kdf;
        if kdf != KDF_SCRYPT {
            return Err(KeystoreError::InvalidKdf);
        }

        let iv_bytes = hex::decode(&self.json.crypto.cipherparams.iv)?;
        let iv: [u8; 16] = iv_bytes
            .as_slice()
            .try_into()
            .map_err(|e: std::array::TryFromSliceError| KeystoreError::Other(e.into()))?;
        let salt = hex::decode(&self.json.crypto.kdfparams.salt)?;
        let json_mac = hex::decode(&self.json.crypto.mac)?;

        let n = self.json.crypto.kdfparams.n as f64;
        let r = self.json.crypto.kdfparams.r as u64;
        let p = self.json.crypto.kdfparams.p as u64;
        let _dklen = self.json.crypto.kdfparams.dklen as usize;

        let params = Params::new(n.log2() as u8, r as u32, p as u32)
            .map_err(|e| KeystoreError::Other(e.into()))?;

        let mut derived_key = vec![0u8; 32];
        scrypt(password.as_bytes(), &salt, &params, &mut derived_key).unwrap();

        let derived_key_first_half: [u8; 16] = derived_key[0..16].try_into().unwrap();
        let derived_key_second_half = &derived_key[16..32];

        let mut input_mac = HmacSha256::new_from_slice(derived_key_second_half).unwrap();
        input_mac.update(&ciphertext);
        let computed_mac = input_mac.finalize().into_bytes();

        if computed_mac.to_vec() != json_mac {
            println!("Password is incorrect");
            return Err(KeystoreError::InvalidPassword);
        }

        println!("Password is correct");
        let private_key_bytes = run_cipher(derived_key_first_half, iv, ciphertext);
        PrivateKey::from_bytes(&private_key_bytes).map_err(Into::into)
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

        let derived_key_first_half: [u8; 16] = derived_key[0..16].try_into().unwrap();
        let derived_key_second_half = derived_key[16..32].to_vec();

        let ciphertext = run_cipher(derived_key_first_half, randomness.iv, data.to_vec());

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
                kind: KIND_SECRET_KEY.to_string(),
                address: public_key.to_string(),
                bech32: bech32_address.bech32,
            },
        }
    }
}

/// Applies AES-128-CTR to `data` in place and returns the result.
///
/// AES-128-CTR is a symmetric stream cipher, so the same operation both
/// encrypts plaintext and decrypts ciphertext.
fn run_cipher(key: [u8; 16], iv: [u8; 16], mut data: Vec<u8>) -> Vec<u8> {
    let mut cipher = Ctr128BE::<Aes128>::new((&key).into(), (&iv).into());
    cipher.apply_keystream(&mut data);
    data
}
