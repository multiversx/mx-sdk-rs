use crate::{
    tx_mock::{
        async_call_tx_input, async_callback_tx_input, async_promise_tx_input, merge_results,
        AsyncCallTxData, BlockchainUpdate, Promise, TxCache, TxCacheSource, TxContext,
        TxContextStack, TxInput, TxResult, TxResultCalls,
    },
    types::VMAddress,
    with_shared::Shareable,
    world_mock::{AccountData, AccountEsdt, BlockchainState},
};
use num_bigint::BigUint;
use num_traits::Zero;
use std::{collections::HashMap, rc::Rc};

use super::BlockchainVMRef;

/// Executes the SC endpoint, as given by the current TxInput in the current TxContext.
///
/// Works directly with the top of the execution stack, that is why it takes no arguments.
///
/// It expectes that the stack is properly set up.
pub fn execute_current_tx_context_input() {
    let tx_context_rc = TxContextStack::static_peek();
    let func_name = tx_context_rc.input_ref().func_name.clone();
    let instance = tx_context_rc.vm_ref.get_contract_instance(&tx_context_rc);
    instance.call(func_name.as_str()).expect("execution error");
}

impl BlockchainVMRef {
    pub fn execute_sc_query_lambda<F>(
        &self,
        tx_input: TxInput,
        state: &mut Shareable<BlockchainState>,
        f: F,
    ) -> TxResult
    where
        F: FnOnce(),
    {
        let (tx_result, _) = self.execute_in_debugger(tx_input, state, f);
        tx_result
    }

    pub fn execute_in_debugger<S, F>(
        &self,
        tx_input: TxInput,
        state: &mut Shareable<S>,
        f: F,
    ) -> (TxResult, BlockchainUpdate)
    where
        S: TxCacheSource + 'static,
        F: FnOnce(),
    {
        state.with_shared(|state_rc| {
            let tx_cache = TxCache::new(state_rc);
            let mut tx_context_sh =
                Shareable::new(TxContext::new(self.clone(), tx_input, tx_cache));
            TxContextStack::execute_on_vm_stack(&mut tx_context_sh, f);
            tx_context_sh.into_inner().into_results()
        })
    }

    pub fn execute_builtin_function_or_default(
        &self,
        tx_input: TxInput,
        tx_cache: TxCache,
    ) -> (TxResult, BlockchainUpdate) {
        if let Some(builtin_func) = self.builtin_functions.get(&tx_input.func_name) {
            builtin_func.execute(self, tx_input, tx_cache)
        } else {
            self.default_execution(tx_input, tx_cache)
        }
    }

    pub fn execute_sc_call(
        &self,
        tx_input: TxInput,
        mut state: BlockchainState,
    ) -> (TxResult, BlockchainState) {
        state.subtract_tx_gas(&tx_input.from, tx_input.gas_limit, tx_input.gas_price);

        let state_rc = Rc::new(state);
        let tx_cache = TxCache::new(state_rc.clone());
        let (tx_result, blockchain_updates) =
            self.execute_builtin_function_or_default(tx_input, tx_cache);

        let mut state = Rc::try_unwrap(state_rc).unwrap();
        if tx_result.result_status == 0 {
            blockchain_updates.apply(&mut state);
        }

        (tx_result, state)
    }

