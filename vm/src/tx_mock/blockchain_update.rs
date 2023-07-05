use std::collections::HashMap;

use crate::{
    types::VMAddress,
    world_mock::{AccountData, BlockchainState},
};

pub struct BlockchainUpdate {
    pub accounts: HashMap<VMAddress, AccountData>,
}

impl BlockchainUpdate {
    pub fn empty() -> Self {
        BlockchainUpdate {
            accounts: HashMap::new(),
        }
    }

    pub fn apply(self, blockchain: &mut BlockchainState) {
        blockchain.update_accounts(self.accounts);
    }
}
