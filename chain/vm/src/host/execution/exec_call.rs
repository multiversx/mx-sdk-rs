use crate::{
    blockchain::{
        state::{AccountData, AccountEsdt, BlockchainStateRef},
        VMConfig,
    },
    host::{
        context::{
            async_call_tx_input, async_callback_tx_input, async_promise_callback_tx_input,
            merge_results, AsyncCallTxData, BlockchainUpdate, CallType, Promise, TxCache, TxInput,
            TxPanic, TxResult, TxResultCalls,
        },
        runtime::{RuntimeInstanceCallLambda, RuntimeInstanceCallLambdaDefault, RuntimeRef},
    },
    types::VMCodeMetadata,
};
use num_bigint::BigUint;
use num_traits::Zero;
use std::collections::HashMap;

use super::execute_builtin_function_or_default;

/// Executes transaction and commits changes back to the underlying blockchain state.
///
/// Does not handle async calls.
pub fn commit_call<F>(
    tx_input: TxInput,
    state: &mut BlockchainStateRef,
    runtime: &RuntimeRef,
    f: F,
) -> TxResult
where
    F: RuntimeInstanceCallLambda,
{
    state.subtract_tx_gas(&tx_input.from, tx_input.gas_limit, tx_input.gas_price);

    let tx_cache = TxCache::new(state.get_arc());
    let (mut tx_result, blockchain_updates) =
        execute_builtin_function_or_default(tx_input.clone(), tx_cache, runtime, f);

    if tx_result.result_status.is_success() {
        blockchain_updates.apply(state);
    }

    // TODO: not sure if this is the best place to put this, investigate
    tx_result.append_internal_vm_errors_event_log(&tx_input);

    tx_result
}

/// Executes transaction and commits changes back to the underlying blockchain state.
///
/// Then executes all asyncs recursively, and commits them as well.
pub fn commit_call_with_async_and_callback<F>(
    tx_input: TxInput,
    state: &mut BlockchainStateRef,
    runtime: &RuntimeRef,
    f: F,
) -> TxResult
where
    F: RuntimeInstanceCallLambda,
{
    // main call
    let mut tx_result = commit_call(tx_input, state, runtime, f);

    // take & clear pending calls
    let pending_calls = std::mem::replace(&mut tx_result.pending_calls, TxResultCalls::empty());

    // legacy async call
    // the async call also gets reset
    if tx_result.result_status.is_success() {
        if let Some(async_data) = pending_calls.async_call {
            let (async_result, callback_result) =
                commit_async_call_and_callback(async_data, state, runtime);

            tx_result = merge_results(tx_result, async_result);
            tx_result = merge_results(tx_result, callback_result);

            return tx_result;
        }
    }

    // calling all promises
    // the promises are also reset
    for promise in pending_calls.promises {
        let (async_result, callback_result) =
            commit_promise_call_and_callback(&promise, state, runtime);

        tx_result = merge_results(tx_result, async_result.clone());
        tx_result = merge_results(tx_result, callback_result.clone());
    }

    tx_result
}

/// Asyncs only.
fn commit_async_call_and_callback(
    async_data: AsyncCallTxData,
    state: &mut BlockchainStateRef,
    runtime: &RuntimeRef,
) -> (TxResult, TxResult) {
    if state.account_exists(&async_data.to) {
        let async_input = async_call_tx_input(&async_data, CallType::AsyncCall);

        let async_result = commit_call_with_async_and_callback(
            async_input,
            state,
            runtime,
            RuntimeInstanceCallLambdaDefault,
        );

        let callback_input = async_callback_tx_input(
            &async_data,
            &async_result,
            &runtime.vm_ref.builtin_functions,
        );
        let callback_result = commit_call(
            callback_input,
            state,
            runtime,
            RuntimeInstanceCallLambdaDefault,
        );
        assert!(
            callback_result.pending_calls.async_call.is_none(),
            "successive asyncs currently not supported"
        );
        (async_result, callback_result)
    } else {
        let result = insert_ghost_account(&runtime.vm_ref, &async_data, state);
        match result {
            Ok(blockchain_updates) => {
                state.commit_updates(blockchain_updates);
                (TxResult::empty(), TxResult::empty())
            }
            Err(err) => (TxResult::from_panic_obj(&err), TxResult::empty()),
        }
    }
}

/// Promises only.
fn commit_promise_call_and_callback(
    promise: &Promise,
    state: &mut BlockchainStateRef,
    runtime: &RuntimeRef,
) -> (TxResult, TxResult) {
    if state.account_exists(&promise.call.to) {
        let async_input = async_call_tx_input(&promise.call, CallType::AsyncCall);
        let async_result = commit_call_with_async_and_callback(
            async_input,
            state,
            runtime,
            RuntimeInstanceCallLambdaDefault,
        );
        let callback_result = commit_promises_callback(&async_result, promise, state, runtime);
        (async_result, callback_result)
    } else {
        let result = insert_ghost_account(&runtime.vm_ref, &promise.call, state);
        match result {
            Ok(blockchain_updates) => {
                state.commit_updates(blockchain_updates);
                (TxResult::empty(), TxResult::empty())
            }
            Err(err) => (TxResult::from_panic_obj(&err), TxResult::empty()),
        }
    }
}

fn commit_promises_callback(
    async_result: &TxResult,
    promise: &Promise,
    state: &mut BlockchainStateRef,
    runtime: &RuntimeRef,
) -> TxResult {
    if !promise.has_callback() {
        return TxResult::empty();
    }
    let callback_input =
        async_promise_callback_tx_input(promise, async_result, &runtime.vm_ref.builtin_functions);
    let callback_result = commit_call(
        callback_input,
        state,
        runtime,
        RuntimeInstanceCallLambdaDefault,
    );
    assert!(
        callback_result.pending_calls.promises.is_empty(),
        "successive promises currently not supported"
    );
    callback_result
}

/// When calling a contract that is unknown to the state, we insert a ghost account.
fn insert_ghost_account(
    vm_config: &VMConfig,
    async_data: &AsyncCallTxData,
    state: &mut BlockchainStateRef,
) -> Result<BlockchainUpdate, TxPanic> {
    let address = async_data.to.clone();

    if !vm_config.insert_ghost_accounts {
        panic!("Recipient account not found: {address}");
    }

    let tx_cache = TxCache::new(state.get_arc());
    tx_cache.subtract_egld_balance(&async_data.from, &async_data.call_value)?;
    tx_cache.insert_account(AccountData {
        address,
        nonce: 0,
        egld_balance: async_data.call_value.clone(),
        esdt: AccountEsdt::default(),
        username: Vec::new(),
        storage: HashMap::new(),
        contract_path: None,
        code_metadata: VMCodeMetadata::empty(),
        contract_owner: None,
        developer_rewards: BigUint::zero(),
    });
    Ok(tx_cache.into_blockchain_updates())
}
