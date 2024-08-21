use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum WalletError {
    InvalidPassword,
    InvalidKdf,
    InvalidCipher,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CryptoParams {
    pub iv: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KdfParams {
    pub dklen: u32,
    pub salt: String,
    pub n: u32,
    pub r: u32,
    pub p: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Crypto {
    pub ciphertext: String,
    pub cipherparams: CryptoParams,
    pub cipher: String,
    pub kdf: String,
    pub kdfparams: KdfParams,
    pub mac: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Keystore {
    pub version: u32,
    pub kind: String,
    pub id: String,
    pub address: String,
    pub bech32: String,
    pub crypto: Crypto,
}

#[derive(Clone, Debug)]
pub struct DecryptionParams {
    pub derived_key_first_half: Vec<u8>,
    pub iv: Vec<u8>,
    pub ciphertext: Vec<u8>,
}
