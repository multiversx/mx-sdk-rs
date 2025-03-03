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
    pub fn execute_on_vm_stack<F>(tx_context: TxContext, f: F) -> TxContext
    where
        F: FnOnce(),
    {
        let tx_context_arc = Arc::new(tx_context);
        TxContextStack::static_push(tx_context_arc);

        f();

        let tx_context_arc = TxContextStack::static_pop();

        Arc::into_inner(tx_context_arc)
            .expect("cannot extract final TxContext from stack because of lingering references")
    }
}
