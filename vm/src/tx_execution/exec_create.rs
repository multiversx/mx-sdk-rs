use std::rc::Rc;

use multiversx_sc::types::Address;

use crate::{
    tx_mock::{TxCache, TxInput, TxResult},
    world_mock::BlockchainMock,
};

use super::deploy_contract;

pub fn sc_create(
    tx_input: TxInput,
    contract_path: &[u8],
    mut state: BlockchainMock,
) -> (TxResult, Address, BlockchainMock) {
    // nonce gets increased irrespective of whether the tx fails or not
    // must be done after computing the new address
    state.increase_account_nonce(&tx_input.from);
    state.subtract_tx_gas(&tx_input.from, tx_input.gas_limit, tx_input.gas_price);

    let state_rc = Rc::new(state);
    let tx_cache = TxCache::new(state_rc.clone());
    let (tx_result, new_address, blockchain_updates) =
        deploy_contract(tx_input, contract_path.to_vec(), tx_cache);
    let mut state = Rc::try_unwrap(state_rc).unwrap();

    blockchain_updates.apply(&mut state);

    (tx_result, new_address, state)
}
