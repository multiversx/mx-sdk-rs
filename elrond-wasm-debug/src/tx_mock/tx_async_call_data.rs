use crate::tx_mock::{TxInput, TxResult};
use elrond_wasm::{
    elrond_codec::*,
    types::heap::{Address, H256},
};

use crate::num_bigint::BigUint;

use alloc::vec::Vec;

use super::Promise;

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

    // for the cases when the callee SC also makes an async call towards the caller SC
    match &async_result.result_calls.async_call {
        Some(result_async_call) => {
            let result_async_input = TxInput {
                from: result_async_call.from.clone(),
                to: result_async_call.to.clone(),
                egld_value: result_async_call.call_value.clone(),
                esdt_values: Vec::new(),
                func_name: result_async_call.endpoint_name.clone(),
                args: result_async_call.arguments.clone(),
                gas_limit: 1000,
                gas_price: 0,
                tx_hash: result_async_call.tx_hash.clone(),
            };
            let mut cb_input = result_async_input.convert_to_token_transfer();
            cb_input.func_name = b"callBack".to_vec();
            cb_input.args = args;

            cb_input
        },
        None => TxInput {
            from: async_data.to.clone(),
            to: async_data.from.clone(),
            egld_value: async_result
                .result_calls
                .transfer_execute_calls
                .egld_value
                .clone(),
            esdt_values: Vec::new(),
            func_name: b"callBack".to_vec(),
            args,
            gas_limit: 1000,
            gas_price: 0,
            tx_hash: async_data.tx_hash.clone(),
        },
    }
}

pub fn async_promise_tx_input(
    address: &Address,
    promise: &Promise,
    async_result: &TxResult,
) -> TxInput {
    let mut args: Vec<Vec<u8>> = Vec::new();
    let serialized_bytes = top_encode_to_vec_u8(&async_result.result_status).unwrap();
    args.push(serialized_bytes);
    let callback: Vec<u8> = if async_result.result_status == 0 {
        args.extend_from_slice(async_result.result_values.as_slice());
        promise.success_callback.to_vec()
    } else {
        args.push(async_result.result_message.clone().into_bytes());
        promise.error_callback.to_vec()
    };

    TxInput {
        from: promise.endpoint.from.clone(),
        to: address.clone(),
        egld_value: 0u32.into(),
        esdt_values: Vec::new(),
        func_name: callback,
        args,
        gas_limit: 1000,
        gas_price: 0,
        tx_hash: promise.endpoint.tx_hash.clone(),
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
