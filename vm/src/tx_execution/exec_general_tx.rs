use multiversx_sc::types::heap::Address;
use num_traits::Zero;

use crate::{
    tx_mock::{BlockchainUpdate, TxCache, TxContext, TxFunctionName, TxInput, TxLog, TxResult},
    world_mock::is_smart_contract_address,
};

use super::execute_tx_context;

pub fn default_execution(tx_input: TxInput, tx_cache: TxCache) -> (TxResult, BlockchainUpdate) {
    let mut tx_context = TxContext::new(tx_input, tx_cache);

    tx_context.tx_cache.subtract_egld_balance(
        &tx_context.tx_input_box.from,
        &tx_context.tx_input_box.egld_value,
    );
    tx_context.tx_cache.increase_egld_balance(
        &tx_context.tx_input_box.to,
        &tx_context.tx_input_box.egld_value,
    );

    // skip for transactions coming directly from scenario json, which should all be coming from user wallets
    // TODO: reorg context logic
    let add_transfer_log = is_smart_contract_address(&tx_context.tx_input_box.from)
        && !tx_context.tx_input_box.egld_value.is_zero();
    let transfer_value_log = if add_transfer_log {
        Some(TxLog {
            address: Address::zero(), // TODO: figure out the real VM behavior
            endpoint: "transferValueOnly".into(),
            topics: vec![
                tx_context.tx_input_box.from.to_vec(),
                tx_context.tx_input_box.to.to_vec(),
                tx_context.tx_input_box.egld_value.to_bytes_be(),
            ],
            data: Vec::new(),
        })
    } else {
        None
    };

    // TODO: temporary, will convert to explicit builtin function first
    for esdt_transfer in tx_context.tx_input_box.esdt_values.iter() {
        tx_context.tx_cache.transfer_esdt_balance(
            &tx_context.tx_input_box.from,
            &tx_context.tx_input_box.to,
            &esdt_transfer.token_identifier,
            esdt_transfer.nonce,
            &esdt_transfer.value,
        );
    }

    let mut tx_result = if !is_smart_contract_address(&tx_context.tx_input_box.to)
        || tx_context.tx_input_box.func_name.is_empty()
    {
        // direct EGLD transfer
        TxResult::empty()
    } else {
        let (tx_context_modified, tx_result) = execute_tx_context(tx_context);
        tx_context = tx_context_modified;
        tx_result
    };

    if let Some(tv_log) = transfer_value_log {
        tx_result.result_logs.insert(0, tv_log);
    }

    let blockchain_updates = tx_context.into_blockchain_updates();

    (tx_result, blockchain_updates)
}

pub fn deploy_contract(
    mut tx_input: TxInput,
    contract_path: Vec<u8>,
    tx_cache: TxCache,
) -> (TxResult, Address, BlockchainUpdate) {
    let new_address = tx_cache.get_new_address(&tx_input.from);
    tx_input.to = new_address.clone();
    tx_input.func_name = TxFunctionName::INIT;
    let tx_context = TxContext::new(tx_input, tx_cache);
    let tx_input_ref = &*tx_context.tx_input_box;

    tx_context
        .tx_cache
        .subtract_egld_balance(&tx_input_ref.from, &tx_input_ref.egld_value);
    tx_context.create_new_contract(&new_address, contract_path, tx_input_ref.from.clone());
    tx_context
        .tx_cache
        .increase_egld_balance(&new_address, &tx_input_ref.egld_value);

    let (tx_context, tx_result) = execute_tx_context(tx_context);
    let blockchain_updates = tx_context.into_blockchain_updates();

    (tx_result, new_address, blockchain_updates)
}
