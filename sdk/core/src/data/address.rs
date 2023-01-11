use std::fmt::Debug;

use crate::crypto::public_key::PublicKey;
use anyhow::Result;
use bech32::{FromBase32, ToBase32, Variant};
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};

#[derive(Clone)]
pub struct Address([u8; 32]);

impl Address {
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        self.0
    }

    pub fn from_bech32_string(bech32: &str) -> Result<Self> {
        let (_, data, _) = bech32::decode(bech32)?;
        let data = Vec::<u8>::from_base32(&data)?;

        let mut bits: [u8; 32] = [0u8; 32];
        bits.copy_from_slice(&data);

        Ok(Self(bits))
    }

    pub fn to_bech32_string(&self) -> Result<String> {
        let address = bech32::encode("erd", self.0.to_base32(), Variant::Bech32)?;
        Ok(address)
    }

    pub fn is_valid(&self) -> bool {
        self.0.len() == 32
    }
}

impl<'a> From<&'a PublicKey> for Address {
    fn from(public_key: &PublicKey) -> Address {
        let bytes = public_key.to_bytes();

        let mut bits: [u8; 32] = [0u8; 32];
        bits.copy_from_slice(&bytes);

        Address(bits)
    }
}

impl ToString for Address {
    fn to_string(&self) -> String {
        self.to_bech32_string().unwrap()
    }
}

impl Debug for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_bech32_string().unwrap().as_str())
    }
}

impl Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_bech32_string().unwrap().as_str())
    }
}

impl<'de> Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self::from_bech32_string(s.as_str()).unwrap())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_decode_address() {
        let addr = Address::from_bech32_string(
            "erd1qqqqqqqqqqqqqpgqyfjjn43spw7teklwtpz4x5waygq2mluyj9ts0mdwn6",
        )
        .unwrap();
        let encode = hex::encode(addr.to_bytes());
        assert_eq!(
            encode,
            "00000000000000000500226529d6300bbcbcdbee58455351dd2200adff849157"
        );
    }
}
