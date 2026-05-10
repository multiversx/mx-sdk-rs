use std::fmt::Display;

use crate::{codec::*, types::Address};
use bech32::{Bech32, Hrp};
use serde::{Deserialize, Serialize};

const BECH32_PREFIX: &str = "bech32:";

const DEFAULT_HRP: &str = "erd";

/// Error type returned by [`Bech32Address::try_from_bech32_string`].
#[derive(Debug)]
pub enum Bech32AddressError {
    /// The string could not be decoded as a valid bech32 value.
    /// Contains a message with the offending input and the underlying error.
    DecodeError(String),
    /// The decoded payload is not exactly 32 bytes.
    /// Contains the actual decoded length.
    InvalidLength(usize),
}

impl core::fmt::Display for Bech32AddressError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Bech32AddressError::DecodeError(msg) => write!(f, "bech32 decode error: {msg}"),
            Bech32AddressError::InvalidLength(len) => {
                write!(
                    f,
                    "invalid address length after decoding: expected 32, got {len}"
                )
            }
        }
    }
}

impl std::error::Error for Bech32AddressError {}

/// Wraps and address, and presents it as a bech32 expression wherever possible.
///
/// In order to avoid repeated conversions, it redundantly keeps the bech32 representation inside.
///
/// Provides:
///- serde serialization/deserialization as bech32
/// - conversions to/from bech32
#[derive(Clone, PartialEq, Eq)]
pub struct Bech32Address {
    pub address: Address,
    pub hrp: String,
    pub bech32: String,
}

impl Bech32Address {
    /// Attempts to create a [`Bech32Address`] by decoding the given bech32 string.
    ///
    /// Returns an error if the string is not valid bech32 or if the decoded
    /// payload is not exactly 32 bytes.
    pub fn try_from_bech32_string(bech32_string: String) -> Result<Self, Bech32AddressError> {
        let (hrp, dest_address_bytes) = bech32::decode(&bech32_string)
            .map_err(|err| Bech32AddressError::DecodeError(format!("{bech32_string}: {err}")))?;
        if dest_address_bytes.len() != 32 {
            return Err(Bech32AddressError::InvalidLength(dest_address_bytes.len()));
        }

        Ok(Bech32Address {
            address: Address::from_slice(&dest_address_bytes),
            hrp: hrp.to_string(),
            bech32: bech32_string,
        })
    }

    /// Creates a [`Bech32Address`] by decoding the given bech32 string.
    ///
    /// # Panics
    ///
    /// Panics if the string is not valid bech32 or the decoded payload is not exactly 32 bytes.
    /// Use [`Self::try_from_bech32_string`] for a non-panicking alternative.
    pub fn from_bech32_string(bech32_string: String) -> Self {
        Self::try_from_bech32_string(bech32_string).unwrap_or_else(|err| panic!("{err}"))
    }

    /// Encodes an [`Address`] as bech32 using the given human-readable part (HRP).
    pub fn encode_address(hrp: &str, address: Address) -> Self {
        let hrp_obj = Hrp::parse(hrp).expect("invalid hrp");
        let bech32_string =
            bech32::encode::<Bech32>(hrp_obj, address.as_bytes()).expect("bech32 encode error");

        Bech32Address {
            address,
            hrp: hrp.to_owned(),
            bech32: bech32_string,
        }
    }

    /// Encodes an [`Address`] as bech32 using the default HRP (`"erd"`).
    pub fn encode_address_default_hrp(address: Address) -> Self {
        Self::encode_address(DEFAULT_HRP, address)
    }

    /// Returns the zero address encoded with the given HRP.
    pub fn zero(hrp: &str) -> Self {
        Bech32Address::encode_address(hrp, Address::zero())
    }

    /// Returns the zero address encoded with the default HRP (`"erd"`).
    pub fn zero_default_hrp() -> Self {
        Bech32Address::encode_address_default_hrp(Address::zero())
    }
}

impl Default for Bech32Address {
    fn default() -> Self {
        Self::zero_default_hrp()
    }
}

impl From<Address> for Bech32Address {
    fn from(value: Address) -> Self {
        Self::encode_address_default_hrp(value)
    }
}

impl From<&Address> for Bech32Address {
    fn from(value: &Address) -> Self {
        Self::encode_address_default_hrp(value.clone())
    }
}

