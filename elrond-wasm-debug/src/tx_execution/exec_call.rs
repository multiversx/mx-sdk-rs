use std::{collections::HashMap, rc::Rc};

use crate::{
    tx_mock::{
        async_call_tx_input, async_callback_tx_input, merge_results, TxCache, TxContext, TxInput,
        TxResult, TxResultCalls,
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
            if state.accounts.contains_key(&async_data.to) {
                let async_input = async_call_tx_input(&async_data);

                let async_result = sc_call_with_async_and_callback(async_input, state, false);

                tx_result = merge_results(tx_result, async_result.clone());

                let callback_input = async_callback_tx_input(&async_data, &async_result);
                let callback_result = sc_call(callback_input, state, false);
                assert!(
                    tx_result.result_calls.async_call.is_none(),
                    "successive asyncs currently not supported"
                );
                tx_result = merge_results(tx_result, callback_result);
            } else {
                let tx_cache = TxCache::new(state.clone());
                tx_cache.subtract_egld_balance(&contract_address, &async_data.call_value);
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
            }
        }

        for te_call in result_calls.transfer_execute {
            let te_input = async_call_tx_input(&te_call);

            let te_result = sc_call(te_input, state, false);

            tx_result = merge_results(tx_result, te_result.clone());
        }
    }

    tx_result
}
