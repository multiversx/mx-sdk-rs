use serde::{Deserialize, Serialize};

use multiversx_chain_core::std::Bech32Address;

use super::{Keystore, KeystoreRandomness};

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
pub struct KeystoreJson {
    pub version: u32,
    pub kind: String,
    pub id: String,
    pub address: String,
    pub bech32: String,
    pub crypto: Crypto,
}

impl Keystore {
    pub fn from_json(json: KeystoreJson) -> anyhow::Result<Self> {
        let ciphertext = hex::decode(&json.crypto.ciphertext)?;
        let iv_bytes = hex::decode(&json.crypto.cipherparams.iv)?;
        let iv: [u8; 16] = iv_bytes
            .as_slice()
            .try_into()
            .map_err(|_: std::array::TryFromSliceError| anyhow::anyhow!("iv must be 16 bytes"))?;
        let salt_bytes = hex::decode(&json.crypto.kdfparams.salt)?;
        let salt: [u8; 32] = salt_bytes
            .as_slice()
            .try_into()
            .map_err(|_: std::array::TryFromSliceError| anyhow::anyhow!("salt must be 32 bytes"))?;
        let mac = hex::decode(&json.crypto.mac)?;
        Ok(Keystore {
            version: json.version,
            kind: json.kind,
            address: Bech32Address::from_bech32_str(&json.bech32),
            cipher: json.crypto.cipher,
            ciphertext,
            kdf: json.crypto.kdf,
            n: json.crypto.kdfparams.n,
            r: json.crypto.kdfparams.r,
            p: json.crypto.kdfparams.p,
            dklen: json.crypto.kdfparams.dklen,
            mac,
            randomness: KeystoreRandomness {
                salt,
                iv,
                id: json.id,
            },
        })
    }

    pub fn to_json(&self) -> KeystoreJson {
        KeystoreJson {
            version: self.version,
            kind: self.kind.clone(),
            id: self.randomness.id.clone(),
            address: self.address.address.to_hex(),
            bech32: self.address.bech32.clone(),
            crypto: Crypto {
                cipher: self.cipher.clone(),
                cipherparams: CryptoParams {
                    iv: hex::encode(self.randomness.iv),
                },
                ciphertext: hex::encode(&self.ciphertext),
                kdf: self.kdf.clone(),
                kdfparams: KdfParams {
                    salt: hex::encode(self.randomness.salt),
                    n: self.n,
                    r: self.r,
                    p: self.p,
                    dklen: self.dklen,
                },
                mac: hex::encode(&self.mac),
            },
        }
    }
}
