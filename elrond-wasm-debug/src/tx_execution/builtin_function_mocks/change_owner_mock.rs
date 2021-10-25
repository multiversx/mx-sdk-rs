use elrond_wasm::{elrond_codec::TopDecode, types::Address};

use crate::tx_mock::{BlockchainUpdate, TxCache, TxInput, TxResult};

pub fn execute_change_owner(tx_input: TxInput, tx_cache: TxCache) -> (TxResult, BlockchainUpdate) {
    if tx_input.args.len() != 1 {
        return (
            TxResult::from_vm_error("ChangeOwnerAddress expects 1 argument".to_string()),
            BlockchainUpdate::empty(),
        );
    }

    let new_owner_address = Address::top_decode(tx_input.args[0].as_slice()).unwrap();
    tx_cache.with_account_mut(&tx_input.to, |account| {
        account.contract_owner = Some(new_owner_address);
    });

    (TxResult::empty(), tx_cache.into_blockchain_updates())
}
