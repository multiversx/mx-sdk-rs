use std::{collections::HashMap, rc::Rc};

use elrond_wasm::types::Address;

use crate::{
    tx_mock::{
        async_call_tx_input, async_callback_tx_input, async_promise_tx_input, merge_results,
        AsyncCallTxData, Promise, TxCache, TxContext, TxInput, TxInputESDTOrEGLD, TxResult,
        TxResultCalls,
    },
    world_mock::{AccountData, AccountEsdt, BlockchainMock},
};

use super::{execute_builtin_function_or_default, execute_tx_context};

pub fn execute_sc_query(tx_input: TxInput, state: BlockchainMock) -> (TxResult, BlockchainMock) {
    let state_rc = Rc::new(state);
    let tx_cache = TxCache::new(state_rc.clone());
    let tx_context = TxContext::new(tx_input, tx_cache);
    let (_, tx_result) = execute_tx_context(tx_context);
    (tx_result, Rc::try_unwrap(state_rc).unwrap())
}

pub fn execute_sc_call(tx_input: TxInput, mut state: BlockchainMock) -> (TxResult, BlockchainMock) {
    state.subtract_tx_gas(&tx_input.from, tx_input.gas_limit, tx_input.gas_price);

    let state_rc = Rc::new(state);
    let tx_cache = TxCache::new(state_rc.clone());
    let (tx_result, blockchain_updates) = execute_builtin_function_or_default(tx_input, tx_cache);

    let mut state = Rc::try_unwrap(state_rc).unwrap();
    if tx_result.result_status == 0 {
        blockchain_updates.apply(&mut state);
    }

    (tx_result, state)
}

pub fn execute_async_call_and_callback(
    async_data: AsyncCallTxData,
    state: BlockchainMock,
    tx_payments: TxInputESDTOrEGLD,
) -> (TxResult, TxResult, BlockchainMock) {
    if state.accounts.contains_key(&async_data.to) {
        let async_input = async_call_tx_input(&async_data);

        let (async_result, state) = sc_call_with_async_and_callback(async_input, state);

        let callback_input = async_callback_tx_input(&async_data, &async_result);
        let (mut callback_result, state) = execute_sc_call(callback_input, state);

        assert!(
            callback_result.result_calls.async_call.is_none(),
            "successive asyncs currently not supported"
        );
        (async_result, callback_result, state)
    } else {
        let state_rc = Rc::new(state);
        let tx_cache = TxCache::new(state_rc.clone());
        tx_cache.subtract_egld_balance(&async_data.from, &async_data.call_value);
        tx_cache.insert_account(AccountData {
            address: async_data.to.clone(),
            nonce: 0,
            egld_balance: async_data.call_value,
            esdt: AccountEsdt::default(),
            username: Vec::new(),
            storage: HashMap::new(),
            contract_path: None,
            contract_owner: None,
        });
        let blockchain_updates = tx_cache.into_blockchain_updates();
        let mut state = Rc::try_unwrap(state_rc).unwrap();
        state.commit_updates(blockchain_updates);

        (TxResult::empty(), TxResult::empty(), state)
    }
}

// sc_call_with_async_and_callback
// forward_async_retrieve_funds (forwarder)
// execute_async_call_and_callback
// sc_call_with_async_and_callback
// retrieve_funds (vault)
// retrieve_funds_callback (forwarder)
// sc_call_with_async_and_callback
//
//
//

// TODO: refactor
pub fn sc_call_with_async_and_callback(
    tx_input: TxInput,
    state: BlockchainMock,
) -> (TxResult, BlockchainMock) {
    let contract_address = tx_input.to.clone();
    let (mut tx_result, mut state) = execute_sc_call(tx_input, state);
    let result_calls = std::mem::replace(&mut tx_result.result_calls, TxResultCalls::empty());
    if tx_result.result_status == 0 {
        if let Some(async_data) = result_calls.async_call {
            let (async_result, callback_result, new_state) = execute_async_call_and_callback(
                async_data,
                state,
                tx_result.result_calls.transfer_execute_calls.clone(),
            );
            state = new_state;

            tx_result = merge_results(tx_result, async_result);
            tx_result = merge_results(tx_result, callback_result);

            return (tx_result, state);
        }
    }

    for callback in result_calls.promises {
        let (async_result, callback_result, new_state) =
            execute_promise_call_and_callback(&contract_address, &callback, state);
        state = new_state;

        tx_result = merge_results(tx_result, async_result.clone());
        tx_result = merge_results(tx_result, callback_result.clone());
    }

    (tx_result, state)
}

pub fn execute_promise_call_and_callback(
    address: &Address,
    promise: &Promise,
    state: BlockchainMock,
) -> (TxResult, TxResult, BlockchainMock) {
    if state.accounts.contains_key(&promise.endpoint.to) {
        let async_input = async_call_tx_input(&promise.endpoint);

        let (async_result, state) = sc_call_with_async_and_callback(async_input, state);

        let callback_input = async_promise_tx_input(address, promise, &async_result);
        let (callback_result, state) = execute_sc_call(callback_input, state);
        assert!(
            callback_result.result_calls.promises.is_empty(),
            "successive promises currently not supported"
        );
        (async_result, callback_result, state)
    } else {
        let state_rc = Rc::new(state);
        let tx_cache = TxCache::new(state_rc.clone());
        tx_cache.subtract_egld_balance(address, &promise.endpoint.call_value);
        tx_cache.insert_account(AccountData {
            address: promise.endpoint.to.clone(),
            nonce: 0,
            egld_balance: promise.endpoint.call_value.clone(),
            esdt: AccountEsdt::default(),
            username: Vec::new(),
            storage: HashMap::new(),
            contract_path: None,
            contract_owner: None,
        });
        let blockchain_updates = tx_cache.into_blockchain_updates();
        let mut state = Rc::try_unwrap(state_rc).unwrap();
        state.commit_updates(blockchain_updates);

        (TxResult::empty(), TxResult::empty(), state)
    }
}
