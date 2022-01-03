use elrond_wasm::{
    api::ESDT_NFT_CREATE_FUNC_NAME,
    elrond_codec::{top_encode_to_vec_u8, TopDecode},
};
use num_bigint::BigUint;

use crate::{
    tx_mock::{BlockchainUpdate, TxCache, TxInput, TxLog, TxResult, TxResultCalls},
    world_mock::{EsdtInstance, EsdtInstanceMetadata},
};

pub fn execute_esdt_nft_create(
    tx_input: TxInput,
    tx_cache: TxCache,
) -> (TxResult, BlockchainUpdate) {
    if tx_input.args.len() < 7 {
        let err_result = TxResult::from_vm_error("ESDTNFTCreate too few arguments".to_string());
        return (err_result, BlockchainUpdate::empty());
    }
    assert!(
        tx_input.to == tx_input.from,
        "ESDTNFTCreate expects that to == from"
    );

    let token_identifier = tx_input.args[0].as_slice();
    let amount = BigUint::from_bytes_be(tx_input.args[1].as_slice());
    let name = tx_input.args[2].clone();
    let royalties = u64::top_decode(tx_input.args[3].as_slice()).unwrap();
    let hash = tx_input.args[4].clone();
    let attributes = tx_input.args[5].clone();
    let uri = tx_input.args[6].clone();

    let new_nonce = tx_cache.with_account_mut(&tx_input.to, |account| {
        let esdt_data = account
            .esdt
            .get_mut_by_identifier(token_identifier)
            .unwrap_or_else(|| panic!("ESDTNFTCreate unexpected token identifier"));
        esdt_data.last_nonce += 1;
        esdt_data.instances.push_instance(EsdtInstance {
            nonce: esdt_data.last_nonce,
            balance: amount.clone(),
            metadata: EsdtInstanceMetadata {
                name,
                creator: Some(tx_input.from.clone()),
                royalties,
                hash: Some(hash),
                uri: Some(uri),
                attributes,
            },
        });

        esdt_data.last_nonce
    });

    let esdt_nft_create_log = TxLog {
        address: tx_input.from.clone(),
        endpoint: ESDT_NFT_CREATE_FUNC_NAME.to_vec(),
        topics: vec![
            token_identifier.to_vec(),
            top_encode_to_vec_u8(&new_nonce).unwrap(),
            amount.to_bytes_be(),
            Vec::new(), // in actuality this should contain the fully marshalled ESDT data
        ],
        data: vec![],
    };

    let tx_result = TxResult {
        result_status: 0,
        result_message: String::new(),
        result_values: vec![top_encode_to_vec_u8(&new_nonce).unwrap()],
        result_logs: vec![esdt_nft_create_log],
        result_calls: TxResultCalls::empty(),
    };

    (tx_result, tx_cache.into_blockchain_updates())
}
