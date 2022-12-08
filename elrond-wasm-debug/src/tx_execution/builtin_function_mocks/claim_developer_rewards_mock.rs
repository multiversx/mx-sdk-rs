use num_bigint::BigUint;
use num_traits::Zero;

use crate::tx_mock::{BlockchainUpdate, TxCache, TxInput, TxResult};

pub fn execute_claim_developer_rewards(
    tx_input: TxInput,
    tx_cache: TxCache,
) -> (TxResult, BlockchainUpdate) {
    if !tx_input.args.is_empty() {
        return (
            TxResult::from_vm_error("ClaimDeveloperRewards expects no arguments".to_string()),
            BlockchainUpdate::empty(),
        );
    }

    let mut developer_rewards = BigUint::zero();
    let mut caller_is_owner = false;

    tx_cache.with_account_mut(&tx_input.to, |account| {
        if account.contract_owner == Some(tx_input.from.clone()) {
            developer_rewards = account.developer_rewards.clone();
            account.developer_rewards = BigUint::zero();
            caller_is_owner = true;
        }
    });

    if caller_is_owner {
        tx_cache.increase_egld_balance(&tx_input.from, &developer_rewards);
        (TxResult::empty(), tx_cache.into_blockchain_updates())
    } else {
        (
            TxResult::from_vm_error("operation in account not permitted".to_string()),
            BlockchainUpdate::empty(),
        )
    }
}
