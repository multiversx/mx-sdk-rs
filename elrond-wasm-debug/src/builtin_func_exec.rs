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

    state.subtract_esdt_balance(&tx_input.from, &token_identifier, 0, &value);
    state.increase_esdt_balance(&tx_input.to, &token_identifier, 0, &value);
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
