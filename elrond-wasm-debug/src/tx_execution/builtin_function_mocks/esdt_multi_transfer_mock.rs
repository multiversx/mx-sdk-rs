use crate::{
    tx_execution::default_execution,
    tx_mock::{BlockchainUpdate, TxCache, TxInput, TxLog, TxResult},
};
use elrond_wasm::{
    api::ESDT_MULTI_TRANSFER_FUNC_NAME,
    elrond_codec::{TopDecode, TopEncode},
};

pub fn execute_esdt_multi_transfer(
    tx_input: TxInput,
    tx_cache: TxCache,
) -> (TxResult, BlockchainUpdate) {
    if tx_input.args.len() < 2 {
        let err_result =
            TxResult::from_vm_error("MultiESDTNFTTransfer too few arguments".to_string());
        return (err_result, BlockchainUpdate::empty());
    }
    assert!(
        tx_input.to == tx_input.from,
        "MultiESDTNFTTransfer expects that to == from"
    );

    let nr_payments = usize::top_decode(tx_input.args[1].as_slice()).unwrap();
    if tx_input.args.len() < 2 + nr_payments * 3 {
        let err_result =
            TxResult::from_vm_error("MultiESDTNFTTransfer too few arguments".to_string());
        return (err_result, BlockchainUpdate::empty());
    }

    let exec_input = tx_input.convert_to_token_transfer();
    let sender_addr = exec_input.from.clone();
    let dest_addr_bytes = exec_input.to.to_vec();

    let mut builtin_logs = Vec::new();
    for esdt_transfer in &exec_input.esdt_values {
        let mut nonce_bytes = Vec::new();
        esdt_transfer.nonce.top_encode(&mut nonce_bytes).unwrap();

        let mut value_bytes = Vec::new();
        esdt_transfer.value.top_encode(&mut value_bytes).unwrap();

        builtin_logs.push(TxLog {
            address: sender_addr.clone(),
            endpoint: ESDT_MULTI_TRANSFER_FUNC_NAME.to_vec(),
            topics: vec![
                esdt_transfer.token_identifier.clone(),
                nonce_bytes,
                value_bytes,
                dest_addr_bytes.clone(),
            ],
            data: vec![],
        });
    }

    let (mut tx_result, blockchain_updates) = default_execution(exec_input, tx_cache);

    // prepends esdt log
    tx_result.result_logs = [builtin_logs.as_slice(), tx_result.result_logs.as_slice()].concat();

    (tx_result, blockchain_updates)
}
