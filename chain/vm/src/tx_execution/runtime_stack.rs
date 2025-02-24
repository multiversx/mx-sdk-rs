use std::{rc::Rc, sync::Mutex};

use multiversx_chain_vm_executor::Instance;

use crate::tx_mock::TxContextRef;

thread_local!(
    static CURRENT_TX_CONTEXT: Mutex<Option<TxContextRef>> = Mutex::new(None)
);

fn set_current_tx_context(value: Option<TxContextRef>) {
    CURRENT_TX_CONTEXT.with(|cell| {
        let mut opt = cell.lock().unwrap();
        *opt = value;
    });
}

pub struct StackItem {
    pub instance_ref: Rc<Box<dyn Instance>>,
    pub tx_context_ref: TxContextRef,
}

impl StackItem {
    pub fn on_stack_top_enter(&self) {
        set_current_tx_context(Some(self.tx_context_ref.clone()));
        self.instance_ref.on_stack_top_enter();
    }

    pub fn on_stack_top_leave(&self) {
        set_current_tx_context(None);
        self.instance_ref.on_stack_top_leave();
    }
}

#[derive(Default)]
pub struct Stack(Vec<StackItem>);

impl Stack {
    pub fn peek(&self) -> Option<&StackItem> {
        self.0.last()
    }

    pub fn push(&mut self, item: StackItem) {
        // if let Some(top) = self.peek() {
        //     top.on_stack_top_leave();
        // }
        // item.on_stack_top_enter();
        self.0.push(item);
    }

    pub fn pop(&mut self) -> StackItem {
        self.0.pop().expect("cannot pop from empty execution stack")

        // let popped = self.0.pop().expect("cannot pop from empty execution stack");
        // popped.on_stack_top_leave();
        // if let Some(top) = self.peek() {
        //     top.on_stack_top_enter();
        // }
        // popped
    }
}
