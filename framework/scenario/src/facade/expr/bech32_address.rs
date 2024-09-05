use std::fmt::Display;

use crate::bech32;
use multiversx_sc::{
    abi::TypeAbiFrom,
    api::ManagedTypeApi,
    codec::*,
    types::{
        Address, AnnotatedValue, ManagedAddress, ManagedBuffer, TxEnv, TxFrom, TxFromSpecified,
        TxTo, TxToSpecified,
    },
};
use serde::{Deserialize, Serialize};

const BECH32_PREFIX: &str = "bech32:";

/// Wraps and address, and presents it as a bech32 expression wherever possible.
///
/// In order to avoid repeated conversions, it redundantly keeps the bech32 representation inside.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bech32Address {
    address: Address,
    bech32: String,
}

impl From<Address> for Bech32Address {
    fn from(value: Address) -> Self {
        let bech32 = bech32::encode(&value);
        Bech32Address {
            address: value,
            bech32,
        }
    }
}

impl From<&Address> for Bech32Address {
    fn from(value: &Address) -> Self {
        let bech32 = bech32::encode(value);
        Bech32Address {
            address: value.clone(),
            bech32,
        }
    }
}

impl Bech32Address {
    pub fn from_bech32_string(bech32: String) -> Self {
        let address = bech32::decode(&bech32);
        Bech32Address { address, bech32 }
    }

    pub fn to_bech32_str(&self) -> &str {
        &self.bech32
    }

    pub fn to_bech32_string(&self) -> String {
        self.bech32.to_owned()
    }

    pub fn to_hex(&self) -> String {
        hex::encode(&self.address)
    }

    pub fn as_address(&self) -> &Address {
        &self.address
    }

    pub fn to_address(&self) -> Address {
        self.address.clone()
    }

    pub fn into_address(self) -> Address {
        self.address
    }

    pub fn to_bech32_expr(&self) -> String {
        format!("{BECH32_PREFIX}{}", &self.bech32)
    }
}

impl<Env> AnnotatedValue<Env, ManagedAddress<Env::Api>> for Bech32Address
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        self.to_bech32_expr().into()
    }

    fn to_value(&self, env: &Env) -> ManagedAddress<Env::Api> {
        self.address.to_value(env)
    }
}

impl<Env> TxFrom<Env> for Bech32Address
where
    Env: TxEnv,
{
    fn resolve_address(&self, env: &Env) -> ManagedAddress<Env::Api> {
        self.address.resolve_address(env)
    }
}
impl<Env> TxFromSpecified<Env> for Bech32Address where Env: TxEnv {}
impl<Env> TxTo<Env> for Bech32Address where Env: TxEnv {}
impl<Env> TxToSpecified<Env> for Bech32Address where Env: TxEnv {}

impl<Env> AnnotatedValue<Env, ManagedAddress<Env::Api>> for &Bech32Address
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        self.to_bech32_expr().into()
    }

    fn to_value(&self, env: &Env) -> ManagedAddress<Env::Api> {
        self.address.to_value(env)
    }
}

impl<Env> TxFrom<Env> for &Bech32Address
where
    Env: TxEnv,
{
    fn resolve_address(&self, env: &Env) -> ManagedAddress<Env::Api> {
        self.address.resolve_address(env)
    }
}
impl<Env> TxFromSpecified<Env> for &Bech32Address where Env: TxEnv {}
impl<Env> TxTo<Env> for &Bech32Address where Env: TxEnv {}
impl<Env> TxToSpecified<Env> for &Bech32Address where Env: TxEnv {}

impl Display for Bech32Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.bech32)
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

impl<M> TypeAbiFrom<Bech32Address> for ManagedAddress<M> where M: ManagedTypeApi {}
impl<M> TypeAbiFrom<&Bech32Address> for ManagedAddress<M> where M: ManagedTypeApi {}

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
