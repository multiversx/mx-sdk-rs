use crate::{
    blockchain::state::BlockchainStateRef,
    host::{
        context::{TxCache, TxContext, TxInput, TxResult},
        runtime::{RuntimeInstanceCall, RuntimeRef},
    },
};

/// Executes VM query and discards any changes to the blockchain state.
pub fn execute_query<F>(
    tx_input: TxInput,
    state: &mut BlockchainStateRef,
    runtime: &RuntimeRef,
    f: F,
) -> TxResult
where
    F: FnOnce(RuntimeInstanceCall<'_>),
{
    let tx_cache = TxCache::new(state.get_arc());
    let tx_context = TxContext::new(runtime.clone(), tx_input, tx_cache);
    let tx_context = runtime.execute_tx_context_in_runtime(tx_context, f);
    let (tx_result, _) = tx_context.into_results();
    tx_result
}
