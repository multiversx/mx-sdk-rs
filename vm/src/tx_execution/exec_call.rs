use crate::{
    tx_mock::{
        async_call_tx_input, async_callback_tx_input, async_promise_tx_input, merge_results,
        AsyncCallTxData, BlockchainUpdate, Promise, TxCache, TxContext, TxContextStack, TxInput,
        TxPanic, TxResult, TxResultCalls,
    },
    types::VMAddress,
    with_shared::Shareable,
    world_mock::{AccountData, AccountEsdt, BlockchainState},
};
use num_bigint::BigUint;
use num_traits::Zero;
use std::collections::HashMap;

use super::BlockchainVMRef;

/// Executes the SC endpoint, as given by the current TxInput in the current TxContext.
///
/// Works directly with the top of the execution stack, that is why it takes no arguments.
///
/// It expectes that the stack is properly set up.
pub fn execute_current_tx_context_input() {
    let tx_context_arc = TxContextStack::static_peek();
    let func_name = tx_context_arc.input_ref().func_name.clone();
    let instance = tx_context_arc.vm_ref.get_contract_instance(&tx_context_arc);
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

    pub fn execute_in_debugger<F>(
        &self,
        tx_input: TxInput,
        state: &mut Shareable<BlockchainState>,
        f: F,
    ) -> (TxResult, BlockchainUpdate)
    where
        F: FnOnce(),
    {
        state.with_shared(|state_arc| {
            let tx_cache = TxCache::new(state_arc);
            let mut tx_context_sh =
                Shareable::new(TxContext::new(self.clone(), tx_input, tx_cache));
            TxContextStack::execute_on_vm_stack(&mut tx_context_sh, f);
            tx_context_sh.into_inner().into_results()
        })
    }

    pub fn execute_builtin_function_or_default<F>(
        &self,
        tx_input: TxInput,
        tx_cache: TxCache,
        f: F,
    ) -> (TxResult, BlockchainUpdate)
    where
        F: FnOnce(),
    {
        self.builtin_functions.execute_builtin_function_or_else(
            self,
            tx_input,
            tx_cache,
            f,
            |tx_input, tx_cache, f| self.default_execution(tx_input, tx_cache, f),
        )
    }

    pub fn execute_sc_call_lambda<F>(
        &self,
        tx_input: TxInput,
        state: &mut Shareable<BlockchainState>,
        f: F,
    ) -> TxResult
    where
        F: FnOnce(),
    {
        state.subtract_tx_gas(&tx_input.from, tx_input.gas_limit, tx_input.gas_price);

        let (tx_result, blockchain_updates) = state.with_shared(|state_arc| {
            let tx_cache = TxCache::new(state_arc);
            self.execute_builtin_function_or_default(tx_input, tx_cache, f)
        });

        if tx_result.result_status == 0 {
            blockchain_updates.apply(state);
        }

        tx_result
    }

    pub fn execute_async_call_and_callback(
        &self,
        async_data: AsyncCallTxData,
        state: &mut Shareable<BlockchainState>,
    ) -> (TxResult, TxResult) {
        if state.accounts.contains_key(&async_data.to) {
            let async_input = async_call_tx_input(&async_data);

            let async_result = self.sc_call_with_async_and_callback(
                async_input,
                state,
                execute_current_tx_context_input,
            );

            let callback_input =
                async_callback_tx_input(&async_data, &async_result, &self.builtin_functions);
            let callback_result = self.execute_sc_call_lambda(
                callback_input,
                state,
                execute_current_tx_context_input,
            );
            assert!(
                callback_result.pending_calls.async_call.is_none(),
                "successive asyncs currently not supported"
            );
            (async_result, callback_result)
        } else {
            let result = self.insert_ghost_account(&async_data, state);
            match result {
                Ok(blockchain_updates) => {
                    state.commit_updates(blockchain_updates);
                    (TxResult::empty(), TxResult::empty())
                },
                Err(err) => (TxResult::from_panic_obj(&err), TxResult::empty()),
            }
        }
    }

    // TODO: refactor
    pub fn sc_call_with_async_and_callback<F>(
        &self,
        tx_input: TxInput,
        state: &mut Shareable<BlockchainState>,
        f: F,
    ) -> TxResult
    where
        F: FnOnce(),
    {
        // main call
        let contract_address = tx_input.to.clone();
        let mut tx_result = self.execute_sc_call_lambda(tx_input, state, f);

        // take & clear pending calls
        let pending_calls = std::mem::replace(&mut tx_result.pending_calls, TxResultCalls::empty());

        // legacy async call
        // the async call also gets reset
        if tx_result.result_status == 0 {
            if let Some(async_data) = pending_calls.async_call {
                let (async_result, callback_result) =
                    self.execute_async_call_and_callback(async_data, state);

                tx_result = merge_results(tx_result, async_result);
                tx_result = merge_results(tx_result, callback_result);

                return tx_result;
            }
        }

        // calling all promises
        // the promises are also reset
        for promise in pending_calls.promises {
            let (async_result, callback_result) =
                self.execute_promise_call_and_callback(&contract_address, &promise, state);

            tx_result = merge_results(tx_result, async_result.clone());
            tx_result = merge_results(tx_result, callback_result.clone());
        }

        tx_result
    }

    pub fn execute_promise_call_and_callback(
        &self,
        address: &VMAddress,
        promise: &Promise,
        state: &mut Shareable<BlockchainState>,
    ) -> (TxResult, TxResult) {
        if state.accounts.contains_key(&promise.call.to) {
            let async_input = async_call_tx_input(&promise.call);
            let async_result = self.sc_call_with_async_and_callback(
                async_input,
                state,
                execute_current_tx_context_input,
            );

            let callback_input = async_promise_tx_input(address, promise, &async_result);
            let callback_result = self.execute_sc_call_lambda(
                callback_input,
                state,
                execute_current_tx_context_input,
            );
            assert!(
                callback_result.pending_calls.promises.is_empty(),
                "successive promises currently not supported"
            );
            (async_result, callback_result)
        } else {
            let result = self.insert_ghost_account(&promise.call, state);
            match result {
                Ok(blockchain_updates) => {
                    state.commit_updates(blockchain_updates);
                    (TxResult::empty(), TxResult::empty())
                },
                Err(err) => (TxResult::from_panic_obj(&err), TxResult::empty()),
            }
        }
    }

    /// When calling a contract that is unknown to the state, we insert a ghost account.
    fn insert_ghost_account(
        &self,
        async_data: &AsyncCallTxData,
        state: &mut Shareable<BlockchainState>,
    ) -> Result<BlockchainUpdate, TxPanic> {
        state.with_shared(|state_arc| {
            let tx_cache = TxCache::new(state_arc);
            tx_cache.subtract_egld_balance(&async_data.from, &async_data.call_value)?;
            tx_cache.insert_account(AccountData {
                address: async_data.to.clone(),
                nonce: 0,
                egld_balance: async_data.call_value.clone(),
                esdt: AccountEsdt::default(),
                username: Vec::new(),
                storage: HashMap::new(),
                contract_path: None,
                contract_owner: None,
                developer_rewards: BigUint::zero(),
            });
            Ok(tx_cache.into_blockchain_updates())
        })
    }
}
