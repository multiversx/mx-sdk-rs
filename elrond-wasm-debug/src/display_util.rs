use elrond_wasm::{Address, H256};

use crate::big_int_mock::*;
use crate::big_uint_mock::*;
use crate::contract_map::*;

use elrond_wasm::api::BigUintApi;
use elrond_wasm::api::CallableContract;
use elrond_wasm::api::ContractHookApi;
use elrond_wasm::err_msg;

use num_bigint::{BigInt, BigUint};
use num_traits::cast::ToPrimitive;

use alloc::boxed::Box;
use alloc::vec::Vec;

use std::collections::HashMap;
use std::fmt;
use std::fmt::Write;

use sha3::{Digest, Keccak256, Sha3_256};

pub fn address_hex(address: &Address) -> alloc::string::String {
	alloc::format!("0x{}", hex::encode(address.as_bytes()))
}

pub fn key_hex(key: &[u8]) -> alloc::string::String {
	alloc::format!("0x{}", hex::encode(key))
}

pub fn verbose_hex(value: &[u8]) -> alloc::string::String {
	alloc::format!("0x{}", hex::encode(value))
}
