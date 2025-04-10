use std::{fmt::Debug, ops::Deref};

use crate::schedule::GasSchedule;

use super::{state::BlockchainStateRef, VMConfigRef};

#[derive(Default)]
pub struct BlockchainMock {
    pub vm: VMConfigRef,
    pub state: BlockchainStateRef,
}

impl BlockchainMock {
    pub fn new_with_gas(gas_schedule: GasSchedule) -> Self {
        Self {
            vm: VMConfigRef::new_with_gas(gas_schedule),
            ..Default::default()
        }
    }
}

impl Debug for BlockchainMock {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("BlockchainMock")
            .field("state", self.state.deref())
            .finish()
    }
}
