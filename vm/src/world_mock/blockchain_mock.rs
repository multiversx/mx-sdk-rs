use crate::tx_execution::BlockchainVMRef;
use multiversx_chain_vm_executor::Executor;
use std::fmt::Debug;

use super::{BlockchainState, FailingExecutor};

pub struct BlockchainMock {
    pub vm: BlockchainVMRef,
    pub state: BlockchainState,
}

impl BlockchainMock {
    pub fn new(executor: Box<dyn Executor>) -> Self {
        BlockchainMock {
            vm: BlockchainVMRef::new(executor),
            state: BlockchainState::default(),
        }
    }
}

impl Default for BlockchainMock {
    fn default() -> Self {
        Self::new(Box::new(FailingExecutor))
    }
}

impl BlockchainMock {
    pub fn with_borrowed<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(BlockchainVMRef, BlockchainState) -> (R, BlockchainState),
    {
        let obj = std::mem::take(&mut self.state);
        let (result, obj) = f(self.vm.clone(), obj);
        self.state = obj;
        result
    }
}

impl Debug for BlockchainMock {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("BlockchainMock")
            .field("state", &self.state)
            .finish()
    }
}
