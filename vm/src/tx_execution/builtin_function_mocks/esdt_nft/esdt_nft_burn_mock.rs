use num_bigint::BigUint;

use crate::{
    tx_execution::{builtin_function_names::ESDT_NFT_BURN_FUNC_NAME, BlockchainVMRef},
    tx_mock::{BlockchainUpdate, TxCache, TxInput, TxLog, TxResult},
    types::{top_decode_u64, top_encode_u64},
};

use super::super::builtin_func_trait::BuiltinFunction;

pub struct ESDTNftBurn;

impl BuiltinFunction for ESDTNftBurn {
    fn name(&self) -> &str {
        ESDT_NFT_BURN_FUNC_NAME
    }

    fn execute<F>(
        &self,
        tx_input: TxInput,
        tx_cache: TxCache,
        _vm: &BlockchainVMRef,
        _f: F,
    ) -> (TxResult, BlockchainUpdate)
    where
        F: FnOnce(),
    {
        if tx_input.args.len() != 3 {
            let err_result = TxResult::from_vm_error("ESDTNFTBurn expects 3 arguments");
            return (err_result, BlockchainUpdate::empty());
        }

        let token_identifier = tx_input.args[0].clone();
        let nonce = top_decode_u64(tx_input.args[1].as_slice());
        let value = BigUint::from_bytes_be(tx_input.args[2].as_slice());

        let subtract_result =
            tx_cache.subtract_esdt_balance(&tx_input.to, &token_identifier, nonce, &value);
        if let Err(err) = subtract_result {
            return (TxResult::from_panic_obj(&err), BlockchainUpdate::empty());
        }

        let esdt_nft_create_log = TxLog {
            address: tx_input.from,
            endpoint: ESDT_NFT_BURN_FUNC_NAME.into(),
            topics: vec![
                token_identifier.to_vec(),
                top_encode_u64(nonce),
                value.to_bytes_be(),
            ],
            data: vec![],
        };

        let tx_result = TxResult {
            result_status: 0,
            result_logs: vec![esdt_nft_create_log],
            ..Default::default()
        };

        (tx_result, tx_cache.into_blockchain_updates())
    }
}
