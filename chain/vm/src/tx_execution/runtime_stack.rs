use crate::tx_mock::TxContextRef;

pub struct StackItem {
    pub tx_context_ref: TxContextRef,
}

#[derive(Default)]
pub struct Stack(Vec<StackItem>);

impl Stack {
    pub fn peek(&self) -> Option<&StackItem> {
        self.0.last()
    }

    pub fn push(&mut self, item: StackItem) {
        self.0.push(item);
    }

    pub fn pop(&mut self) -> StackItem {
        self.0.pop().expect("cannot pop from empty execution stack")
    }
}