impl Bech32Address {
    /// Returns the bech32 string as a borrowed `&str`.
    pub fn to_bech32_str(&self) -> &str {
        &self.bech32
    }

    /// Returns the bech32 string as an owned [`String`].
    pub fn to_bech32_string(&self) -> String {
        self.bech32.to_owned()
    }

    /// Returns a reference to the underlying [`Address`].
    pub fn as_address(&self) -> &Address {
        &self.address
    }

    /// Clones and returns the underlying [`Address`].
    pub fn to_address(&self) -> Address {
        self.address.clone()
    }

    /// Returns the human-readable part (HRP) as a borrowed `&str`.
    pub fn as_hrp(&self) -> &str {
        &self.hrp
    }

    /// Clones and returns the human-readable part (HRP) as an owned [`String`].
    pub fn to_hrp(&self) -> String {
        self.hrp.clone()
    }

    /// Consumes `self` and returns the underlying [`Address`].
    pub fn into_address(self) -> Address {
        self.address
    }

    /// Returns the address as a `bech32:`-prefixed expression (e.g. `"bech32:erd1..."`),
    /// used in scenario / mandos files.
    pub fn to_bech32_expr(&self) -> String {
        format!("{BECH32_PREFIX}{}", &self.bech32)
    }
}

impl NestedEncode for Bech32Address {
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.address.dep_encode_or_handle_err(dest, h)
    }
}

impl TopEncode for Bech32Address {
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.address.top_encode_or_handle_err(output, h)
    }
}

impl NestedDecode for Bech32Address {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(Bech32Address::from(Address::dep_decode_or_handle_err(
            input, h,
        )?))
    }
}

impl TopDecode for Bech32Address {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(Bech32Address::from(Address::top_decode_or_handle_err(
            input, h,
        )?))
    }
}

impl Serialize for Bech32Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.bech32.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Bech32Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // some old interactors have it serialized like this
        let mut bech32 = String::deserialize(deserializer)?;
        if let Some(stripped) = bech32.strip_prefix("bech32:") {
            bech32 = stripped.to_string();
        }
        Ok(Bech32Address::from_bech32_string(bech32))
    }
}

impl core::fmt::Debug for Bech32Address {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Bech32Address").field(&self.bech32).finish()
    }
}

impl Display for Bech32Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.bech32)
    }
}

impl PartialEq<&str> for Bech32Address {
    fn eq(&self, other: &&str) -> bool {
        &self.bech32 == other
    }
}

impl PartialEq<Address> for Bech32Address {
    fn eq(&self, other: &Address) -> bool {
        &self.address == other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // A known valid bech32 address on the MultiversX network (32-byte payload).
    const VALID_BECH32: &str = "erd1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq6gq4hu";

    #[test]
    fn test_try_from_bech32_string_valid() {
        let result = Bech32Address::try_from_bech32_string(VALID_BECH32.to_string());
        assert!(result.is_ok());
        let addr = result.unwrap();
        assert_eq!(addr.bech32, VALID_BECH32);
        assert_eq!(addr.hrp, "erd");
        assert_eq!(addr.address.as_bytes().len(), 32);
    }

    #[test]
    fn test_try_from_bech32_string_roundtrip() {
        let original = Bech32Address::try_from_bech32_string(VALID_BECH32.to_string()).unwrap();
        let re_encoded = Bech32Address::encode_address(&original.hrp, original.address.clone());
        assert_eq!(re_encoded.bech32, VALID_BECH32);
    }

    #[test]
    fn test_try_from_bech32_string_invalid_encoding() {
        let result = Bech32Address::try_from_bech32_string("not_a_valid_bech32!!!".to_string());
        assert!(matches!(result, Err(Bech32AddressError::DecodeError(_))));
    }

    #[test]
    fn test_try_from_bech32_string_wrong_length() {
        // Encode a payload that is not 32 bytes to trigger the length check.
        let hrp = bech32::Hrp::parse("erd").unwrap();
        let short_payload = [0u8; 10];
        let bad_bech32 = bech32::encode::<bech32::Bech32>(hrp, &short_payload).unwrap();
        let result = Bech32Address::try_from_bech32_string(bad_bech32);
        assert!(matches!(result, Err(Bech32AddressError::InvalidLength(10))));
    }
}
