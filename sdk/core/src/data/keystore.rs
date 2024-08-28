use serde::{Deserialize, Serialize};

pub const KDF_N: u32 = 4096;
pub const KDF_R: u32 = 8;
pub const KDF_P: u32 = 1;
pub const KDF_DKLEN: usize = 32;
pub const KEYSTORE_VERSION: u32 = 4;

#[derive(Debug)]
pub enum WalletError {
    InvalidPassword,
    InvalidKdf,
    InvalidCipher,
}

#[derive(Debug)]
pub enum InsertPassword {
    Plaintext(String),
    StandardInput,
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
    pub data: Vec<u8>,
}
