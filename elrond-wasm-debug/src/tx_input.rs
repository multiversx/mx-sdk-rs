use crate::display_util::*;
use alloc::vec::Vec;
use elrond_wasm::types::{Address, H256};
use num_bigint::BigUint;
use std::fmt;

#[derive(Clone, Debug)]
pub struct TxInput {
    pub from: Address,
    pub to: Address,
    pub call_value: BigUint,
    pub esdt_value: BigUint,
    pub esdt_token_identifier: Vec<u8>,
    pub nonce: u64,
    pub func_name: Vec<u8>,
    pub args: Vec<Vec<u8>>,
    pub gas_limit: u64,
    pub gas_price: u64,
    pub tx_hash: H256,
}

impl fmt::Display for TxInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TxInput {{ func: {}, args: {:?}, call_value: {}, esdt_token_identifier: {:?}, esdt_nonce: {:?}, esdt_value: {:?}, from: 0x{}, to: 0x{}\n}}", 
            String::from_utf8(self.func_name.clone()).unwrap(),
            self.args,
            self.call_value,
            self.esdt_token_identifier,
            self.nonce,
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
