use crate::{
    tx_execution::instance_call,
    tx_mock::{
        async_call_tx_input, async_callback_tx_input, async_promise_callback_tx_input,
        merge_results, AsyncCallTxData, BlockchainUpdate, CallType, Promise, TxCache, TxContext,
        TxContextStack, TxInput, TxPanic, TxResult, TxResultCalls,
    },
    types::VMCodeMetadata,
    world_mock::{AccountData, AccountEsdt, BlockchainStateRef},
};
use num_bigint::BigUint;
use num_traits::Zero;
use std::collections::HashMap;

use super::{RuntimeInstanceCall, RuntimeRef};

/// Executes the SC endpoint, as given by the current TxInput in the current TxContext.
///
/// Works directly with the top of the execution stack, that is why it takes no arguments.
///
/// It expectes that the stack is properly set up.
pub fn execute_current_tx_context_input() {
    let tx_context_arc = TxContextStack::static_peek();
    let func_name = tx_context_arc.input_ref().func_name.clone();
    let instance = tx_context_arc
        .runtime_ref
        .vm_ref
        .get_contract_instance(&tx_context_arc);
    instance.call(func_name.as_str()).expect("execution error");
}

impl RuntimeRef {
    pub fn execute_sc_query_lambda<F>(
        &self,
        tx_input: TxInput,
        state: &mut BlockchainStateRef,
        f: F,
    ) -> TxResult
    where
        F: FnOnce(RuntimeInstanceCall<'_>),
    {
        let (tx_result, _) = self.execute_in_debugger(tx_input, state, f);
        tx_result
    }

    pub fn execute_in_debugger<F>(
        &self,
        tx_input: TxInput,
        state: &mut BlockchainStateRef,
        f: F,
    ) -> (TxResult, BlockchainUpdate)
    where
        F: FnOnce(RuntimeInstanceCall<'_>),
    {
        let tx_cache = TxCache::new(state.get_arc());
        let tx_context = TxContext::new(self.clone(), tx_input, tx_cache);
        let tx_context = self.execute_tx_context_in_runtime(tx_context, f);
        tx_context.into_results()
    }

    pub fn execute_builtin_function_or_default<F>(
        &self,
        tx_input: TxInput,
        tx_cache: TxCache,
        f: F,
    ) -> (TxResult, BlockchainUpdate)
    where
        F: FnOnce(RuntimeInstanceCall<'_>),
    {
        self.vm_ref
            .builtin_functions
            .execute_builtin_function_or_else(
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
        state: &mut BlockchainStateRef,
        f: F,
    ) -> TxResult
    where
        F: FnOnce(RuntimeInstanceCall<'_>),
    {
        state.subtract_tx_gas(&tx_input.from, tx_input.gas_limit, tx_input.gas_price);

        let tx_cache = TxCache::new(state.get_arc());
        let (tx_result, blockchain_updates) =
            self.execute_builtin_function_or_default(tx_input, tx_cache, f);

        if tx_result.result_status.is_success() {
            blockchain_updates.apply(state);
        }

        tx_result
    }

    pub fn execute_async_call_and_callback(
        &self,
        async_data: AsyncCallTxData,
        state: &mut BlockchainStateRef,
    ) -> (TxResult, TxResult) {
        if state.accounts.contains_key(&async_data.to) {
            let async_input = async_call_tx_input(&async_data, CallType::AsyncCall);

            let async_result =
                self.sc_call_with_async_and_callback(async_input, state, instance_call);

            let callback_input =
                async_callback_tx_input(&async_data, &async_result, &self.vm_ref.builtin_functions);
            let callback_result = self.execute_sc_call_lambda(callback_input, state, instance_call);
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
        state: &mut BlockchainStateRef,
        f: F,
    ) -> TxResult
    where
        F: FnOnce(RuntimeInstanceCall<'_>),
    {
        // main call
        let mut tx_result = self.execute_sc_call_lambda(tx_input, state, f);

        // take & clear pending calls
        let pending_calls = std::mem::replace(&mut tx_result.pending_calls, TxResultCalls::empty());

        // legacy async call
        // the async call also gets reset
        if tx_result.result_status.is_success() {
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
                self.execute_promise_call_and_callback(&promise, state);

            tx_result = merge_results(tx_result, async_result.clone());
            tx_result = merge_results(tx_result, callback_result.clone());
        }

        tx_result
    }

    pub fn execute_promise_call_and_callback(
        &self,
        promise: &Promise,
        state: &mut BlockchainStateRef,
    ) -> (TxResult, TxResult) {
        if state.accounts.contains_key(&promise.call.to) {
            let async_input = async_call_tx_input(&promise.call, CallType::AsyncCall);
            let async_result =
                self.sc_call_with_async_and_callback(async_input, state, instance_call);
            let callback_result = self.execute_promises_callback(&async_result, promise, state);
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

    fn execute_promises_callback(
        &self,
        async_result: &TxResult,
        promise: &Promise,
        state: &mut BlockchainStateRef,
    ) -> TxResult {
        if !promise.has_callback() {
            return TxResult::empty();
        }
        let callback_input =
            async_promise_callback_tx_input(promise, async_result, &self.vm_ref.builtin_functions);
        let callback_result = self.execute_sc_call_lambda(callback_input, state, instance_call);
        assert!(
            callback_result.pending_calls.promises.is_empty(),
            "successive promises currently not supported"
        );
        callback_result
    }

    /// When calling a contract that is unknown to the state, we insert a ghost account.
    fn insert_ghost_account(
        &self,
        async_data: &AsyncCallTxData,
        state: &mut BlockchainStateRef,
    ) -> Result<BlockchainUpdate, TxPanic> {
        let tx_cache = TxCache::new(state.get_arc());
        tx_cache.subtract_egld_balance(&async_data.from, &async_data.call_value)?;
        tx_cache.insert_account(AccountData {
            address: async_data.to.clone(),
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
}