    pub fn execute_async_call_and_callback(
        &self,
        async_data: AsyncCallTxData,
        state: BlockchainState,
    ) -> (TxResult, TxResult, BlockchainState) {
        if state.accounts.contains_key(&async_data.to) {
            let async_input = async_call_tx_input(&async_data);

            let (async_result, state) = self.sc_call_with_async_and_callback(async_input, state);

            let callback_input =
                async_callback_tx_input(&async_data, &async_result, &self.builtin_functions);
            let (callback_result, state) = self.execute_sc_call(callback_input, state);
            assert!(
                callback_result.pending_calls.async_call.is_none(),
                "successive asyncs currently not supported"
            );
            (async_result, callback_result, state)
        } else {
            let state_rc = Rc::new(state);
            let tx_cache = TxCache::new(state_rc.clone());
            if let Err(err) =
                tx_cache.subtract_egld_balance(&async_data.from, &async_data.call_value)
            {
                let state = Rc::try_unwrap(state_rc).unwrap();
                return (TxResult::from_panic_obj(&err), TxResult::empty(), state);
            }
            tx_cache.insert_account(AccountData {
                address: async_data.to.clone(),
                nonce: 0,
                egld_balance: async_data.call_value,
                esdt: AccountEsdt::default(),
                username: Vec::new(),
                storage: HashMap::new(),
                contract_path: None,
                contract_owner: None,
                developer_rewards: BigUint::zero(),
            });
            let blockchain_updates = tx_cache.into_blockchain_updates();
            let mut state = Rc::try_unwrap(state_rc).unwrap();
            state.commit_updates(blockchain_updates);

            (TxResult::empty(), TxResult::empty(), state)
        }
    }

    // TODO: refactor
    pub fn sc_call_with_async_and_callback(
        &self,
        tx_input: TxInput,
        state: BlockchainState,
    ) -> (TxResult, BlockchainState) {
        let contract_address = tx_input.to.clone();
        let (mut tx_result, mut state) = self.execute_sc_call(tx_input, state);

        // take & clear pending calls
        let pending_calls = std::mem::replace(&mut tx_result.pending_calls, TxResultCalls::empty());

        // legacy async call
        // the async call also gets reset
        if tx_result.result_status == 0 {
            if let Some(async_data) = pending_calls.async_call {
                let (async_result, callback_result, new_state) =
                    self.execute_async_call_and_callback(async_data, state);
                state = new_state;

                tx_result = merge_results(tx_result, async_result);
                tx_result = merge_results(tx_result, callback_result);

                return (tx_result, state);
            }
        }

        // calling all promises
        // the promises are also reset
        for promise in pending_calls.promises {
            let (async_result, callback_result, new_state) =
                self.execute_promise_call_and_callback(&contract_address, &promise, state);
            state = new_state;

            tx_result = merge_results(tx_result, async_result.clone());
            tx_result = merge_results(tx_result, callback_result.clone());
        }

        (tx_result, state)
    }

    pub fn execute_promise_call_and_callback(
        &self,
        address: &VMAddress,
        promise: &Promise,
        state: BlockchainState,
    ) -> (TxResult, TxResult, BlockchainState) {
        if state.accounts.contains_key(&promise.call.to) {
            let async_input = async_call_tx_input(&promise.call);
            let (async_result, state) = self.sc_call_with_async_and_callback(async_input, state);

            let callback_input = async_promise_tx_input(address, promise, &async_result);
            let (callback_result, state) = self.execute_sc_call(callback_input, state);
            assert!(
                callback_result.pending_calls.promises.is_empty(),
                "successive promises currently not supported"
            );
            (async_result, callback_result, state)
        } else {
            let state_rc = Rc::new(state);
            let tx_cache = TxCache::new(state_rc.clone());
            if let Err(err) = tx_cache.subtract_egld_balance(address, &promise.call.call_value) {
                let state = Rc::try_unwrap(state_rc).unwrap();
                return (TxResult::from_panic_obj(&err), TxResult::empty(), state);
            }
            tx_cache.insert_account(AccountData {
                address: promise.call.to.clone(),
                nonce: 0,
                egld_balance: promise.call.call_value.clone(),
                esdt: AccountEsdt::default(),
                username: Vec::new(),
                storage: HashMap::new(),
                contract_path: None,
                contract_owner: None,
                developer_rewards: BigUint::zero(),
            });
            let blockchain_updates = tx_cache.into_blockchain_updates();
            let mut state = Rc::try_unwrap(state_rc).unwrap();
            state.commit_updates(blockchain_updates);

            (TxResult::empty(), TxResult::empty(), state)
        }
    }
}
