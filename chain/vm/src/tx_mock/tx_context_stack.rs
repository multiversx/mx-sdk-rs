use crate::with_shared::Shareable;

use super::TxContext;

use std::sync::{Arc, Mutex};

thread_local!(
    static API_STACK: Mutex<TxContextStack> = Mutex::new(TxContextStack::default())
);

#[derive(Debug, Default)]
pub struct TxContextStack(Vec<Arc<TxContext>>);

impl TxContextStack {
    pub fn static_peek() -> Arc<TxContext> {
        API_STACK.with(|cell| {
            let stack = cell.lock().unwrap();
            stack.0.last().unwrap().clone()
        })
    }

    pub fn static_push(tx_context_arc: Arc<TxContext>) {
        API_STACK.with(|cell| {
            let mut stack = cell.lock().unwrap();
            stack.0.push(tx_context_arc);
        })
    }

    pub fn static_pop() -> Arc<TxContext> {
        API_STACK.with(|cell| {
            let mut stack = cell.lock().unwrap();
            stack.0.pop().unwrap()
        })
    }

    /// Manages the stack.
    ///
    /// Pushes the context to the stack, executes closure, pops after.
    pub fn execute_on_vm_stack<F, R>(tx_context_sh: &mut Shareable<TxContext>, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        tx_context_sh.with_shared(|tx_context_arc| {
            TxContextStack::static_push(tx_context_arc);

            let result = f();

            let _ = TxContextStack::static_pop();

            result
        })
    }
}
