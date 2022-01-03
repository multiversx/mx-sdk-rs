use std::rc::Rc;

use crate::{
    tx_mock::{TxCache, TxInput, TxResult},
    world_mock::BlockchainMock,
};

use super::deploy_contract;

pub fn sc_create(
    tx_input: TxInput,
    contract_path: &[u8],
    state: &mut Rc<BlockchainMock>,
) -> TxResult {
    // nonce gets increased irrespective of whether the tx fails or not
    // must be done after computing the new address
    state.increase_account_nonce(&tx_input.from);
    state.subtract_tx_gas(&tx_input.from, tx_input.gas_limit, tx_input.gas_price);

    let tx_cache = TxCache::new(state.clone());
    let (tx_result, blockchain_updates, _) =
        deploy_contract(tx_input, contract_path.to_vec(), tx_cache);

    blockchain_updates.apply(Rc::get_mut(state).unwrap());

    tx_result
}
