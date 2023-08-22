use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, Mutex},
};

use crate::{
    display_util::address_hex,
    types::VMAddress,
    world_mock::{AccountData, BlockchainState},
};

use super::{BlockchainUpdate, TxCacheSource};

pub struct TxCache {
    source_ref: Arc<dyn TxCacheSource>,
    pub(super) accounts: Mutex<HashMap<VMAddress, AccountData>>,
    pub(super) new_token_identifiers: Mutex<Option<Vec<String>>>,
}

impl fmt::Debug for TxCache {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TxCache")
            .field("accounts", &self.accounts)
            .finish()
    }
}

impl TxCache {
    pub fn new(source_ref: Arc<dyn TxCacheSource>) -> Self {
        TxCache {
            source_ref,
            accounts: Mutex::new(HashMap::new()),
            new_token_identifiers: Mutex::new(None),
        }
    }

    pub fn blockchain_ref(&self) -> &BlockchainState {
        self.source_ref.blockchain_ref()
    }

    fn load_account_if_necessary(&self, address: &VMAddress) {
        let mut accounts_mut = self.accounts.lock().unwrap();
        if !accounts_mut.contains_key(address) {
            if let Some(blockchain_account) = self.source_ref.load_account(address) {
                accounts_mut.insert(address.clone(), blockchain_account);
            }
        }
    }

    pub fn with_account<R, F>(&self, address: &VMAddress, f: F) -> R
    where
        F: FnOnce(&AccountData) -> R,
    {
        self.load_account_if_necessary(address);
        let accounts = self.accounts.lock().unwrap();
        let account = accounts
            .get(address)
            .unwrap_or_else(|| panic!("Account {} not found", address_hex(address)));
        f(account)
    }

    pub fn with_account_mut<R, F>(&self, address: &VMAddress, f: F) -> R
    where
        F: FnOnce(&mut AccountData) -> R,
    {
        self.load_account_if_necessary(address);
        let mut accounts = self.accounts.lock().unwrap();
        let account = accounts
            .get_mut(address)
            .unwrap_or_else(|| panic!("Account {} not found", address_hex(address)));
        f(account)
    }

    pub fn insert_account(&self, account_data: AccountData) {
        self.accounts
            .lock()
            .unwrap()
            .insert(account_data.address.clone(), account_data);
    }

    pub fn increase_acount_nonce(&self, address: &VMAddress) {
        self.with_account_mut(address, |account| {
            account.nonce += 1;
        });
    }

    /// Assumes the nonce has already been increased.
    pub fn get_new_address(&self, creator_address: &VMAddress) -> VMAddress {
        let current_nonce = self.with_account(creator_address, |account| account.nonce);
        self.blockchain_ref()
            .get_new_address(creator_address.clone(), current_nonce - 1)
            .unwrap_or_else(|| {
                panic!("Missing new address. Only explicit new deploy addresses supported")
            })
    }

    pub fn get_new_token_identifiers(&self) -> Vec<String> {
        self.blockchain_ref().get_new_token_identifiers()
    }

    pub fn set_new_token_identifiers(&self, token_identifiers: Vec<String>) {
        *self.new_token_identifiers.lock().unwrap() = Some(token_identifiers);
    }

    pub fn into_blockchain_updates(self) -> BlockchainUpdate {
        BlockchainUpdate {
            accounts: self.accounts.into_inner().unwrap(),
            new_token_identifiers: self.new_token_identifiers.into_inner().unwrap(),
        }
    }

    pub fn commit_updates(&self, updates: BlockchainUpdate) {
        self.accounts.lock().unwrap().extend(updates.accounts);
    }
}
