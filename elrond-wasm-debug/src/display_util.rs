

use elrond_wasm::{H256, Address};

use crate::big_int_mock::*;
use crate::big_uint_mock::*;
use crate::contract_map::*;

use elrond_wasm::ContractHookApi;
use elrond_wasm::CallableContract;
use elrond_wasm::BigUintApi;
use elrond_wasm::err_msg;

use num_bigint::{BigInt, BigUint};
use num_traits::cast::ToPrimitive;

use alloc::boxed::Box;
use alloc::vec::Vec;

use std::collections::HashMap;
use std::fmt;
use std::fmt::Write;

use sha3::{Sha3_256, Keccak256, Digest};


pub fn address_hex(address: &H256) -> alloc::string::String {
    alloc::format!("0x{}", hex::encode(address.as_bytes()))
}

pub fn key_hex(key: &[u8]) -> alloc::string::String {
    alloc::format!("0x{}", hex::encode(key))
}

pub fn verbose_hex(value: &[u8]) -> alloc::string::String {
    alloc::format!("0x{}", hex::encode(value))
}

// impl fmt::Display for TxData {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "TxData {{ func: {}, args: {:?}, call_value: {}, from: 0x{}, to: 0x{}\n}}", 
//             String::from_utf8(self.func_name.clone()).unwrap(), 
//             self.args, 
//             self.call_value,
//             address_hex(&self.from), 
//             address_hex(&self.to))
//     }
// }

// impl fmt::Display for TxResult {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let results_hex: Vec<String> = self.result_values.iter().map(|r| format!("0x{}", hex::encode(r))).collect();
//         write!(f, "TxResult {{\n\tresult_status: {},\n\tresult_values:{:?}\n}}", self.result_status, results_hex)
//     }
// }