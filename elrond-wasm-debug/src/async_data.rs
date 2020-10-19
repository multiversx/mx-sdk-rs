
use elrond_wasm::{H256, Address};
use elrond_wasm::call_data::*;
use elrond_wasm::elrond_codec::*;
use crate::display_util::*;
use crate::ext_mock::*;

use num_bigint::{BigInt, BigUint};
use num_traits::cast::ToPrimitive;

use alloc::vec::Vec;


#[derive(Debug)]
pub struct AsyncCallTxData {
    pub to: Address,
    pub call_data: Vec<u8>,
    pub call_value: BigUint,
    pub tx_hash: H256,
}

pub fn async_call_tx_input(async_data: &AsyncCallTxData, contract_addr: &Address) -> TxInput {
    let mut de = CallDataDeserializer::new(async_data.call_data.as_slice());
    let func_name = de.get_func_name().to_vec();
    let mut args: Vec<Vec<u8>> = Vec::new();
    while let Some(deserialized_arg) = de.next_argument().unwrap() {
        args.push(deserialized_arg);
    }
    TxInput{
        from: contract_addr.clone(),
        to: async_data.to.clone(),
        call_value: async_data.call_value.clone(),
        func_name,
        args,
        gas_limit: 1000,
        gas_price: 0,
        tx_hash: async_data.tx_hash.clone(),
    }
}

pub fn async_callback_tx_input(async_data: &AsyncCallTxData, contract_addr: &Address, async_result: &TxResult) -> TxInput {
    let mut args: Vec<Vec<u8>> = Vec::new();
    let serialized_bytes = top_encode_to_vec(&async_result.result_status).unwrap();
    args.push(serialized_bytes);
    if async_result.result_status == 0 {
        args.extend_from_slice(async_result.result_values.as_slice());
    } else {
        args.push(async_result.result_message.clone());
    }
    TxInput{
        from: async_data.to.clone(),
        to: contract_addr.clone(),
        call_value: 0u32.into(),
        func_name: b"callBack".to_vec(),
        args,
        gas_limit: 1000,
        gas_price: 0,
        tx_hash: async_data.tx_hash.clone(),
    }
}

pub fn merge_results(mut original: TxResult, new: TxResult) -> TxResult {
    if original.result_status == 0 {
        original.result_values.extend_from_slice(new.result_values.as_slice());
        original
    } else {
        new
    }
}
