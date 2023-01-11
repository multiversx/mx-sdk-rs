use super::private_key::PrivateKey;
use anyhow::Result;
use bech32::{self, ToBase32, Variant};
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};

pub const PUBLIC_KEY_LENGTH: usize = 32;

#[derive(Copy, Clone, Debug)]
pub struct PublicKey([u8; PUBLIC_KEY_LENGTH]);

impl PublicKey {
    pub fn to_bytes(&self) -> [u8; PUBLIC_KEY_LENGTH] {
        self.0
    }

    pub fn as_bytes(&self) -> &[u8; PUBLIC_KEY_LENGTH] {
        &self.0
    }

    pub fn to_address(&self) -> Result<String> {
        let address = bech32::encode("erd", self.0.to_base32(), Variant::Bech32)?;
        Ok(address)
    }

    pub fn from_hex_str(pk: &str) -> Result<Self> {
        let bytes = hex::decode(pk)?;
        let mut bits: [u8; 32] = [0u8; 32];
        bits.copy_from_slice(&bytes[32..]);
        Ok(Self(bits))
    }
}

impl<'a> From<&'a PrivateKey> for PublicKey {
    fn from(private_key: &PrivateKey) -> PublicKey {
        let bytes = private_key.to_bytes();

        let mut bits: [u8; 32] = [0u8; 32];
        bits.copy_from_slice(&bytes[32..]);

        PublicKey(bits)
    }
}

impl ToString for PublicKey {
    fn to_string(&self) -> String {
        hex::encode(self.0)
    }
}

impl Serialize for PublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
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
