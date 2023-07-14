use crate::{
    tx_mock::{BlockchainUpdate, TxCache, TxInput, TxResult},
    types::VMAddress,
};

pub fn set_special_role(tx_input: TxInput, tx_cache: TxCache) -> (TxResult, BlockchainUpdate) {
    if tx_input.args.len() < 3 {
        return (
            TxResult::from_vm_error("setSpecialRole too few arguments"),
            BlockchainUpdate::empty(),
        );
    }

    let token_identifier = tx_input.args[0].clone();
    let address = VMAddress::from_slice(tx_input.args[1].as_slice());
    let role = tx_input.args[2].clone();

    tx_cache.with_account_mut(&address, |account| {
        account.esdt.set_special_role(&token_identifier, &role);
    });

    (TxResult::empty(), tx_cache.into_blockchain_updates())
}
