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
    /// Constructs a [`PrivateKey`] from a 32-byte ed25519 seed.
    ///
    /// The corresponding public (verifying) key is derived automatically.
    /// This is the canonical representation used throughout the MultiversX SDK.
    pub fn from_seed_bytes(bytes: &[u8; 32]) -> PrivateKey {
        PrivateKey(ed25519::Ed25519SigningKey::from_seed(bytes))
    }

    /// Constructs a [`PrivateKey`] from a 64-byte keypair `[seed || public_key]`.
    ///
    /// Returns an error if the embedded public key does not match the seed.
    pub fn from_keypair_bytes(bytes: &[u8; 64]) -> Result<PrivateKey> {
        ed25519::Ed25519SigningKey::from_keypair_bytes(bytes)
            .map(PrivateKey)
            .map_err(|e| anyhow!("Invalid keypair bytes: {e}"))
    }

    /// Constructs a [`PrivateKey`] from a slice whose length determines the format:
    /// - 32 bytes → treated as a seed (see [`from_seed_bytes`](Self::from_seed_bytes))
    /// - 64 bytes → treated as a keypair (see [`from_keypair_bytes`](Self::from_keypair_bytes))
    ///
    /// Returns an error for any other length.
    pub fn from_bytes(bytes: &[u8]) -> Result<PrivateKey> {
        match bytes.len() {
            SEED_LENGTH => {
                let seed: &[u8; 32] = bytes
                    .try_into()
                    .map_err(|_| anyhow!("Invalid secret key length"))?;
                Ok(PrivateKey::from_seed_bytes(seed))
            }
            PRIVATE_KEY_LENGTH => {
                let keypair: &[u8; 64] = bytes
                    .try_into()
                    .map_err(|_| anyhow!("Invalid secret key length"))?;
                PrivateKey::from_keypair_bytes(keypair)
            }
            _ => Err(anyhow!("Invalid secret key length")),
        }
    }

    /// Decodes a hex string into a 32-byte seed and constructs a [`PrivateKey`].
    ///
    /// The input must be exactly 64 hex characters (32 bytes). Returns an error
    /// if the string is not valid hex or does not decode to exactly 32 bytes.
    pub fn from_seed_hex_str(pk: &str) -> Result<Self> {
        let bytes = hex::decode(pk)?;
        let seed: [u8; 32] = bytes
            .try_into()
            .map_err(|_| anyhow!("Invalid seed key length"))?;
        Ok(PrivateKey::from_seed_bytes(&seed))
    }

    /// Decodes a hex string into a 64-byte keypair and constructs a [`PrivateKey`].
    ///
    /// The input must be exactly 128 hex characters (64 bytes). Returns an error
    /// if the string is not valid hex, does not decode to exactly 64 bytes, or
    /// the embedded public key does not match the seed.
    pub fn from_keypair_hex_str(pk: &str) -> Result<Self> {
        let bytes = hex::decode(pk)?;
        let keypair: [u8; 64] = bytes
            .try_into()
            .map_err(|_| anyhow!("Invalid keypair key length"))?;
        PrivateKey::from_keypair_bytes(&keypair)
    }

    /// Decodes a hex string and constructs a [`PrivateKey`], inferring the format
    /// from the decoded length (32 bytes → seed, 64 bytes → keypair).
    ///
    /// Prefer [`from_seed_hex_str`](Self::from_seed_hex_str) or
    /// [`from_keypair_hex_str`](Self::from_keypair_hex_str) when the format is known.
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

    /// Derives a [`PrivateKey`] from a BIP-39 mnemonic using the MultiversX HD path
    /// `m/44'/508'/<account>'/0'/<address_index>'`.
    ///
    /// Returns an error if the internal BIP-32 key derivation produces a key of
    /// unexpected length (should not happen in practice).
    pub fn from_mnemonic(
        mnemonic: Mnemonic,
        account: u32,
        address_index: u32,
    ) -> Result<PrivateKey> {
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

        let seed: &[u8; 32] = key
            .as_slice()
            .try_into()
            .map_err(|_| anyhow!("BIP32 derived key has unexpected length"))?;
        Ok(PrivateKey::from_seed_bytes(seed))
    }

    /// Returns the full 64-byte keypair as `[seed (32 bytes) || public_key (32 bytes)]`.
    pub fn to_bytes(&self) -> [u8; PRIVATE_KEY_LENGTH] {
        self.0.to_keypair_bytes()
    }

    /// Returns the 32-byte seed encoded as a lowercase hex string (64 characters).
    pub fn to_seed_hex(&self) -> String {
        hex::encode(self.0.to_seed_bytes())
    }

    /// Signs `message` with this key and returns a [`WalletSignature`].
    pub fn sign(&self, message: impl AsRef<[u8]>) -> WalletSignature {
        WalletSignature::from(self.0.sign(message.as_ref()))
    }
}

impl Display for PrivateKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_seed_hex().fmt(f)
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
        serializer.serialize_str(self.to_seed_hex().as_str())
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
