use std::fmt;

use anyhow::Result;
use multiversx_chain_core::std::crypto::ed25519;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// An Ed25519 signature, serialized as a lowercase hex string.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct WalletSignature(ed25519::Ed25519Signature);

impl WalletSignature {
    pub fn from_bytes(bytes: [u8; 64]) -> Self {
        Self(ed25519::Ed25519Signature::from_bytes(&bytes))
    }

    pub fn to_bytes(&self) -> [u8; 64] {
        self.0.to_bytes()
    }

    pub fn inner(&self) -> &ed25519::Ed25519Signature {
        &self.0
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.to_bytes())
    }

    pub fn from_hex_str(s: &str) -> Result<Self> {
        let bytes = hex::decode(s)?;
        let bits: [u8; 64] = bytes
            .try_into()
            .map_err(|_| anyhow::anyhow!("invalid signature length, expected 64 bytes"))?;
        Ok(Self::from_bytes(bits))
    }
}

impl From<[u8; 64]> for WalletSignature {
    fn from(bytes: [u8; 64]) -> Self {
        Self::from_bytes(bytes)
    }
}

impl From<ed25519::Ed25519Signature> for WalletSignature {
    fn from(sig: ed25519::Ed25519Signature) -> Self {
        Self(sig)
    }
}

impl fmt::Display for WalletSignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_hex().fmt(f)
    }
}

impl fmt::Debug for WalletSignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WalletSignature({})", self)
    }
}

impl Serialize for WalletSignature {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_hex())
    }
}

impl<'de> Deserialize<'de> for WalletSignature {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_hex_str(&s).map_err(serde::de::Error::custom)
    }
}
