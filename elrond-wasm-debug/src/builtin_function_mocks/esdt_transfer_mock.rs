use std::{cell::RefCell, rc::Rc};

use elrond_wasm::types::Address;
use num_bigint::BigUint;

use crate::{
    tx_mock::{TxCache, TxContext, TxInput, TxLog, TxResult},
    world_mock::BlockchainMock,
};

use super::builtin_func_exec::ESDT_TRANSFER_FUNC;

pub fn execute_esdt_transfer(tx_input: &TxInput, state: &mut BlockchainMock) -> TxResult {
    if tx_input.args.len() != 2 {
        return TxResult {
            result_status: 10,
            result_message: b"ESDTTransfer too few arguments".to_vec(),
            result_values: Vec::new(),
            result_logs: Vec::new(),
        };
    }

    let token_identifier = tx_input.args[0].clone();
    let value = BigUint::from_bytes_be(tx_input.args[1].as_slice());

    let blockchain_cell = Rc::new(RefCell::new(*state));
    let tx_cache = TxCache::new(blockchain_cell);

    tx_cache.subtract_esdt_balance(&tx_input.from, &token_identifier, 0, &value);
    tx_cache.increase_esdt_balance(&tx_input.to, &token_identifier, 0, &value);
    tx_cache.commit();
    TxResult {
        result_status: 0,
        result_message: Vec::new(),
        result_values: Vec::new(),
        result_logs: vec![esdt_transfer_event_log(
            tx_input.from.clone(),
            tx_input.to.clone(),
            token_identifier,
            &value,
        )],
    }
}

pub fn esdt_transfer_event_log(
    from: Address,
    to: Address,
    esdt_token_identifier: Vec<u8>,
    esdt_value: &BigUint,
) -> TxLog {
    let nonce_topic = Vec::<u8>::new();
    TxLog {
        address: from,
        endpoint: ESDT_TRANSFER_FUNC.to_vec(),
        topics: vec![
            esdt_token_identifier,
            nonce_topic,
            esdt_value.to_bytes_be(),
            to.to_vec(),
        ],
        data: vec![],
    }
}
