use crate::async_data::*;
use crate::blockchain_mock::*;
use crate::display_util::*;
use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::RefCell;
use elrond_wasm::types::{Address, TokenIdentifier, H256};
use num_bigint::BigUint;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug)]
pub struct TxInput {
    pub from: Address,
    pub to: Address,
    pub call_value: BigUint,
    pub esdt_value: BigUint,
    pub esdt_token_identifier: Vec<u8>,
    pub func_name: Vec<u8>,
    pub args: Vec<Vec<u8>>,
    pub gas_limit: u64,
    pub gas_price: u64,
    pub tx_hash: H256,
}

impl fmt::Display for TxInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TxInput {{ func: {}, args: {:?}, call_value: {}, esdt_token_identifier: {:?}, esdt_value: {:?}, from: 0x{}, to: 0x{}\n}}", 
            String::from_utf8(self.func_name.clone()).unwrap(),
            self.args,
            self.call_value,
            self.esdt_token_identifier,
            self.esdt_value,
            address_hex(&self.from),
            address_hex(&self.to))
    }
}

impl TxInput {
    pub fn add_arg(&mut self, arg: Vec<u8>) {
        self.args.push(arg);
    }
}