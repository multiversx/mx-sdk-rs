use crate::chain_core::builtin_func_names::MIGRATE_USERNAME_FUNC_NAME;
use crate::host::context::{BlockchainUpdate, TxCache, TxInput, TxResult};
use crate::host::runtime::{RuntimeInstanceCallLambda, RuntimeRef};

use super::super::builtin_func_trait::BuiltinFunction;

pub struct MigrateUserName;

type BlockchainResult = Result<(TxResult, BlockchainUpdate), TxResult>;

impl BuiltinFunction for MigrateUserName {
    fn name(&self) -> &str {
        MIGRATE_USERNAME_FUNC_NAME
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
        self.execute_with_result(tx_input, tx_cache)
            .unwrap_or_else(|err_result| (err_result, BlockchainUpdate::empty()))
    }
}

impl MigrateUserName {
    #[allow(clippy::result_large_err)]
    fn execute_with_result(&self, tx_input: TxInput, tx_cache: TxCache) -> BlockchainResult {
        if tx_input.args.len() != 1 {
            return Result::Err(TxResult::from_vm_error(
                "migrateUserName expects 1 argument",
            ));
        }

        let username = tx_input.args[0].clone();
        tx_cache.with_account_mut(&tx_input.to, |account| {
            if account.username != username {
                return Result::Err(TxResult::from_vm_error("username mismatch"));
            }
            if let Some(name_without_suffix) = username.strip_suffix(".elrond".as_bytes()) {
                account.username = [name_without_suffix, ".x".as_bytes()].concat();
                Ok(())
            } else {
                Result::Err(TxResult::from_vm_error("expected .elrond suffix"))
            }
        })?;

        Result::Ok((TxResult::empty(), tx_cache.into_blockchain_updates()))
    }
}
