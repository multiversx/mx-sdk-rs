use std::sync::Mutex;

use multiversx_chain_vm::tx_mock::TxContextRef;

thread_local!(
    static API_STACK: Mutex<TxContextStack> = Mutex::new(TxContextStack::default())
);

#[derive(Debug, Default)]
pub struct TxContextStack(Vec<TxContextRef>);

impl TxContextStack {
    pub fn static_peek() -> TxContextRef {
        API_STACK.with(|cell| {
            let stack = cell.lock().unwrap();
            stack.0.last().unwrap().clone()
        })
    }

    pub fn static_push(tx_context_arc: TxContextRef) {
        API_STACK.with(|cell| {
            let mut stack = cell.lock().unwrap();
            stack.0.push(tx_context_arc);
        })
    }

    pub fn static_pop() -> TxContextRef {
        API_STACK.with(|cell| {
            let mut stack = cell.lock().unwrap();
            stack.0.pop().unwrap()
        })
    }
}
