use elrond_wasm::{elrond_codec::TopDecode, types::heap::Address};
use num_bigint::BigUint;
use num_traits::Zero;

use crate::{
    tx_execution::default_execution,
    tx_mock::{BlockchainUpdate, TxCache, TxInput, TxResult},
};

pub fn execute_claim_developer_rewards(
    tx_input: TxInput,
    tx_cache: TxCache,
) -> (TxResult, BlockchainUpdate) {
    if tx_input.args.len() != 0 {
        return (
            TxResult::from_vm_error("ClaimDeveloperRewards expects no arguments".to_string()),
            BlockchainUpdate::empty(),
        );
    }

    let caller_address = Address::top_decode(tx_input.args[0].as_slice()).unwrap();
    let func_name = tx_input.args.get(2).map(Vec::clone).unwrap_or_default();

    let mut exec_input = TxInput {
        from: tx_input.from,
        to: tx_input.to,
        egld_value: BigUint::zero(),
        esdt_values: Vec::new(),
        func_name,
        args: Vec::new(),
        gas_limit: tx_input.gas_limit,
        gas_price: tx_input.gas_price,
        tx_hash: tx_input.tx_hash,
    };

    tx_cache.with_account_mut(&tx_input.to, |account| {
        if account.contract_owner == Some(caller_address) {
            exec_input.egld_value = account.developer_rewards.clone();
            let (mut tx_result, blockchain_updates) = default_execution(exec_input, tx_cache);
            account.developer_rewards = BigUint::zero();
        }
    });

    (TxResult::empty(), tx_cache.into_blockchain_updates())
}
