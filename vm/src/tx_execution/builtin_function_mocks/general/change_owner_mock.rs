use crate::tx_execution::{
    builtin_function_names::CHANGE_OWNER_BUILTIN_FUNC_NAME, BlockchainVMRef,
};

use crate::{
    tx_mock::{BlockchainUpdate, TxCache, TxInput, TxResult},
    types::VMAddress,
};

use super::super::builtin_func_trait::BuiltinFunction;

pub struct ChangeOwner;

impl BuiltinFunction for ChangeOwner {
    fn name(&self) -> &str {
        CHANGE_OWNER_BUILTIN_FUNC_NAME
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
        if tx_input.args.len() != 1 {
            return (
                TxResult::from_vm_error("ChangeOwnerAddress expects 1 argument"),
                BlockchainUpdate::empty(),
            );
        }

        let new_owner_address = VMAddress::from_slice(&tx_input.args[0]);
        tx_cache.with_account_mut(&tx_input.to, |account| {
            account.contract_owner = Some(new_owner_address);
        });

        (TxResult::empty(), tx_cache.into_blockchain_updates())
    }
}
