use elrond_wasm::{
    api::ESDT_NFT_ADD_URI_FUNC_NAME,
    elrond_codec::{top_encode_to_vec_u8, TopDecode},
};

use crate::tx_mock::{BlockchainUpdate, TxCache, TxInput, TxLog, TxResult, TxResultCalls};

pub fn execute_nft_add_uri(tx_input: TxInput, tx_cache: TxCache) -> (TxResult, BlockchainUpdate) {
    if tx_input.args.len() < 3 {
        let err_result =
            TxResult::from_vm_error("ESDTNFTAddURI expects at least 3 arguments".to_string());
        return (err_result, BlockchainUpdate::empty());
    }

    let token_identifier = tx_input.args[0].clone();
    let nonce = u64::top_decode(tx_input.args[1].as_slice()).unwrap();
    let mut new_uris = tx_input.args[2..].to_vec();

    tx_cache.with_account_mut(&tx_input.from, |account| {
        account
            .esdt
            .add_uris(token_identifier.as_slice(), nonce, new_uris.clone());
    });

    let mut topics = vec![
        token_identifier.to_vec(),
        top_encode_to_vec_u8(&nonce).unwrap(),
        Vec::new(), // value = 0
    ];
    topics.append(&mut new_uris);
    let esdt_nft_create_log = TxLog {
        address: tx_input.from,
        endpoint: ESDT_NFT_ADD_URI_FUNC_NAME.to_vec(),
        topics,
        data: vec![],
    };

    let tx_result = TxResult {
        result_status: 0,
        result_message: String::new(),
        result_values: Vec::new(),
        result_logs: vec![esdt_nft_create_log],
        result_calls: TxResultCalls::empty(),
    };

    (tx_result, tx_cache.into_blockchain_updates())
}
