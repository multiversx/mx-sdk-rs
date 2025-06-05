use std::fmt::Display;

use crate::{codec::*, types::Address};
use bech32::{Bech32, Hrp};
use serde::{Deserialize, Serialize};

const BECH32_PREFIX: &str = "bech32:";

/// Wraps and address, and presents it as a bech32 expression wherever possible.
///
/// In order to avoid repeated conversions, it redundantly keeps the bech32 representation inside.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bech32Address {
    pub address: Address,
    pub hrp: String,
    pub bech32: String,
}

fn decode(bech32_address: &str) -> (String, Address) {
    let (hrp, dest_address_bytes) = bech32::decode(bech32_address)
        .unwrap_or_else(|err| panic!("bech32 decode error for {bech32_address}: {err}"));
    if dest_address_bytes.len() != 32 {
        panic!("Invalid address length after decoding")
    }

    (hrp.to_string(), Address::from_slice(&dest_address_bytes))
}

fn encode(hrp: &str, address: &Address) -> String {
    let hrp = Hrp::parse(hrp).expect("invalid hrp");
    bech32::encode::<Bech32>(hrp, address.as_bytes()).expect("bech32 encode error")
}

impl Bech32Address {}

impl From<Address> for Bech32Address {
    fn from(value: Address) -> Self {
        let bech32 = encode("erd", &value);
        Bech32Address {
            hrp: "erd".to_string(),
            address: value,
            bech32,
        }
    }
}

impl From<(&str, Address)> for Bech32Address {
    fn from(value: (&str, Address)) -> Self {
        let bech32 = encode(value.0, &value.1);
        Bech32Address {
            hrp: value.0.to_string(),
            address: value.1,
            bech32,
        }
    }
}

impl From<(&str, &Address)> for Bech32Address {
    fn from(value: (&str, &Address)) -> Self {
        let bech32 = encode(value.0, value.1);
        Bech32Address {
            hrp: value.0.to_string(),
            address: value.1.clone(),
            bech32,
        }
    }
}

impl From<&Address> for Bech32Address {
    fn from(value: &Address) -> Self {
        let bech32 = encode("erd", value);
        Bech32Address {
            hrp: "erd".to_string(),
            address: value.clone(),
            bech32,
        }
    }
}

impl Bech32Address {
    pub fn from_bech32_string(bech32: String) -> Self {
        let (hrp, address) = decode(&bech32);
        Bech32Address {
            hrp,
            address,
            bech32,
        }
    }

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

impl Display for Bech32Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.bech32)
    }
}
