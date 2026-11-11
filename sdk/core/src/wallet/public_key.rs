use std::fmt::Display;

use super::PrivateKey;
use super::wallet_signature::WalletSignature;
use anyhow::Result;
use multiversx_chain_core::std::crypto::ed25519;
use multiversx_chain_core::types::Address;
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};

pub const PUBLIC_KEY_LENGTH: usize = 32;

#[derive(Copy, Clone)]
pub struct PublicKey(ed25519::Ed25519VerifyingKey);

impl PublicKey {
    /// Returns the raw 32-byte public key.
    pub fn to_bytes(&self) -> [u8; PUBLIC_KEY_LENGTH] {
        self.0.to_bytes()
    }

    /// Returns a reference to the raw 32-byte public key.
    pub fn as_bytes(&self) -> &[u8; PUBLIC_KEY_LENGTH] {
        self.0.as_bytes()
    }

    /// Derives the MultiversX [`Address`] from this public key.
    ///
    /// The address is the raw 32-byte public key interpreted as an address.
    pub fn to_address(&self) -> Address {
        (*self.0.as_bytes()).into()
    }

    /// Returns the public key encoded as a lowercase hex string (64 characters).
    pub fn to_hex(&self) -> String {
        hex::encode(self.0.as_bytes())
    }

    /// Decodes a 64-character hex string into a [`PublicKey`].
    ///
    /// Returns an error if the string is not valid hex, does not decode to
    /// exactly 32 bytes, or the bytes do not represent a valid ed25519 point.
    pub fn from_hex_str(pk: &str) -> Result<Self> {
        let bytes = hex::decode(pk)?;
        let bits: [u8; PUBLIC_KEY_LENGTH] = bytes
            .try_into()
            .map_err(|_| anyhow::anyhow!("invalid public key length, expected 32 bytes"))?;
        ed25519::Ed25519VerifyingKey::from_bytes(&bits)
            .map(PublicKey)
            .ok_or_else(|| anyhow::anyhow!("invalid ed25519 public key"))
    }

    /// Verifies that `signature` is a valid ed25519 signature over `message`
    /// produced by the private key corresponding to this public key.
    pub fn verify(&self, message: &[u8], signature: &WalletSignature) -> bool {
        self.0.verify(message, signature.inner())
    }
}

impl From<&PrivateKey> for PublicKey {
    fn from(private_key: &PrivateKey) -> PublicKey {
        PublicKey(private_key.0.verifying_key())
    }
}

impl Display for PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_hex().fmt(f)
    }
}

impl std::fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PublicKey({})", self)
    }
}

impl Serialize for PublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_hex().as_str())
    }
}

impl<'de> Deserialize<'de> for PublicKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self::from_hex_str(s.as_str()).unwrap())
    }
}
