use std::sync::Mutex;

use multiversx_chain_vm::host::context::TxContextRef;

use super::ContractDebugInstance;

thread_local!(
    static API_STACK: Mutex<ContractDebugStack> = Mutex::new(ContractDebugStack::default())
);

#[derive(Debug, Default)]
pub struct ContractDebugStack(Vec<ContractDebugInstance>);

impl ContractDebugStack {
    pub fn static_peek() -> ContractDebugInstance {
        API_STACK.with(|cell| {
            let stack = cell.lock().unwrap();
            stack.0.last().unwrap().clone()
        })
    }

    /// Searches the stack based on `TxContext` pointer (no deep equls performed).
    ///
    /// Used for resolving `DebugHandle`s.
    pub fn find_by_tx_context(tx_context_ref: &TxContextRef) -> ContractDebugInstance {
        API_STACK.with(|cell| {
            let stack = cell.lock().unwrap();
            stack
                .0
                .iter()
                .find(|instance| TxContextRef::ptr_eq(tx_context_ref, &instance.tx_context_ref))
                .expect("invalid TxContext: not found on ContractDebugStack")
                .clone()
        })
    }

    pub fn static_push(tx_context_arc: ContractDebugInstance) {
        API_STACK.with(|cell| {
            let mut stack = cell.lock().unwrap();
            stack.0.push(tx_context_arc);
        })
    }

    pub fn static_pop() -> ContractDebugInstance {
        API_STACK.with(|cell| {
            let mut stack = cell.lock().unwrap();
            stack.0.pop().unwrap()
        })
    }
}
