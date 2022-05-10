use crate::{
    tx_execution::default_execution,
    tx_mock::{BlockchainUpdate, TxCache, TxInput, TxLog, TxResult},
};
use elrond_wasm::api::ESDT_NFT_TRANSFER_FUNC_NAME;

pub fn execute_esdt_nft_transfer(
    tx_input: TxInput,
    tx_cache: TxCache,
) -> (TxResult, BlockchainUpdate) {
    if tx_input.args.len() < 4 {
        let err_result = TxResult::from_vm_error("ESDTNFTTransfer too few arguments".to_string());
        return (err_result, BlockchainUpdate::empty());
    }
    assert!(
        tx_input.to == tx_input.from,
        "ESDTNFTTransfer expects that to == from"
    );

    let esdt_nft_transfer_log = TxLog {
        address: tx_input.from.clone(),
        endpoint: ESDT_NFT_TRANSFER_FUNC_NAME.to_vec(),
        topics: vec![
            tx_input.args[0].clone(),
            tx_input.args[1].clone(),
            tx_input.args[2].clone(),
            tx_input.args[3].clone(),
        ],
        data: vec![],
    };
    let exec_input = tx_input.convert_to_token_transfer();

    let (mut tx_result, blockchain_updates) = default_execution(exec_input, tx_cache);

    // prepends esdt log
    tx_result.result_logs = [
        &[esdt_nft_transfer_log][..],
        tx_result.result_logs.as_slice(),
    ]
    .concat();

    (tx_result, blockchain_updates)
}
