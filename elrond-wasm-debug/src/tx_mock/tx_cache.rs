use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    rc::Rc,
};

use elrond_wasm::types::Address;

use crate::{
    address_hex,
    world_mock::{AccountData, BlockchainMock},
};

#[derive(Debug)]
pub struct TxCache {
    blockchain_ref: Rc<BlockchainMock>,
    accounts: RefCell<HashMap<Address, AccountData>>,
}

impl TxCache {
    pub fn new(blockchain_ref: Rc<BlockchainMock>) -> Self {
        TxCache {
            blockchain_ref,
            accounts: RefCell::new(HashMap::new()),
        }
    }

    pub fn blockchain_ref(&self) -> &BlockchainMock {
        &*self.blockchain_ref
    }

    fn load_account_if_necessary(&self, address: &Address) {
        let mut accounts_mut = self.accounts.borrow_mut();
        if !accounts_mut.contains_key(address) {
            if let Some(blockchain_account) = self.blockchain_ref.accounts.get(address) {
                accounts_mut.insert(address.clone(), blockchain_account.clone());
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
        f(&account)
    }

    pub fn with_account_mut<R, F>(&self, address: &Address, f: F) -> R
    where
        F: FnOnce(&mut AccountData) -> R,
    {
        self.load_account_if_necessary(address);
        let mut accounts = self.accounts.borrow_mut();
        let mut account = accounts
            .get_mut(address)
            .unwrap_or_else(|| panic!("Account {} not found", address_hex(address)));
        f(&mut account)
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

    pub fn into_blockchain_updates(self) -> BlockchainUpdate {
        BlockchainUpdate {
            accounts: self.accounts.into_inner(),
        }
    }
}

pub struct BlockchainUpdate {
    accounts: HashMap<Address, AccountData>,
}

impl BlockchainUpdate {
    pub fn apply(self, blockchain: &mut BlockchainMock) {
        blockchain.accounts.extend(self.accounts.into_iter());
    }
}
