use multiversx_sc::types::heap::Address;

use crate::world_mock::{AccountData, BlockchainMock};

use super::TxCache;

pub trait TxCacheSource {
    fn load_account(&self, address: &Address) -> Option<AccountData>;

    fn blockchain_ref(&self) -> &BlockchainMock;
}

impl TxCacheSource for TxCache {
    fn load_account(&self, address: &Address) -> Option<AccountData> {
        Some(self.with_account(address, AccountData::clone))
    }

    fn blockchain_ref(&self) -> &BlockchainMock {
        self.blockchain_ref()
    }
}

impl TxCacheSource for BlockchainMock {
    fn load_account(&self, address: &Address) -> Option<AccountData> {
        self.accounts.get(address).map(AccountData::clone)
    }

    fn blockchain_ref(&self) -> &BlockchainMock {
        self
    }
}
