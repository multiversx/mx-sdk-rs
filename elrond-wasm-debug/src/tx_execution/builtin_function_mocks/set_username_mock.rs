use crate::tx_mock::{BlockchainUpdate, TxCache, TxInput, TxResult};

pub fn execute_set_username(tx_input: TxInput, tx_cache: TxCache) -> (TxResult, BlockchainUpdate) {
    if tx_input.args.len() != 1 {
        return (
            TxResult::from_vm_error("SetUserName expects 1 argument".to_string()),
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
            TxResult::from_vm_error("SetUserName expects 1 argument".to_string()),
            BlockchainUpdate::empty(),
        )
    }
}
