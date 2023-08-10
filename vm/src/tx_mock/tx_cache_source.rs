use crate::{
    types::VMAddress,
    world_mock::{AccountData, BlockchainState},
};

use super::TxCache;

pub trait TxCacheSource: Send + Sync {
    fn load_account(&self, address: &VMAddress) -> Option<AccountData>;

    fn blockchain_ref(&self) -> &BlockchainState;
}

impl TxCacheSource for TxCache {
    fn load_account(&self, address: &VMAddress) -> Option<AccountData> {
        Some(self.with_account(address, AccountData::clone))
    }

    fn blockchain_ref(&self) -> &BlockchainState {
        self.blockchain_ref()
    }
}

impl TxCacheSource for BlockchainState {
    fn load_account(&self, address: &VMAddress) -> Option<AccountData> {
        self.accounts.get(address).map(AccountData::clone)
    }

    fn blockchain_ref(&self) -> &BlockchainState {
        self
    }
}
