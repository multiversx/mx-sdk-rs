use crate::TxLog;
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

pub struct TxPanic {
    pub status: u64,
    pub message: Vec<u8>,
}


#[derive(Clone, Debug)]
pub struct TxResult {
    pub result_status: u64,
    pub result_message: Vec<u8>,
    pub result_values: Vec<Vec<u8>>,
    pub result_logs: Vec<TxLog>,
}

impl fmt::Display for TxResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let results_hex: Vec<String> = self
            .result_values
            .iter()
            .map(|r| format!("0x{}", hex::encode(r)))
            .collect();
        write!(
            f,
            "TxResult {{\n\tresult_status: {},\n\tresult_values:{:?}\n}}",
            self.result_status, results_hex
        )
    }
}

impl TxResult {
    pub fn empty() -> TxResult {
        TxResult {
            result_status: 0,
            result_message: Vec::new(),
            result_values: Vec::new(),
            result_logs: Vec::new(),
        }
    }
    pub fn print(&self) {
        println!("{}", self);
    }
}

#[derive(Debug)]
pub struct SendBalance {
    pub recipient: Address,
    pub token: TokenIdentifier,
    pub amount: BigUint,
}

#[derive(Debug)]
pub struct TxOutput {
    pub contract_storage: HashMap<Vec<u8>, Vec<u8>>,
    pub result: TxResult,
    pub send_balance_list: Vec<SendBalance>,
    pub async_call: Option<AsyncCallTxData>,
}

impl Default for TxOutput {
    fn default() -> Self {
        TxOutput {
            contract_storage: HashMap::new(),
            result: TxResult::empty(),
            send_balance_list: Vec::new(),
            async_call: None,
        }
    }
}

impl TxOutput {
    pub fn from_panic_obj(panic_obj: &TxPanic) -> Self {
        TxOutput {
            contract_storage: HashMap::new(),
            result: TxResult {
                result_status: panic_obj.status,
                result_message: panic_obj.message.clone(),
                result_values: Vec::new(),
                result_logs: Vec::new(),
            },
            send_balance_list: Vec::new(),
            async_call: None,
        }
    }

    pub fn from_panic_string(_: &str) -> Self {
        TxOutput {
            contract_storage: HashMap::new(),
            result: TxResult {
                result_status: 4,
                result_message: b"panic occurred".to_vec(),
                result_values: Vec::new(),
                result_logs: Vec::new(),
            },
            send_balance_list: Vec::new(),
            async_call: None,
        }
    }
}