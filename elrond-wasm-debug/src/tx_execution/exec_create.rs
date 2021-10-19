use std::rc::Rc;

use elrond_wasm::types::Address;

use crate::{
    tx_mock::{TxContextRef, TxInput, TxResult},
    world_mock::{BlockchainMock, BlockchainMockError},
};

use super::execute_tx_context;

fn get_new_address(tx_input: &TxInput, state: Rc<BlockchainMock>) -> Address {
    let sender = state
        .accounts
        .get(&tx_input.from)
        .unwrap_or_else(|| panic!("scDeploy sender does not exist"));
    state
        .get_new_address(tx_input.from.clone(), sender.nonce)
        .unwrap_or_else(|| {
            panic!("Missing new address. Only explicit new deploy addresses supported")
        })
}

pub fn sc_create(
    mut tx_input: TxInput,
    contract_path: &[u8],
    state: &mut Rc<BlockchainMock>,
) -> Result<TxResult, BlockchainMockError> {
    let new_address = get_new_address(&tx_input, state.clone());
    tx_input.to = new_address.clone();

    // nonce gets increased irrespective of whether the tx fails or not
    // must be done after computing the new address
    state.increase_account_nonce(&tx_input.from);
    state.subtract_tx_gas(&tx_input.from, tx_input.gas_limit, tx_input.gas_price);

    let tx_context = TxContextRef::new(tx_input, state.clone());
    let tx_input_ref = &*tx_context.tx_input_box;

    tx_context
        .blockchain_cache
        .subtract_egld_balance(&tx_input_ref.from, &tx_input_ref.egld_value)?;
    tx_context.create_new_contract(
        &new_address,
        contract_path.to_vec(),
        tx_input_ref.from.clone(),
    );
    tx_context
        .blockchain_cache
        .increase_egld_balance(&new_address, &tx_input_ref.egld_value);

    let tx_result = execute_tx_context(tx_context.clone());

    let blockchain_updates = tx_context.into_blockchain_updates();
    blockchain_updates.apply(Rc::get_mut(state).unwrap());

    Ok(tx_result)
}
