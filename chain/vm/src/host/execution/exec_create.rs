use crate::{
    blockchain::state::BlockchainStateRef,
    host::context::{BlockchainUpdate, TxCache, TxContext, TxInput, TxResult},
    host::runtime::{RuntimeInstanceCallLambda, RuntimeRef},
    types::{VMAddress, VMCodeMetadata},
};

/// Executes deploy transaction and commits changes back to the underlying blockchain state.
pub fn commit_deploy<F>(
    tx_input: TxInput,
    contract_path: &[u8],
    code_metadata: VMCodeMetadata,
    state: &mut BlockchainStateRef,
    runtime: &RuntimeRef,
    f: F,
) -> (VMAddress, TxResult)
where
    F: RuntimeInstanceCallLambda,
{
    // nonce gets increased irrespective of whether the tx fails or not
    // must be done after computing the new address
    state.increase_account_nonce(&tx_input.from);
    state.subtract_tx_gas(&tx_input.from, tx_input.gas_limit, tx_input.gas_price);

    let tx_cache = TxCache::new(state.get_arc());

    let (mut tx_result, new_address, blockchain_updates) = execute_deploy(
        tx_input.clone(),
        contract_path.to_vec(),
        code_metadata,
        tx_cache,
        runtime,
        f,
    );

    if tx_result.result_status.is_success() {
        blockchain_updates.apply(state);
    }

    // TODO: not sure if this is the best place to put this, investigate
    tx_result.append_internal_vm_errors_event_log(&tx_input);

    (new_address, tx_result)
}

/// Runs transaction and produces a `TxResult`.
pub fn execute_deploy<F>(
    mut tx_input: TxInput,
    contract_path: Vec<u8>,
    code_metadata: VMCodeMetadata,
    tx_cache: TxCache,
    runtime: &RuntimeRef,
    f: F,
) -> (TxResult, VMAddress, BlockchainUpdate)
where
    F: RuntimeInstanceCallLambda,
{
    let new_address = tx_cache.get_new_address(&tx_input.from);
    tx_input.to = new_address.clone();
    let tx_context = TxContext::new(runtime.clone(), tx_input, tx_cache);
    let tx_input_ref = tx_context.input_ref();

    if let Err(err) = tx_context
        .tx_cache
        .subtract_egld_balance(&tx_input_ref.from, &tx_input_ref.egld_value)
    {
        return (
            TxResult::from_panic_obj(&err),
            VMAddress::zero(),
            BlockchainUpdate::empty(),
        );
    }
    tx_context.create_new_contract(
        &new_address,
        contract_path,
        code_metadata,
        tx_input_ref.from.clone(),
    );
    tx_context
        .tx_cache
        .increase_egld_balance(&new_address, &tx_input_ref.egld_value);

    let tx_context = runtime.execute(tx_context, f);

    let (tx_result, blockchain_updates) = tx_context.into_results();
    (tx_result, new_address, blockchain_updates)
}
