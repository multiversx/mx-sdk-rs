use crate::chain_core::builtin_func_names::SET_USERNAME_FUNC_NAME;
use crate::host::context::{BlockchainUpdate, TxCache, TxInput, TxResult};
use crate::host::runtime::{RuntimeInstanceCallLambda, RuntimeRef};

use super::super::builtin_func_trait::BuiltinFunction;

pub struct SetUsername;

impl BuiltinFunction for SetUsername {
    fn name(&self) -> &str {
        SET_USERNAME_FUNC_NAME
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
        if tx_input.args.len() != 1 {
            return (
                TxResult::from_vm_error("SetUserName expects 1 argument"),
                BlockchainUpdate::empty(),
            );
        }

        let username = tx_input.args[0].clone();
        let success = tx_cache.with_account_mut(&tx_input.to, |account| {
            if account.username.is_empty() {
                account.username = username;
                true
            } else {
                false
            }
        });

        if success {
            (TxResult::empty(), tx_cache.into_blockchain_updates())
        } else {
            (
                TxResult::from_vm_error("SetUserName expects 1 argument"),
                BlockchainUpdate::empty(),
            )
        }
    }
}
