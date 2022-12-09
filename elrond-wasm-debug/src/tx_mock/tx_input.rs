use crate::{display_util::*, num_bigint::BigUint};
use alloc::vec::Vec;
use elrond_wasm::types::heap::{Address, H256};
use num_traits::Zero;
use std::fmt;

use super::TxFunctionName;

#[derive(Clone, Debug)]
pub struct TxInput {
    pub from: Address,
    pub to: Address,
    pub egld_value: BigUint,
    pub esdt_values: Vec<TxInputESDT>,
    pub func_name: TxFunctionName,
    pub args: Vec<Vec<u8>>,
    pub gas_limit: u64,
    pub gas_price: u64,
    pub tx_hash: H256,
    pub promise_callback_closure_data: Vec<u8>,
}

impl fmt::Display for TxInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TxInput {{ func: {}, args: {:?}, call_value: {}, esdt_value: {:?}, from: 0x{}, to: 0x{}\n}}", 
            self.func_name.as_str(),
            self.args,
            self.egld_value,
            self.esdt_values,
            address_hex(&self.from),
            address_hex(&self.to))
    }
}

impl TxInput {
    pub fn add_arg(&mut self, arg: Vec<u8>) {
        self.args.push(arg);
    }

    pub fn dummy() -> Self {
        TxInput {
            from: Address::zero(),
            to: Address::zero(),
            egld_value: BigUint::zero(),
            esdt_values: Vec::new(),
            func_name: TxFunctionName::empty(),
            args: Vec::new(),
            gas_limit: 0,
            gas_price: 0,
            tx_hash: H256::zero(),
            promise_callback_closure_data: Vec::new(),
        }
    }

    pub fn func_name_from_arg_index(&self, arg_index: usize) -> TxFunctionName {
        if let Some(arg) = self.args.get(arg_index) {
            arg.into()
        } else {
            TxFunctionName::empty()
        }
    }
}

#[derive(Clone, Debug)]
pub struct TxInputESDT {
    pub token_identifier: Vec<u8>,
    pub nonce: u64,
    pub value: BigUint,
}
