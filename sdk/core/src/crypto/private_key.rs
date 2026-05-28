use std::fmt::Display;

use anyhow::{Result, anyhow};
use bip39::Mnemonic;
use hmac::{Hmac, KeyInit, Mac};
use multiversx_chain_core::std::crypto::ed25519;
use pbkdf2::pbkdf2;
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};
use sha2::Sha512;
use zeroize::Zeroize;

use super::wallet_signature::WalletSignature;

pub const PRIVATE_KEY_LENGTH: usize = 64;
pub const SIGNATURE_LENGTH: usize = 64;
pub const SEED_LENGTH: usize = 32;

const EGLD_COIN_TYPE: u32 = 508;
const HARDENED: u32 = 0x80000000;

#[derive(Clone, PartialEq, Eq)]
pub struct PrivateKey(pub ed25519::Ed25519SigningKey);

impl PrivateKey {
    pub fn from_bytes(bytes: &[u8]) -> Result<PrivateKey> {
        match bytes.len() {
            SEED_LENGTH => {
                let seed: [u8; 32] = bytes.try_into().unwrap();
                Ok(PrivateKey(ed25519::Ed25519SigningKey::from_seed(&seed)))
            }
            PRIVATE_KEY_LENGTH => {
                let keypair: &[u8; 64] = bytes.try_into().unwrap();
                ed25519::Ed25519SigningKey::from_keypair_bytes(keypair)
                    .map(PrivateKey)
                    .map_err(|e| anyhow!("Invalid keypair bytes: {e}"))
            }
            _ => Err(anyhow!("Invalid secret key length")),
        }
    }

    pub fn from_hex_str(pk: &str) -> Result<Self> {
        let bytes = hex::decode(pk)?;
        PrivateKey::from_bytes(bytes.as_slice())
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

    pub fn from_mnemonic(mnemonic: Mnemonic, account: u32, address_index: u32) -> PrivateKey {
        let seed = Self::seed_from_mnemonic(mnemonic, "");

        let serialized_key_len = 32;
        let hardened_child_padding: u8 = 0;

        let mut digest =
            Hmac::<Sha512>::new_from_slice(b"ed25519 seed").expect("HMAC can take key of any size");
        digest.update(&seed);
        let intermediary: Vec<u8> = digest.finalize().into_bytes().into_iter().collect();
        let mut key = intermediary[..serialized_key_len].to_vec();
        let mut chain_code = intermediary[serialized_key_len..].to_vec();

        for child_idx in [
            44 | HARDENED,
            EGLD_COIN_TYPE | HARDENED,
            account | HARDENED,
            HARDENED,
            address_index | HARDENED,
        ] {
            let mut buff = [vec![hardened_child_padding], key.clone()].concat();
            buff.push((child_idx >> 24) as u8);
            buff.push((child_idx >> 16) as u8);
            buff.push((child_idx >> 8) as u8);
            buff.push(child_idx as u8);

            digest =
                Hmac::<Sha512>::new_from_slice(&chain_code).expect("HMAC can take key of any size");
            digest.update(&buff);
            let intermediary: Vec<u8> = digest.finalize().into_bytes().into_iter().collect();
            key = intermediary[..serialized_key_len].to_vec();
            chain_code = intermediary[serialized_key_len..].to_vec();
        }

        PrivateKey::from_bytes(key.as_slice()).unwrap()
    }

    pub fn to_bytes(&self) -> [u8; PRIVATE_KEY_LENGTH] {
        self.0.to_keypair_bytes()
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.0.to_seed_bytes())
    }

    pub fn sign(&self, message: impl AsRef<[u8]>) -> WalletSignature {
        WalletSignature::from(self.0.sign(message.as_ref()))
    }
}

impl Display for PrivateKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_hex().fmt(f)
    }
}

impl std::fmt::Debug for PrivateKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PrivateKey({})", self)
    }
}

impl Serialize for PrivateKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_hex().as_str())
    }
}

impl<'de> Deserialize<'de> for PrivateKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self::from_hex_str(s.as_str()).unwrap())
    }
}
