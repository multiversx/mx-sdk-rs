pub mod abi_json;
pub mod api;
mod arwen_mandos_runner;
mod async_data;
mod blockchain_mock;
mod builtin_func_exec;
mod contract_map;
mod display_util;
mod execute_mandos;
mod mandos_step;
mod mock_error;
mod tx_context;
mod tx_input;
mod tx_log;
mod tx_managed_types;
mod tx_output;

pub use async_data::*;
pub use blockchain_mock::*;
pub use builtin_func_exec::*;
pub use contract_map::*;
pub use display_util::*;
pub use mandos_step::*;
pub use mock_error::*;
pub use tx_context::*;
pub use tx_input::*;
pub use tx_log::*;
pub use tx_managed_types::*;
pub use tx_output::*;

pub use arwen_mandos_runner::mandos_go;
pub use execute_mandos::mandos_rs;

#[macro_use]
extern crate alloc;
pub use alloc::boxed::Box;
pub use alloc::vec::Vec;

pub use std::collections::HashMap;

// #[cfg(test)]
// mod elrond_codec_tests {
//     use crate::api::RustBigUint;
//     use core::fmt::Debug;
//     use elrond_wasm::elrond_codec::test_util::{check_top_decode, check_top_encode};
//     use elrond_wasm::elrond_codec::*;

//     pub fn check_top_encode_decode<V>(element: V, expected_bytes: &[u8])
//     where
//         V: TopEncode + TopDecode + PartialEq + Debug + 'static,
//     {
//         // serialize
//         let serialized_bytes = check_top_encode(&element);
//         assert_eq!(serialized_bytes.as_slice(), expected_bytes);

//         // deserialize
//         let deserialized: V = check_top_decode::<V>(&serialized_bytes[..]);
//         assert_eq!(deserialized, element);
//     }

//     #[test]
//     fn test_big_int_serialization() {
//         check_top_encode_decode(RustBigInt::from(5), &[5u8]);
//         check_top_encode_decode(RustBigInt::from(-5), &[251u8]);
//     }

//     #[test]
//     fn test_big_uint_serialization() {
//         check_top_encode_decode(RustBigUint::from(5u32), &[5u8]);
//     }
// }
