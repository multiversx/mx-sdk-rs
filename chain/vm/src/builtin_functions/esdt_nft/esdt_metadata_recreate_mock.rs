use multiversx_chain_core::types::ReturnCode;

use crate::{
    blockchain::state::EsdtInstanceMetadata,
    chain_core::builtin_func_names::ESDT_METADATA_RECREATE_FUNC_NAME,
    host::context::{BlockchainUpdate, TxCache, TxInput, TxLog, TxResult},
    host::runtime::{RuntimeInstanceCallLambda, RuntimeRef},
    types::{top_decode_u64, top_encode_u64},
};

use super::super::builtin_func_trait::BuiltinFunction;

pub struct ESDTMetadataRecreate;

impl BuiltinFunction for ESDTMetadataRecreate {
    fn name(&self) -> &str {
        ESDT_METADATA_RECREATE_FUNC_NAME
    }

    fn execute<F>(
        &self,
        tx_input: TxInput,
        tx_cache: TxCache,
        _runtime: &RuntimeRef,
        _f: F,
    ) -> (TxResult, BlockchainUpdate)
    where
        F: RuntimeInstanceCallLambda,
    {
        if tx_input.args.len() < 6 {
            let err_result =
                TxResult::from_vm_error("ESDTMetaDataRecreate too few arguments");
            return (err_result, BlockchainUpdate::empty());
        }
        assert!(
            tx_input.to == tx_input.from,
            "ESDTMetaDataRecreate expects that to == from"
        );

        let token_identifier = tx_input.args[0].as_slice();
        let nonce = top_decode_u64(tx_input.args[1].as_slice());
        let name = tx_input.args[2].clone();
        let royalties = top_decode_u64(tx_input.args[3].as_slice());
        let hash = tx_input.args[4].clone();
        let attributes = tx_input.args[5].clone();
        let uris = tx_input.args[6..].to_vec();

        tx_cache.with_account_mut(&tx_input.from, |account| {
            let esdt_data = account
                .esdt
                .get_mut_by_identifier(token_identifier)
                .unwrap_or_else(|| panic!("ESDTMetaDataRecreate: token not found"));

            let instance = esdt_data
                .instances
                .get_mut_by_nonce(nonce)
                .unwrap_or_else(|| panic!("ESDTMetaDataRecreate: nonce not found"));

            // Recreate replaces all metadata fields unconditionally.
            instance.metadata = EsdtInstanceMetadata {
                name,
                creator: instance.metadata.creator.clone(),
                royalties,
                hash: Some(hash),
                uri: uris,
                attributes,
            };
        });

        let esdt_metadata_recreate_log = TxLog {
            address: tx_input.from,
            endpoint: ESDT_METADATA_RECREATE_FUNC_NAME.into(),
            topics: vec![
                token_identifier.to_vec(),
                top_encode_u64(nonce),
                Vec::new(), // value = 0
            ],
            data: vec![],
        };

        let tx_result = TxResult {
            result_status: ReturnCode::Success,
            result_logs: vec![esdt_metadata_recreate_log],
            ..Default::default()
        };

        (tx_result, tx_cache.into_blockchain_updates())
    }
}
