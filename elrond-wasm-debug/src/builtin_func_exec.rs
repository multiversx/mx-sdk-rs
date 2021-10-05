use elrond_wasm::types::Address;
use num_bigint::BigUint;

use crate::*;

const ESDT_TRANSFER_FUNC: &[u8] = b"ESDTTransfer";
const SET_USERNAME_FUNC: &[u8] = b"SetUserName";

pub fn try_execute_builtin_function(
    tx_input: &TxInput,
    state: &mut BlockchainMock,
) -> Option<TxResult> {
    match tx_input.func_name.as_slice() {
        ESDT_TRANSFER_FUNC => Some(execute_esdt_transfer(tx_input, state)),
        SET_USERNAME_FUNC => Some(execute_set_username(tx_input, state)),
        _ => None,
    }
}

fn execute_esdt_transfer(tx_input: &TxInput, state: &mut BlockchainMock) -> TxResult {
    let from = tx_input.from.clone();
    let to = tx_input.to.clone();
    let esdt_token_identifier = tx_input.esdt_token_identifier.clone();
    let nonce = tx_input.nonce.clone();
    let esdt_value = tx_input.esdt_value.clone();

    state.substract_esdt_balance(&from, &esdt_token_identifier, nonce.clone(), &esdt_value);
    state.increase_esdt_balance(&to, &esdt_token_identifier, nonce, &esdt_value);
    TxResult {
        result_status: 0,
        result_message: Vec::new(),
        result_values: Vec::new(),
        result_logs: vec![esdt_transfer_event_log(
            from,
            to,
            esdt_token_identifier,
            &esdt_value,
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
        endpoint: b"ESDTTransfer".to_vec(),
        topics: vec![
            esdt_token_identifier,
            nonce_topic,
            esdt_value.to_bytes_be(),
            to.to_vec(),
        ],
        data: vec![],
    }
}

fn execute_set_username(tx_input: &TxInput, state: &mut BlockchainMock) -> TxResult {
    assert_eq!(tx_input.args.len(), 1, "SetUserName expects 1 argument");
    if state.try_set_username(&tx_input.to, tx_input.args[0].as_slice()) {
        TxResult::empty()
    } else {
        TxResult {
            result_status: 10,
            result_message: b"username already set".to_vec(),
            result_values: Vec::new(),
            result_logs: Vec::new(),
        }
    }
}
