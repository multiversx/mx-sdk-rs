use num_traits::Zero;

use crate::{
    tx_execution::execute_system_sc,
    tx_mock::{
        BlockchainUpdate, CallType, TxCache, TxContext, TxFunctionName, TxInput, TxLog, TxResult,
    },
    types::top_encode_big_uint,
};

use super::{is_system_sc_address, RuntimeInstanceCall, RuntimeRef};

pub fn execute_builtin_function_or_default<F>(
    tx_input: TxInput,
    tx_cache: TxCache,
    runtime: &RuntimeRef,
    f: F,
) -> (TxResult, BlockchainUpdate)
where
    F: FnOnce(RuntimeInstanceCall<'_>),
{
    runtime
        .vm_ref
        .builtin_functions
        .execute_builtin_function_or_else(
            runtime,
            tx_input,
            tx_cache,
            f,
            |tx_input, tx_cache, f| execute_default(tx_input, tx_cache, runtime, f),
        )
}

fn should_execute_sc_call(tx_input: &TxInput) -> bool {
    // execute whitebox calls no matter what
    if tx_input.func_name == TxFunctionName::WHITEBOX_CALL {
        return true;
    }

    // don't execute anything for an EOA
    if !tx_input.to.is_smart_contract_address() {
        return false;
    }

    // calls with empty func name are simple transfers
    !tx_input.func_name.is_empty()
}

fn should_add_transfer_value_log(tx_input: &TxInput) -> bool {
    if tx_input.call_type == CallType::AsyncCallback
        && !tx_input.callback_payments.esdt_values.is_empty()
    {
        // edge case in the VM
        return false;
    }

    if tx_input.call_type == CallType::UpgradeFromSource {
        // already handled in upgradeContract builtin function
        return false;
    }

    if tx_input.call_type != CallType::DirectCall {
        return true;
    }

    // skip for transactions coming directly from scenario json, which should all be coming from user wallets
    tx_input.from.is_smart_contract_address() && !tx_input.egld_value.is_zero()
}

pub(crate) fn create_transfer_value_log(tx_input: &TxInput, call_type: CallType) -> TxLog {
    let mut data = vec![call_type.to_log_bytes(), tx_input.func_name.to_bytes()];
    data.append(&mut tx_input.args.clone());

    if tx_input.esdt_values.is_empty()
        && !tx_input.callback_payments.egld_value.is_zero()
        && tx_input.call_type == CallType::AsyncCallback
    {
        return TxLog {
            address: tx_input.from.clone(),
            endpoint: "transferValueOnly".into(),
            topics: vec![b"".to_vec(), tx_input.to.to_vec()],
            data,
        };
    }

    let egld_value = if tx_input.call_type == CallType::AsyncCallback {
        &tx_input.callback_payments.egld_value
    } else {
        &tx_input.egld_value
    };

    TxLog {
        address: tx_input.from.clone(),
        endpoint: "transferValueOnly".into(),
        topics: vec![top_encode_big_uint(egld_value), tx_input.to.to_vec()],
        data,
    }
}

/// Executes without builtin functions, directly on the contract or the given lambda closure.
pub fn execute_default<F>(
    tx_input: TxInput,
    tx_cache: TxCache,
    runtime: &RuntimeRef,
    f: F,
) -> (TxResult, BlockchainUpdate)
where
    F: FnOnce(RuntimeInstanceCall<'_>),
{
    if let Err(err) =
        tx_cache.transfer_egld_balance(&tx_input.from, &tx_input.to, &tx_input.egld_value)
    {
        return (TxResult::from_panic_obj(&err), BlockchainUpdate::empty());
    }

    let transfer_value_log = if should_add_transfer_value_log(&tx_input) {
        Some(create_transfer_value_log(&tx_input, tx_input.call_type))
    } else {
        None
    };

    // TODO: temporary, will convert to explicit builtin function first
    for esdt_transfer in tx_input.esdt_values.iter() {
        let transfer_result = tx_cache.transfer_esdt_balance(
            &tx_input.from,
            &tx_input.to,
            &esdt_transfer.token_identifier,
            esdt_transfer.nonce,
            &esdt_transfer.value,
        );
        if let Err(err) = transfer_result {
            return (TxResult::from_panic_obj(&err), BlockchainUpdate::empty());
        }
    }

    let (mut tx_result, blockchain_updates) = if is_system_sc_address(&tx_input.to) {
        execute_system_sc(tx_input, tx_cache)
    } else if should_execute_sc_call(&tx_input) {
        let tx_context = TxContext::new(runtime.clone(), tx_input, tx_cache);

        let tx_context = runtime.execute_tx_context_in_runtime(tx_context, f);

        tx_context.into_results()
    } else {
        // no execution
        (TxResult::empty(), tx_cache.into_blockchain_updates())
    };

    if let Some(tv_log) = transfer_value_log {
        tx_result.result_logs.insert(0, tv_log);
    }

    (tx_result, blockchain_updates)
}
