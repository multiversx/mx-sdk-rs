use crate::{
    num_bigint::BigUint,
    tx_execution::default_execution,
    tx_mock::{BlockchainUpdate, TxCache, TxInput, TxLog, TxResult},
};
use elrond_wasm::{api::ESDT_TRANSFER_FUNC_NAME, types::heap::Address};

pub fn execute_esdt_transfer(tx_input: TxInput, tx_cache: TxCache) -> (TxResult, BlockchainUpdate) {
    if tx_input.args.len() < 2 {
        let err_result = TxResult::from_vm_error("ESDTTransfer too few arguments".to_string());
        return (err_result, BlockchainUpdate::empty());
    }

    let exec_input = tx_input.convert_to_token_transfer();
    let esdt_transfer_log = esdt_transfer_event_log(
        exec_input.from.clone(),
        exec_input.to.clone(),
        exec_input.esdt_values[0].token_identifier.clone(),
        &exec_input.esdt_values[0].value,
    );

    let (mut tx_result, blockchain_updates) = default_execution(exec_input, tx_cache);

    // prepends esdt log
    tx_result.result_logs = [&[esdt_transfer_log][..], tx_result.result_logs.as_slice()].concat();

    (tx_result, blockchain_updates)
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
        endpoint: ESDT_TRANSFER_FUNC_NAME.to_vec(),
        topics: vec![
            esdt_token_identifier,
            nonce_topic,
            esdt_value.to_bytes_be(),
            to.to_vec(),
        ],
        data: vec![],
    }
}
