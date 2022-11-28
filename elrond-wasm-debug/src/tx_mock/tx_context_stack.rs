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
}
