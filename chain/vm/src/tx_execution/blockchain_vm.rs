use std::{ops::Deref, sync::Arc};

use super::BuiltinFunctionContainer;

#[derive(Default)]
pub struct BlockchainVM {
    pub builtin_functions: BuiltinFunctionContainer,
}

#[derive(Clone, Default)]
pub struct BlockchainVMRef(Arc<BlockchainVM>);

impl BlockchainVM {
    pub fn new() -> Self {
        BlockchainVM {
            builtin_functions: BuiltinFunctionContainer,
        }
    }
}

impl BlockchainVMRef {
    pub fn new() -> Self {
        BlockchainVMRef(Arc::new(BlockchainVM::new()))
    }
}

impl Deref for BlockchainVMRef {
    type Target = BlockchainVM;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
