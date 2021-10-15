use alloc::vec::Vec;
use elrond_wasm::types::{Address, BoxedBytes};
use num_bigint::BigUint;
use std::{collections::HashMap, fmt};

use crate::AsyncCallTxData;

use super::{TxLog, TxPanic, TxResult};

#[derive(Debug)]
pub struct SendBalance {
    pub recipient: Address,
    pub token_identifier: BoxedBytes,
    pub nonce: u64,
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
                result_message: String::from_utf8(panic_obj.message.clone()).unwrap(),
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
                result_message: "panic occurred".to_string(),
                result_values: Vec::new(),
                result_logs: Vec::new(),
            },
            send_balance_list: Vec::new(),
            async_call: None,
        }
    }
}
