use crate::{
    chain_core::builtin_func_names::DELETE_USERNAME_FUNC_NAME,
    host::context::{BlockchainUpdate, TxCache, TxInput, TxResult},
    host::runtime::{RuntimeInstanceCallLambda, RuntimeRef},
};

use super::super::builtin_func_trait::BuiltinFunction;

pub struct DeleteUsername;

impl BuiltinFunction for DeleteUsername {
    fn name(&self) -> &str {
        DELETE_USERNAME_FUNC_NAME
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
        if !tx_input.args.is_empty() {
            return (
                TxResult::from_vm_error("DeleteUserName expects no arguments"),
                BlockchainUpdate::empty(),
            );
        }

        tx_cache.with_account_mut(&tx_input.to, |account| {
            account.username = Vec::new();
        });

        (TxResult::empty(), tx_cache.into_blockchain_updates())
    }
}
