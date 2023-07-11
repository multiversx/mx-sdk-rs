use std::{ops::Deref, rc::Rc};

use multiversx_chain_vm_executor::Executor;

use super::BuiltinFunctionContainer;

pub struct BlockchainVM {
    pub builtin_functions: BuiltinFunctionContainer,
    pub executor: Box<dyn Executor>,
}

#[derive(Clone)]
pub struct BlockchainVMRef(Rc<BlockchainVM>);

impl BlockchainVM {
    pub fn new(executor: Box<dyn Executor>) -> Self {
        BlockchainVM {
            builtin_functions: BuiltinFunctionContainer,
            executor,
        }
    }
}

impl BlockchainVMRef {
    pub fn new(executor: Box<dyn Executor>) -> Self {
        BlockchainVMRef(Rc::new(BlockchainVM::new(executor)))
    }
}

impl Deref for BlockchainVMRef {
    type Target = BlockchainVM;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
