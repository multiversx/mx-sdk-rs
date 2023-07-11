use std::collections::HashMap;

use crate::{
    types::VMAddress,
    world_mock::{AccountData, BlockchainState},
};

#[derive(Default)]
pub struct BlockchainUpdate {
    pub accounts: HashMap<VMAddress, AccountData>,
    pub new_token_identifiers: Option<Vec<String>>,
}

impl BlockchainUpdate {
    pub fn empty() -> Self {
        BlockchainUpdate::default()
    }

    pub fn apply(self, blockchain: &mut BlockchainState) {
        blockchain.update_accounts(self.accounts);

        if let Some(token_identifiers) = self.new_token_identifiers {
            blockchain.update_new_token_identifiers(token_identifiers);
        }
    }
}
