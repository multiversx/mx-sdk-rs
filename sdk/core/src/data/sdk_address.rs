use std::fmt::{Debug, Display};

use anyhow::Result;
use multiversx_chain_core::types::Address;
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};

/// Wrapper around a regular Address.
///
/// Provides:
/// - serde serialization/deserialization as bech32
/// - conversions to/from bech32
///
/// It should only be used in the sdk, to serialize/deserialize to JSON as bech32.
///
/// It exists primarily because it is currently inconvenient to provide
/// bech32 and serde functionality to the base Address directly.
#[derive(Clone)]
pub struct SdkAddress(pub Address);

impl SdkAddress {
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(Address::from(bytes))
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        *self.0.as_array()
    }

    pub fn from_bech32_string(bech32: &str) -> Result<Self> {
        Ok(SdkAddress(crate::bech32::decode(bech32)))
    }

    pub fn to_bech32_string(&self) -> Result<String> {
        Ok(crate::bech32::encode(&self.0))
    }
}

impl From<multiversx_chain_core::types::Address> for SdkAddress {
    fn from(value: multiversx_chain_core::types::Address) -> Self {
        SdkAddress(value)
    }
}

impl From<SdkAddress> for multiversx_chain_core::types::Address {
    fn from(value: SdkAddress) -> Self {
        value.0
    }
}

impl Display for SdkAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_bech32_string().unwrap().as_str())
    }
}

impl Debug for SdkAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_bech32_string().unwrap().as_str())
    }
}

impl Default for SdkAddress {
    fn default() -> Self {
        SdkAddress(Address::zero())
    }
}

impl Serialize for SdkAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_bech32_string().unwrap().as_str())
    }
}

impl<'de> Deserialize<'de> for SdkAddress {
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
        let addr = SdkAddress::from_bech32_string(
            "erd1qqqqqqqqqqqqqpgqyfjjn43spw7teklwtpz4x5waygq2mluyj9ts0mdwn6",
        )
        .unwrap();
        let encode = hex::encode(addr.0.as_bytes());
        assert_eq!(
            encode,
            "00000000000000000500226529d6300bbcbcdbee58455351dd2200adff849157"
        );
    }
}
