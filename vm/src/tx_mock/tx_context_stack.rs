use crate::with_shared::Shareable;

use super::TxContext;

use std::{cell::RefCell, rc::Rc};

thread_local!(
    static API_STACK: RefCell<TxContextStack> = RefCell::new(TxContextStack::default())
);

#[derive(Debug, Default)]
pub struct TxContextStack(Vec<Rc<TxContext>>);

impl TxContextStack {
    pub fn static_peek() -> Rc<TxContext> {
        API_STACK.with(|cell| {
            let stack = cell.borrow();
            stack.0.last().unwrap().clone()
        })
    }

    pub fn static_push(tx_context_rc: Rc<TxContext>) {
        API_STACK.with(|cell| {
            let mut stack = cell.borrow_mut();
            stack.0.push(tx_context_rc);
        })
    }

    pub fn static_pop() -> Rc<TxContext> {
        API_STACK.with(|cell| {
            let mut stack = cell.borrow_mut();
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
        tx_context_sh.with_shared(|tx_context_rc| {
            TxContextStack::static_push(tx_context_rc);

            let result = f();

            let _ = TxContextStack::static_pop();

            result
        })
    }
}
