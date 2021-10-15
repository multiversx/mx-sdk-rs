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
    blockchain_cell: Rc<RefCell<BlockchainMock>>,
    accounts: HashMap<Address, AccountData>,
}

impl TxCache {
    pub fn new(blockchain_cell: Rc<RefCell<BlockchainMock>>) -> Self {
        TxCache {
            blockchain_cell,
            accounts: HashMap::new(),
        }
    }

    pub fn blockchain_ref(&self) -> Ref<BlockchainMock> {
        self.blockchain_cell.borrow()
    }

    fn load_account_if_necessary(&mut self, address: &Address) {
        if !self.accounts.contains_key(address) {
            let blockchain_ref = self.blockchain_cell.borrow();
            if let Some(blockchain_account) = blockchain_ref.accounts.get(address) {
                self.accounts
                    .insert(address.clone(), blockchain_account.clone());
            }
        }
    }

    pub fn get_account(&self, address: &Address) -> &AccountData {
        self.load_account_if_necessary(address);
        self.accounts
            .get(address)
            .unwrap_or_else(|| panic!("Account {} not found", address_hex(address)))
    }

    pub fn get_account_mut(&self, address: &Address) -> &mut AccountData {
        self.load_account_if_necessary(address);
        self.accounts
            .get_mut(address)
            .unwrap_or_else(|| panic!("Account {} not found", address_hex(address)))
    }

    pub fn insert_account(&self, address: Address, account_data: AccountData) {
        self.accounts.insert(address, account_data);
    }

    pub fn increase_acount_nonce(&self, address: &Address) {
        let account = self.get_account_mut(address);
        account.nonce += 1;
    }

    pub fn commit(self) {
        let mut blockchain_ref = self.blockchain_cell.borrow_mut();
        blockchain_ref.accounts.extend(self.accounts.into_iter());
    }

    pub fn discard(self) {}
}
