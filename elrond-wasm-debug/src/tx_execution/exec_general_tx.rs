use elrond_wasm::types::Address;

use crate::{
    tx_mock::{
        BlockchainUpdate, StaticStack, TxCache, TxContext, TxContextRef, TxInput, TxResult,
        API_INSTANCE,
    },
    world_mock::is_smart_contract_address,
};

use super::execute_tx_context;

pub fn default_execution(tx_input: TxInput, tx_cache: TxCache) -> (TxResult, BlockchainUpdate) {
    API_INSTANCE.push(TxContext::new(tx_input, tx_cache));
    API_INSTANCE.first().tx_cache.subtract_egld_balance(
        &API_INSTANCE.first().tx_input_box.from,
        &API_INSTANCE.first().tx_input_box.egld_value,
    );
    API_INSTANCE.first().tx_cache.increase_egld_balance(
        &API_INSTANCE.first().tx_input_box.to,
        &API_INSTANCE.first().tx_input_box.egld_value,
    );

    // TODO: temporary, will convert to explicit builtin function first
    for esdt_transfer in API_INSTANCE.first().tx_input_box.esdt_values.iter() {
        API_INSTANCE.first().tx_cache.transfer_esdt_balance(
            &API_INSTANCE.first().tx_input_box.from,
            &API_INSTANCE.first().tx_input_box.to,
            &esdt_transfer.token_identifier,
            esdt_transfer.nonce,
            &esdt_transfer.value,
        );
    }

    let tx_result = if !is_smart_contract_address(&API_INSTANCE.first().tx_input_box.to)
        || API_INSTANCE.first().tx_input_box.func_name.is_empty()
    {
        // direct EGLD transfer
        TxResult::empty()
    } else {
        execute_tx_context(API_INSTANCE.first())
    };

    let blockchain_updates = API_INSTANCE.first().into_blockchain_updates();
    API_INSTANCE.pop();
    (tx_result, blockchain_updates)
}

pub fn deploy_contract(
    mut tx_input: TxInput,
    contract_path: Vec<u8>,
    tx_cache: TxCache,
) -> (TxResult, BlockchainUpdate, Address) {
    let new_address = tx_cache.get_new_address(&tx_input.from);
    tx_input.to = new_address.clone();
    tx_input.func_name = b"init".to_vec();
    API_INSTANCE
        .lock()
        .unwrap()
        .push([TxContext::new(tx_input, tx_cache)]);
    let tx_input_ref = &*API_INSTANCE.first().tx_input_box;

    API_INSTANCE
        .get()
        .tx_cache
        .subtract_egld_balance(&tx_input_ref.from, &tx_input_ref.egld_value);
    API_INSTANCE
        .get()
        .create_new_contract(&new_address, contract_path, tx_input_ref.from.clone());
    API_INSTANCE
        .get()
        .tx_cache
        .increase_egld_balance(&new_address, &tx_input_ref.egld_value);

    let tx_result = execute_tx_context(API_INSTANCE.first());
    let blockchain_updates = TxContextRef::into_blockchain_updates();
    API_INSTANCE.pop();
    (tx_result, blockchain_updates, new_address)
}
