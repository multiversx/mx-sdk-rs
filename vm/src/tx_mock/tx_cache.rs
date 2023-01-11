use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    fmt,
    rc::Rc,
};

use multiversx_sc::types::heap::Address;

use crate::{
    address_hex,
    world_mock::{AccountData, BlockchainMock},
};

use super::TxCacheSource;

pub struct TxCache {
    source_ref: Rc<dyn TxCacheSource>,
    pub(super) accounts: RefCell<HashMap<Address, AccountData>>,
}

impl fmt::Debug for TxCache {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TxCache")
            .field("accounts", &self.accounts)
            .finish()
    }
}

impl TxCache {
    pub fn new(source_ref: Rc<dyn TxCacheSource>) -> Self {
        TxCache {
            source_ref,
            accounts: RefCell::new(HashMap::new()),
        }
    }

    pub fn blockchain_ref(&self) -> &BlockchainMock {
        self.source_ref.blockchain_ref()
    }

    fn load_account_if_necessary(&self, address: &Address) {
        let mut accounts_mut = self.accounts.borrow_mut();
        if !accounts_mut.contains_key(address) {
            if let Some(blockchain_account) = self.source_ref.load_account(address) {
                accounts_mut.insert(address.clone(), blockchain_account);
            }
        }
    }

    pub fn with_account<R, F>(&self, address: &Address, f: F) -> R
    where
        F: FnOnce(&AccountData) -> R,
    {
        self.load_account_if_necessary(address);
        let accounts = self.accounts.borrow();
        let account = accounts
            .get(address)
            .unwrap_or_else(|| panic!("Account {} not found", address_hex(address)));
        f(account)
    }

    pub fn with_account_mut<R, F>(&self, address: &Address, f: F) -> R
    where
        F: FnOnce(&mut AccountData) -> R,
    {
        self.load_account_if_necessary(address);
        let mut accounts = self.accounts.borrow_mut();
        let account = accounts
            .get_mut(address)
            .unwrap_or_else(|| panic!("Account {} not found", address_hex(address)));
        f(account)
    }

    pub fn insert_account(&self, account_data: AccountData) {
        self.accounts
            .borrow_mut()
            .insert(account_data.address.clone(), account_data);
    }

    pub fn increase_acount_nonce(&self, address: &Address) {
        self.with_account_mut(address, |account| {
            account.nonce += 1;
        });
    }

    pub fn get_all_accounts(&self) -> Ref<HashMap<Address, AccountData>> {
        self.accounts.borrow()
    }

    /// Assumes the nonce has already been increased.
    pub fn get_new_address(&self, creator_address: &Address) -> Address {
        let current_nonce = self.with_account(creator_address, |account| account.nonce);
        self.blockchain_ref()
            .get_new_address(creator_address.clone(), current_nonce - 1)
            .unwrap_or_else(|| {
                panic!("Missing new address. Only explicit new deploy addresses supported")
            })
    }

    pub fn into_blockchain_updates(self) -> BlockchainUpdate {
        BlockchainUpdate {
            accounts: self.accounts.into_inner(),
        }
    }

    pub fn commit_updates(&self, updates: BlockchainUpdate) {
        self.accounts
            .borrow_mut()
            .extend(updates.accounts.into_iter());
    }
}

pub struct BlockchainUpdate {
    accounts: HashMap<Address, AccountData>,
}

impl BlockchainUpdate {
    pub fn empty() -> Self {
        BlockchainUpdate {
            accounts: HashMap::new(),
        }
    }

    pub fn apply(self, blockchain: &mut BlockchainMock) {
        blockchain.update_accounts(self.accounts);
    }
}
