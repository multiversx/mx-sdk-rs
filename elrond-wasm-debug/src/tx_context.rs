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

#[derive(Debug)]
pub struct TxContext {
    pub blockchain_info_box: Box<BlockchainTxInfo>,
    pub tx_input_box: Box<TxInput>,
    pub tx_output_cell: Rc<RefCell<TxOutput>>,
}

impl TxContext {
    pub fn new(blockchain_info: BlockchainTxInfo, tx_input: TxInput, tx_output: TxOutput) -> Self {
        TxContext {
            blockchain_info_box: Box::new(blockchain_info),
            tx_input_box: Box::new(tx_input),
            tx_output_cell: Rc::new(RefCell::new(tx_output)),
        }
    }

    pub fn into_output(self) -> TxOutput {
        let ref_cell = Rc::try_unwrap(self.tx_output_cell).unwrap();
        ref_cell.replace(TxOutput::default())
    }

    pub fn dummy() -> Self {
        TxContext {
            blockchain_info_box: Box::new(BlockchainTxInfo {
                previous_block_info: BlockInfo::new(),
                current_block_info: BlockInfo::new(),
                contract_balance: 0u32.into(),
                contract_esdt: HashMap::new(),
                contract_owner: None,
            }),
            tx_input_box: Box::new(TxInput {
                from: Address::zero(),
                to: Address::zero(),
                call_value: 0u32.into(),
                esdt_value: 0u32.into(),
                esdt_token_identifier: Vec::new(),
                func_name: Vec::new(),
                args: Vec::new(),
                gas_limit: 0,
                gas_price: 0,
                tx_hash: b"dummy...........................".into(),
            }),
            tx_output_cell: Rc::new(RefCell::new(TxOutput::default())),
        }
    }
}

impl Clone for TxContext {
    fn clone(&self) -> Self {
        TxContext {
            blockchain_info_box: self.blockchain_info_box.clone(),
            tx_input_box: self.tx_input_box.clone(),
            tx_output_cell: Rc::clone(&self.tx_output_cell),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TxLog {
    pub address: Address,
    pub endpoint: Vec<u8>,
    pub topics: Vec<Vec<u8>>,
    pub data: Vec<u8>,
}

impl TxLog {
    pub fn equals(&self, check_log: &mandos::CheckLog) -> bool {
        if self.address.to_vec() == check_log.address.value
            && self.endpoint == check_log.endpoint.value
            && self.data == check_log.data.value
        {
            for (topic, other_topic) in self.topics.iter().zip(check_log.topics.iter()) {
                if topic != &other_topic.value {
                    return false;
                }
            }

            true
        } else {
            false
        }
    }
}
