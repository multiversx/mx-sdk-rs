use std::{cell::RefCell, rc::Rc};

use multiversx_sc::types::LockableStaticBuffer;

use super::TxStaticVars;

#[derive(Debug, Default)]
pub struct StaticVarData {
    pub lockable_static_buffer_cell: RefCell<LockableStaticBuffer>,
    pub static_vars_cell: RefCell<TxStaticVars>,
}

#[derive(Debug, Default)]
pub struct StaticVarStack(Vec<Rc<StaticVarData>>);

thread_local!(
    static STATIC_STACK: RefCell<StaticVarStack> = RefCell::new(StaticVarStack::default())
);

impl StaticVarStack {
    pub fn static_peek() -> Rc<StaticVarData> {
        STATIC_STACK.with(|cell| {
            let stack = cell.borrow();
            stack.0.last().unwrap().clone()
        })
    }

    pub fn static_push() {
        STATIC_STACK.with(|cell| {
            let mut stack = cell.borrow_mut();
            stack.0.push(Rc::default());
        })
    }

    pub fn static_pop() -> Rc<StaticVarData> {
        STATIC_STACK.with(|cell| {
            let mut stack = cell.borrow_mut();
            stack.0.pop().unwrap()
        })
    }
}
