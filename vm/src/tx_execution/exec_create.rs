use crate::{
    tx_mock::{TxCache, TxInput, TxResult},
    types::VMAddress,
    with_shared::Shareable,
    world_mock::BlockchainState,
};

use super::BlockchainVMRef;

impl BlockchainVMRef {
    pub fn sc_create<F>(
        &self,
        tx_input: TxInput,
        contract_path: &[u8],
        state: &mut Shareable<BlockchainState>,
        f: F,
    ) -> (VMAddress, TxResult)
    where
        F: FnOnce(),
    {
        // nonce gets increased irrespective of whether the tx fails or not
        // must be done after computing the new address
        state.increase_account_nonce(&tx_input.from);
        state.subtract_tx_gas(&tx_input.from, tx_input.gas_limit, tx_input.gas_price);

        let (tx_result, new_address, blockchain_updates) = state.with_shared(|state_arc| {
            let tx_cache = TxCache::new(state_arc);

            self.deploy_contract(tx_input, contract_path.to_vec(), tx_cache, f)
        });

        blockchain_updates.apply(state);

        (new_address, tx_result)
    }
}
