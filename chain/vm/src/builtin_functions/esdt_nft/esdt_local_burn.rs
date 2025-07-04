use multiversx_chain_core::types::ReturnCode;
use num_bigint::BigUint;

use crate::{
    chain_core::builtin_func_names::ESDT_LOCAL_BURN_FUNC_NAME,
    host::context::{BlockchainUpdate, TxCache, TxInput, TxLog, TxResult},
    host::runtime::{RuntimeInstanceCallLambda, RuntimeRef},
};

use super::super::builtin_func_trait::BuiltinFunction;

pub struct ESDTLocalBurn;

impl BuiltinFunction for ESDTLocalBurn {
    fn name(&self) -> &str {
        ESDT_LOCAL_BURN_FUNC_NAME
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
        if tx_input.args.len() != 2 {
            let err_result = TxResult::from_vm_error("ESDTLocalBurn expects 2 arguments");
            return (err_result, BlockchainUpdate::empty());
        }

        let token_identifier = tx_input.args[0].clone();
        let value = BigUint::from_bytes_be(tx_input.args[1].as_slice());

        let subtract_result =
            tx_cache.subtract_esdt_balance(&tx_input.to, &token_identifier, 0, &value);
        if let Err(err) = subtract_result {
            return (TxResult::from_panic_obj(&err), BlockchainUpdate::empty());
        }

        let esdt_nft_create_log = TxLog {
            address: tx_input.from,
            endpoint: ESDT_LOCAL_BURN_FUNC_NAME.into(),
            topics: vec![token_identifier.to_vec(), Vec::new(), value.to_bytes_be()],
            data: vec![],
        };

        let tx_result = TxResult {
            result_status: ReturnCode::Success,
            result_logs: vec![esdt_nft_create_log],
            ..Default::default()
        };

        (tx_result, tx_cache.into_blockchain_updates())
    }
}
