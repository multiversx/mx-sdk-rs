// use core::slice;
// use std::{fmt::Display, ops::Add};

// use crate::bech32;
// use multiversx_sc::{
//     abi::TypeAbiFrom,
//     api::ManagedTypeApi,
//     codec::*,
//     types::{
//         Address, AnnotatedValue, ManagedAddress, ManagedBuffer, TxEnv, TxFrom, TxFromSpecified,
//         TxTo, TxToSpecified,
//     },
// };
// use serde::{Deserialize, Serialize};

// const BECH32_PREFIX: &str = "bech32:";

// #[derive(Clone, Debug, PartialEq, Eq)]
// pub struct ExternalAddress {
//     pub hrp: String,
//     pub address: Address,
// }

// /// Wraps and address, and presents it as a bech32 expression wherever possible.
// ///
// /// In order to avoid repeated conversions, it redundantly keeps the bech32 representation inside.
// #[derive(Clone, Debug, PartialEq, Eq)]
// pub struct ExternalBech32Address {
//     external_address: ExternalAddress,
//     bech32: String,
// }

// impl From<ExternalAddress> for ExternalBech32Address {
//     fn from(external_address: ExternalAddress) -> Self {
//         let bech32 = bech32::external_encode(&external_address.hrp, &external_address.address);
//         ExternalBech32Address {
//             external_address: external_address,
//             bech32,
//         }
//     }
// }

// impl From<&ExternalAddress> for ExternalBech32Address {
//     fn from(value: &ExternalAddress) -> Self {
//         let bech32 = bech32::external_encode(&value.hrp, &value.address);
//         ExternalBech32Address {
//             external_address: value.clone(),
//             bech32,
//         }
//     }
// }

// impl ExternalBech32Address {
//     pub fn from_bech32_string(bech32: String) -> Self {
//         let (hrp, address) = bech32::decode(&bech32);
//         let external_address = ExternalAddress {
//             hrp: hrp.to_string(),
//             address,
//         };
//         ExternalBech32Address {
//             external_address,
//             bech32,
//         }
//     }

//     pub fn to_bech32_str(&self) -> &str {
//         &self.bech32
//     }

//     pub fn to_bech32_string(&self) -> String {
//         self.bech32.to_owned()
//     }

//     pub fn to_hex(&self) -> String {
//         hex::encode(&self.external_address.address)
//     }

//     pub fn as_address(&self) -> &Address {
//         &self.external_address.address
//     }

//     pub fn to_address(&self) -> Address {
//         self.external_address.address.clone()
//     }

//     pub fn as_hrp(&self) -> &str {
//         &self.external_address.hrp
//     }

//     pub fn to_hrp(&self) -> String {
//         self.external_address.hrp.clone()
//     }

//     pub fn into_address(self) -> Address {
//         self.external_address.address
//     }

//     pub fn to_bech32_expr(&self) -> String {
//         format!("{BECH32_PREFIX}{}", &self.bech32)
//     }
// }

// impl<Env> AnnotatedValue<Env, ManagedAddress<Env::Api>> for ExternalBech32Address
// where
//     Env: TxEnv,
// {
//     fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
//         self.to_bech32_expr().into()
//     }

//     fn to_value(&self, env: &Env) -> ManagedAddress<Env::Api> {
//         self.external_address.address.to_value(env)
//     }
// }

// impl<Env> TxFrom<Env> for ExternalBech32Address
// where
//     Env: TxEnv,
// {
//     fn resolve_address(&self, env: &Env) -> ManagedAddress<Env::Api> {
//         self.external_address.address.resolve_address(env)
//     }
// }
// impl<Env> TxFromSpecified<Env> for ExternalBech32Address where Env: TxEnv {}
// impl<Env> TxTo<Env> for ExternalBech32Address where Env: TxEnv {}
// impl<Env> TxToSpecified<Env> for ExternalBech32Address where Env: TxEnv {}

// impl<Env> AnnotatedValue<Env, ManagedAddress<Env::Api>> for &ExternalBech32Address
// where
//     Env: TxEnv,
// {
//     fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
//         self.to_bech32_expr().into()
//     }

//     fn to_value(&self, env: &Env) -> ManagedAddress<Env::Api> {
//         self.external_address.address.to_value(env)
//     }
// }

// impl<Env> TxFrom<Env> for &ExternalBech32Address
// where
//     Env: TxEnv,
// {
//     fn resolve_address(&self, env: &Env) -> ManagedAddress<Env::Api> {
//         self.external_address.address.resolve_address(env)
//     }
// }
// impl<Env> TxFromSpecified<Env> for &ExternalBech32Address where Env: TxEnv {}
// impl<Env> TxTo<Env> for &ExternalBech32Address where Env: TxEnv {}
// impl<Env> TxToSpecified<Env> for &ExternalBech32Address where Env: TxEnv {}

// impl Display for ExternalBech32Address {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_str(&self.bech32)
//     }
// }

// impl NestedEncode for ExternalBech32Address {
//     fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
//     where
//         O: NestedEncodeOutput,
//         H: EncodeErrorHandler,
//     {
//         self.external_address
//             .address
//             .dep_encode_or_handle_err(dest, h)
//     }
// }

// impl TopEncode for ExternalBech32Address {
//     fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
//     where
//         O: TopEncodeOutput,
//         H: EncodeErrorHandler,
//     {
//         self.external_address
//             .address
//             .top_encode_or_handle_err(output, h)
//     }
// }

// impl NestedDecode for ExternalBech32Address {
//     fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
//     where
//         I: NestedDecodeInput,
//         H: DecodeErrorHandler,
//     {
//         Ok(ExternalBech32Address::from(ExternalAddress {
//             hrp: String::dep_decode_or_handle_err(input, h)?,
//             address: Address::dep_decode_or_handle_err(input, h)?,
//         }))
//     }
// }

// // impl TopDecode for ExternalBech32Address {
// //     fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
// //     where
// //         I: TopDecodeInput,
// //         H: DecodeErrorHandler,
// //     {
// //         let mut nested_buffer = input.into_nested_buffer();
// //         let hrp = String::top_decode_or_handle_err(nested_buffer, h)?;
// //         let address = Address::top_decode_or_handle_err(input, h)?;

// //         Ok(ExternalBech32Address::from(ExternalAddress {
// //             hrp,
// //             address,
// //         }))
// //     }
// // }

// impl<M> TypeAbiFrom<ExternalBech32Address> for ManagedAddress<M> where M: ManagedTypeApi {}
// impl<M> TypeAbiFrom<&ExternalBech32Address> for ManagedAddress<M> where M: ManagedTypeApi {}

// impl Serialize for ExternalBech32Address {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         self.bech32.serialize(serializer)
//     }
// }

// impl<'de> Deserialize<'de> for ExternalBech32Address {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         // some old interactors have it serialized like this
//         let mut bech32 = String::deserialize(deserializer)?;
//         if let Some(stripped) = bech32.strip_prefix("bech32:") {
//             bech32 = stripped.to_string();
//         }
//         Ok(ExternalBech32Address::from_bech32_string(bech32))
//     }
// }
