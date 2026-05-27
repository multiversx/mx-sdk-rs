use std::fs;
use std::path::Path;

use aes::{Aes128, cipher::KeyIvInit};
use ctr::{Ctr128BE, cipher::StreamCipher};
use hmac::{Hmac, KeyInit, Mac};
use multiversx_chain_core::std::Bech32Address;
use scrypt::{Params, scrypt};
use sha2::Sha256;

use crate::crypto::{private_key::PrivateKey, public_key::PublicKey};

use super::{KeystoreError, KeystoreJson};

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

/// Keystore with all fields decoded from their hex-string representation.
pub struct Keystore {
    pub version: u32,
    pub kind: String,
    pub address: String,
    pub bech32: String,
    pub cipher: String,
    pub ciphertext: Vec<u8>,
    pub kdf: String,
    pub n: u32,
    pub r: u32,
    pub p: u32,
    pub dklen: u32,
    pub mac: Vec<u8>,
    pub randomness: KeystoreRandomness,
}

impl Keystore {
    pub fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let json_str = fs::read_to_string(path)?;
        let json: KeystoreJson = serde_json::from_str(&json_str)?;
        Self::from_json(json)
    }

    pub fn to_json_string(&self) -> String {
        let mut s = serde_json::to_string_pretty(&self.to_json()).unwrap();
        s.push('\n');
        s
    }

    pub fn extract_private_key(&self, password: &str) -> Result<PrivateKey, KeystoreError> {
        if self.cipher != CIPHER_ALGORITHM_AES_128_CTR {
            return Err(KeystoreError::InvalidCipher);
        }
        if self.kdf != KDF_SCRYPT {
            return Err(KeystoreError::InvalidKdf);
        }

        let n = self.n as f64;
        let params = Params::new(n.log2() as u8, self.r, self.p)
            .map_err(|e| KeystoreError::Other(e.into()))?;

        let mut derived_key = [0u8; 32];
        scrypt(
            password.as_bytes(),
            &self.randomness.salt,
            &params,
            &mut derived_key,
        )
        .unwrap();

        let (derived_key_first_half, derived_key_second_half) = split_derived_key(&derived_key);

        let mut input_mac = HmacSha256::new_from_slice(&derived_key_second_half).unwrap();
        input_mac.update(&self.ciphertext);
        let computed_mac = input_mac.finalize().into_bytes();

        if computed_mac.to_vec() != self.mac {
            return Err(KeystoreError::InvalidPassword);
        }

        let private_key_bytes = run_cipher(
            derived_key_first_half,
            self.randomness.iv,
            self.ciphertext.clone(),
        );
        PrivateKey::from_bytes(&private_key_bytes).map_err(Into::into)
    }

    /// Not available in dapps, since it uses randomness to generate the keystore.
    ///
    /// Only available in the sc-meta standalone CLI.
    pub fn encrypt(
        private_key: PrivateKey,
        bech32_address: Bech32Address,
        password: &str,
        randomness: KeystoreRandomness,
    ) -> Self {
        let private_key_bytes = private_key.to_bytes();
        let public_key_hex = PublicKey::from(&private_key).to_string();

        let params = Params::new((KDF_N as f64).log2() as u8, KDF_R, KDF_P).unwrap();

        let mut derived_key = [0u8; 32];
        scrypt(
            password.as_bytes(),
            &randomness.salt,
            &params,
            &mut derived_key,
        )
        .unwrap();

        let (derived_key_first_half, derived_key_second_half) = split_derived_key(&derived_key);

        let ciphertext = run_cipher(
            derived_key_first_half,
            randomness.iv,
            private_key_bytes.to_vec(),
        );

        let mut h = HmacSha256::new_from_slice(&derived_key_second_half).unwrap();
        h.update(&ciphertext);
        let mac = h.finalize().into_bytes().to_vec();

        Keystore {
            version: KEYSTORE_VERSION,
            kind: KIND_SECRET_KEY.to_string(),
            address: public_key_hex,
            bech32: bech32_address.bech32,
            cipher: CIPHER_ALGORITHM_AES_128_CTR.to_string(),
            ciphertext,
            kdf: KDF_SCRYPT.to_string(),
            n: KDF_N,
            r: KDF_R,
            p: KDF_P,
            dklen: KDF_DKLEN as u32,
            mac,
            randomness,
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

/// Splits a 32-byte scrypt derived key into two 16-byte halves.
///
/// The first half is used as the AES-128 cipher key; the second half is
/// used as the HMAC-SHA256 key for MAC verification.
fn split_derived_key(derived_key: &[u8; 32]) -> ([u8; 16], [u8; 16]) {
    let mut first_half = [0u8; 16];
    let mut second_half = [0u8; 16];
    first_half.copy_from_slice(&derived_key[..16]);
    second_half.copy_from_slice(&derived_key[16..]);
    (first_half, second_half)
}
