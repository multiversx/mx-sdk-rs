use std::{fmt::Debug, ops::Deref};

use super::{state::BlockchainStateRef, BlockchainVMRef};

#[derive(Default)]
pub struct BlockchainMock {
    pub vm: BlockchainVMRef,
    pub state: BlockchainStateRef,
}

impl Debug for BlockchainMock {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("BlockchainMock")
            .field("state", self.state.deref())
            .finish()
    }
}
