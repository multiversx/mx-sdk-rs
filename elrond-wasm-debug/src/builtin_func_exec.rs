use elrond_wasm::types::Address;
use num_bigint::BigUint;

use crate::*;

const ESDT_TRANSFER_FUNC: &[u8] = b"ESDTTransfer";
// const ESDT_NFT_TRANSFER_FUNC: &[u8] = b"ESDTNFTTransfer";
const SET_USERNAME_FUNC: &[u8] = b"SetUserName";

pub fn try_execute_builtin_function(
    tx_input: &TxInput,
    state: &mut BlockchainMock,
    contract_map: &ContractMap<TxContext>,
) -> Option<TxResult> {
    match tx_input.func_name.as_slice() {
        ESDT_TRANSFER_FUNC => Some(execute_esdt_transfer(tx_input, state, contract_map)),
        // ESDT_NFT_TRANSFER_FUNC => Some(execute_esdt_nft_transfer(tx_input, state)),
        SET_USERNAME_FUNC => Some(execute_set_username(tx_input, state)),
        _ => None,
    }
}

fn execute_esdt_transfer(
    tx_input: &TxInput,
    state: &mut BlockchainMock,
    contract_map: &ContractMap<TxContext>,
) -> TxResult {
    let from = tx_input.from.clone();
    let to = tx_input.to.clone();
    let esdt_token_identifier = tx_input.esdt_token_identifier.clone();
    let nonce = tx_input.nonce;
    let esdt_value = tx_input.esdt_value.clone();

    state.substract_esdt_balance(&from, &esdt_token_identifier, nonce, &esdt_value);
    state.increase_esdt_balance(&to, &esdt_token_identifier, nonce, &esdt_value);
    let mut result = TxResult {
        result_status: 0,
        result_message: Vec::new(),
        result_values: Vec::new(),
        result_logs: Vec::new(),
    };

    if esdt_transfer_has_additional_execution(tx_input) {
        let tx_output = esdt_transfer_execute_additional(tx_input, state, contract_map);
        result = tx_output.result;
    }

    result
}

fn esdt_transfer_execute_additional(
    tx_input: &TxInput,
    state: &mut BlockchainMock,
    contract_map: &ContractMap<TxContext>,
) -> TxOutput {
    let mut new_tx_input = tx_input.clone();
    new_tx_input.func_name = tx_input.args[2].clone();
    new_tx_input.args = Vec::<Vec<u8>>::new();
    let dest = new_tx_input.to.clone();

    if tx_input.args.len() > 3 {
        for arg in tx_input.args[3..].iter() {
            new_tx_input.args.push(arg.clone());
        }
    };

    let blockchain_info = state.create_tx_info(&tx_input.to);
    let contract_account = state
        .accounts
        .get_mut(&tx_input.to)
        .unwrap_or_else(|| panic!("Recipient account not found: {}", address_hex(&tx_input.to)));

    let contract_path = &contract_account
        .contract_path
        .clone()
        .unwrap_or_else(|| panic!("Recipient account is not a smart contract"));

    let tx_context = TxContext::new(
        blockchain_info,
        new_tx_input,
        TxOutput {
            contract_storage: contract_account.storage.clone(),
            managed_types: TxManagedTypes::new(),
            result: TxResult::empty(),
            send_balance_list: Vec::new(),
            async_call: None,
        },
    );

    let mut tx_output = execute_tx(tx_context, contract_path, contract_map);
    if tx_output.result.result_status != 0 {
        std::panic::panic_any(TxPanic {
            status: 10,
            message: b"builtin exec failed".to_vec(),
        });
    }

    let _ = std::mem::replace(
        &mut contract_account.storage,
        tx_output.contract_storage.clone(),
    );

    let send_result = state.send_balance(
        &dest,
        tx_output.send_balance_list.as_slice(),
        &mut tx_output.result.result_logs,
    );
    if send_result.is_err() {
        std::panic::panic_any(TxPanic {
            status: 10,
            message: b"builtin send failed".to_vec(),
        });
    }

    // let exec_result = state.execute();

    tx_output
}

fn esdt_transfer_has_additional_execution(tx_input: &TxInput) -> bool {
    tx_input.args.len() > 2
}

// fn esdt_nft_transfer_has_additional_execution(tx_input: &TxInput) -> bool {
//     tx_input.args.len() > 4
// }

// fn execute_esdt_nft_transfer(tx_input: &TxInput, state: &mut BlockchainMock) -> TxResult {}

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
