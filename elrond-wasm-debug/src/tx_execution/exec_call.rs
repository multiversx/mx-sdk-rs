use std::{collections::HashMap, rc::Rc};

use elrond_wasm::types::Address;

use crate::{
    tx_mock::{
        async_call_tx_input, async_callback_tx_input, async_promise_tx_input, merge_results,
        AsyncCallTxData, Promise, TxCache, TxContext, TxInput, TxResult, TxResultCalls,
    },
    world_mock::{AccountData, AccountEsdt, BlockchainMock},
};

use super::{execute_builtin_function_or_default, execute_tx_context};

pub fn sc_query(tx_input: TxInput, state: Rc<BlockchainMock>) -> TxResult {
    let tx_cache = TxCache::new(state);
    let tx_context = TxContext::new(tx_input, tx_cache);
    let (_, tx_result) = execute_tx_context(tx_context);
    tx_result
}

pub fn sc_call(
    tx_input: TxInput,
    state: &mut Rc<BlockchainMock>,
    increase_nonce: bool, // TODO: flag = code smell, refactor!
) -> TxResult {
    if increase_nonce {
        // nonce gets increased irrespective of whether the tx fails or not
        state.increase_account_nonce(&tx_input.from);
    }
    state.subtract_tx_gas(&tx_input.from, tx_input.gas_limit, tx_input.gas_price);

    let tx_cache = TxCache::new(state.clone());
    let (tx_result, blockchain_updates) = execute_builtin_function_or_default(tx_input, tx_cache);

    if tx_result.result_status == 0 {
        blockchain_updates.apply(Rc::get_mut(state).unwrap());
    }

    tx_result
}

pub fn execute_async_call_and_callback(
    async_data: AsyncCallTxData,
    state: &mut Rc<BlockchainMock>,
) -> (TxResult, TxResult) {
    if state.accounts.contains_key(&async_data.to) {
        let async_input = async_call_tx_input(&async_data);

        let async_result = sc_call_with_async_and_callback(async_input, state, false);

        let callback_input = async_callback_tx_input(&async_data, &async_result);
        let callback_result = sc_call(callback_input, state, false);
        assert!(
            callback_result.result_calls.async_call.is_none(),
            "successive asyncs currently not supported"
        );
        (async_result, callback_result)
    } else {
        let tx_cache = TxCache::new(state.clone());
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
        state.commit_tx_cache(tx_cache);

        (TxResult::empty(), TxResult::empty())
    }
}

// TODO: refactor
pub fn sc_call_with_async_and_callback(
    tx_input: TxInput,
    state: &mut Rc<BlockchainMock>,
    increase_nonce: bool,
) -> TxResult {
    let contract_address = tx_input.to.clone();

    let mut tx_result = sc_call(tx_input, state, increase_nonce);
    let result_calls = std::mem::replace(&mut tx_result.result_calls, TxResultCalls::empty());
    if tx_result.result_status == 0 {
        if let Some(async_data) = result_calls.async_call {
            let (async_result, callback_result) =
                execute_async_call_and_callback(async_data, state);

            tx_result = merge_results(tx_result, async_result);
            tx_result = merge_results(tx_result, callback_result);
        }
    }

    for callback in result_calls.promises {
        let (async_result, callback_result) =
            execute_promise_call_and_callback(&contract_address, &callback, state);

        tx_result = merge_results(tx_result, async_result.clone());
        tx_result = merge_results(tx_result, callback_result.clone());
    }

    tx_result
}

pub fn execute_promise_call_and_callback(
    address: &Address,
    promise: &Promise,
    state: &mut Rc<BlockchainMock>,
) -> (TxResult, TxResult) {
    if state.accounts.contains_key(&promise.endpoint.to) {
        let async_input = async_call_tx_input(&promise.endpoint);
        let async_result = sc_call_with_async_and_callback(async_input, state, false);

        let callback_input = async_promise_tx_input(&address, &promise, &async_result);
        let callback_result = sc_call(callback_input, state, false);
        assert!(
            callback_result.result_calls.promises.is_empty(),
            "successive promises currently not supported"
        );
        (async_result, callback_result)
    } else {
        let tx_cache = TxCache::new(state.clone());
        tx_cache.subtract_egld_balance(&address, &promise.endpoint.call_value);
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
        state.commit_tx_cache(tx_cache);
        (TxResult::empty(), TxResult::empty())
    }
}
