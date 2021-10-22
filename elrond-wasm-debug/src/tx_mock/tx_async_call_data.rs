use crate::tx_mock::{TxInput, TxResult};
use elrond_wasm::{
    elrond_codec::*,
    types::{Address, H256},
};

use num_bigint::BigUint;

use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct AsyncCallTxData {
    pub from: Address,
    pub to: Address,
    pub call_value: BigUint,
    pub endpoint_name: Vec<u8>,
    pub arguments: Vec<Vec<u8>>,
    pub tx_hash: H256,
}

pub fn async_call_tx_input(async_data: &AsyncCallTxData) -> TxInput {
    TxInput {
        from: async_data.from.clone(),
        to: async_data.to.clone(),
        egld_value: async_data.call_value.clone(),
        esdt_values: Vec::new(),
        func_name: async_data.endpoint_name.clone(),
        args: async_data.arguments.clone(),
        gas_limit: 1000,
        gas_price: 0,
        tx_hash: async_data.tx_hash.clone(),
    }
}

pub fn async_callback_tx_input(async_data: &AsyncCallTxData, async_result: &TxResult) -> TxInput {
    let mut args: Vec<Vec<u8>> = Vec::new();
    let serialized_bytes = top_encode_to_vec_u8(&async_result.result_status).unwrap();
    args.push(serialized_bytes);
    if async_result.result_status == 0 {
        args.extend_from_slice(async_result.result_values.as_slice());
    } else {
        args.push(async_result.result_message.clone().into_bytes());
    }
    TxInput {
        from: async_data.to.clone(),
        to: async_data.from.clone(),
        egld_value: 0u32.into(),
        esdt_values: Vec::new(),
        func_name: b"callBack".to_vec(),
        args,
        gas_limit: 1000,
        gas_price: 0,
        tx_hash: async_data.tx_hash.clone(),
    }
}

pub fn merge_results(mut original: TxResult, mut new: TxResult) -> TxResult {
    if original.result_status == 0 {
        original.result_values.append(&mut new.result_values);
        original.result_logs.append(&mut new.result_logs);
        original.result_message = new.result_message;
        original
    } else {
        new
    }
}
