use std::fmt::Display;
use std::str::FromStr;

use crate::{codec::*, types::Address};
use bech32::{Bech32, Hrp};
use serde::{Deserialize, Serialize};

const BECH32_PREFIX: &str = "bech32:";

const DEFAULT_HRP: &str = "erd";

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
    /// Decodes a bech32 string (owned) into a `Bech32Address`.
    /// Panics if the string is not valid bech32 or does not decode to a 32-byte address.
    pub fn from_bech32_string(bech32_string: String) -> Self {
        let (hrp, dest_address_bytes) = bech32::decode(&bech32_string)
            .unwrap_or_else(|err| panic!("bech32 decode error for {bech32_string}: {err}"));
        if dest_address_bytes.len() != 32 {
            panic!("Invalid address length after decoding")
        }

        Bech32Address {
            address: Address::from_slice(&dest_address_bytes),
            hrp: hrp.to_string(),
            bech32: bech32_string,
        }
    }

    /// Decodes a bech32 string slice into a `Bech32Address`.
    /// Convenience wrapper around [`Self::from_bech32_string`] that clones the input.
    /// Panics if the string is not valid bech32 or does not decode to a 32-byte address.
    pub fn from_bech32_str(bech32_str: &str) -> Self {
        Self::from_bech32_string(bech32_str.to_string())
    }

    /// Encodes an address with an explicit HRP into a `Bech32Address`.
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

    /// Encodes an address using the default HRP (`erd`).
    pub fn encode_address_default_hrp(address: Address) -> Self {
        Self::encode_address(DEFAULT_HRP, address)
    }

    /// Returns the zero address encoded with the given HRP.
    pub fn zero(hrp: &str) -> Self {
        Bech32Address::encode_address(hrp, Address::zero())
    }

    /// Returns the zero address encoded with the default HRP (`erd`).
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
    pub fn to_bech32_str(&self) -> &str {
        &self.bech32
    }

    pub fn to_bech32_string(&self) -> String {
        self.bech32.to_owned()
    }

    pub fn as_address(&self) -> &Address {
        &self.address
    }

    pub fn to_address(&self) -> Address {
        self.address.clone()
    }

    pub fn as_hrp(&self) -> &str {
        &self.hrp
    }

    pub fn to_hrp(&self) -> String {
        self.hrp.clone()
    }

    pub fn into_address(self) -> Address {
        self.address
    }

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

/// Error returned when parsing a [`Bech32Address`] from a string fails.
#[derive(Debug)]
pub enum Bech32AddressParseError {
    /// The bech32 string could not be decoded.
    DecodeError(bech32::DecodeError),
    /// The decoded payload was not exactly 32 bytes.
    InvalidLength(usize),
}

impl std::fmt::Display for Bech32AddressParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Bech32AddressParseError::DecodeError(e) => write!(f, "bech32 decode error: {e}"),
            Bech32AddressParseError::InvalidLength(n) => {
                write!(f, "invalid address length: expected 32 bytes, got {n}")
            }
        }
    }
}

impl std::error::Error for Bech32AddressParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Bech32AddressParseError::DecodeError(e) => Some(e),
            Bech32AddressParseError::InvalidLength(_) => None,
        }
    }
}

impl From<bech32::DecodeError> for Bech32AddressParseError {
    fn from(e: bech32::DecodeError) -> Self {
        Bech32AddressParseError::DecodeError(e)
    }
}

impl FromStr for Bech32Address {
    type Err = Bech32AddressParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hrp, dest_address_bytes) = bech32::decode(s)?;
        if dest_address_bytes.len() != 32 {
            return Err(Bech32AddressParseError::InvalidLength(
                dest_address_bytes.len(),
            ));
        }
        Ok(Bech32Address {
            address: Address::from_slice(&dest_address_bytes),
            hrp: hrp.to_string(),
            bech32: s.to_string(),
        })
    }
}
