use alloc::vec::Vec;
use elrond_wasm::types::{Address, BoxedBytes};
use num_bigint::BigUint;
use std::{collections::HashMap, fmt};

use crate::AsyncCallTxData;

use super::{TxLog, TxPanic};

#[derive(Clone, Default, Debug)]
pub struct TxResult {
    pub result_status: u64,
    pub result_message: String,
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
            result_message: String::new(),
            result_values: Vec::new(),
            result_logs: Vec::new(),
        }
    }
    pub fn print(&self) {
        println!("{}", self);
    }

    pub fn from_panic_obj(panic_obj: &TxPanic) -> Self {
        TxResult {
            result_status: panic_obj.status,
            result_message: String::from_utf8(panic_obj.message.clone()).unwrap(),
            result_values: Vec::new(),
            result_logs: Vec::new(),
        }
    }

    pub fn from_panic_string(s: &str) -> Self {
        TxResult {
            result_status: 4,
            result_message: "panic occurred".to_string(),
            // result_message: s.to_string(),
            result_values: Vec::new(),
            result_logs: Vec::new(),
        }
    }

    pub fn from_unknown_panic() -> Self {
        Self::from_panic_string("")
    }
}
