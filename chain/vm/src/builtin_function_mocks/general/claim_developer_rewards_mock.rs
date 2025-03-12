use num_bigint::BigUint;
use num_traits::Zero;

use crate::{
    chain_core::builtin_func_names::CLAIM_DEVELOPER_REWARDS_FUNC_NAME,
    host::runtime::{RuntimeInstanceCall, RuntimeRef},
    host::tx_mock::{BlockchainUpdate, TxCache, TxInput, TxResult},
};

use super::super::builtin_func_trait::BuiltinFunction;

pub struct ClaimDeveloperRewards;

impl BuiltinFunction for ClaimDeveloperRewards {
    fn name(&self) -> &str {
        CLAIM_DEVELOPER_REWARDS_FUNC_NAME
    }

    fn execute<F>(
        &self,
        tx_input: TxInput,
        tx_cache: TxCache,
        _runtime: &RuntimeRef,
        _f: F,
    ) -> (TxResult, BlockchainUpdate)
    where
        F: FnOnce(RuntimeInstanceCall<'_>),
    {
        if !tx_input.args.is_empty() {
            return (
                TxResult::from_vm_error("ClaimDeveloperRewards expects no arguments"),
                BlockchainUpdate::empty(),
            );
        }

        let mut developer_rewards = BigUint::zero();
        let mut caller_is_owner = false;

        tx_cache.with_account_mut(&tx_input.to, |account| {
            if account.contract_owner == Some(tx_input.from.clone()) {
                developer_rewards.clone_from(&account.developer_rewards);
                account.developer_rewards = BigUint::zero();
                caller_is_owner = true;
            }
        });

        if caller_is_owner {
            tx_cache.increase_egld_balance(&tx_input.from, &developer_rewards);
            (TxResult::empty(), tx_cache.into_blockchain_updates())
        } else {
            (
                TxResult::from_vm_error("operation in account not permitted"),
                BlockchainUpdate::empty(),
            )
        }
    }
}
