use std::{collections::HashMap, fmt::Write};

use crate::{display_util::address_hex, types::VMAddress};

use super::{AccountData, BlockchainState};

impl BlockchainState {
    pub fn add_account(&mut self, acct: AccountData) {
        let address = acct.address.clone();
        self.accounts.insert(address, acct);
    }

    pub fn validate_and_add_account(&mut self, acct: AccountData) {
        self.validate_account(&acct);
        self.add_account(acct);
    }

    pub fn update_accounts(&mut self, accounts: HashMap<VMAddress, AccountData>) {
        self.accounts.extend(accounts);
    }

    pub fn print_accounts(&self) {
        let mut accounts_buf = String::new();
        for (address, account) in &self.accounts {
            write!(accounts_buf, "\n\t{} -> {account}", address_hex(address)).unwrap();
        }
        println!("Accounts: {}", &accounts_buf);
    }

    pub fn put_new_address(
        &mut self,
        creator_address: VMAddress,
        creator_nonce: u64,
        new_address: VMAddress,
    ) {
        self.new_addresses
            .insert((creator_address, creator_nonce), new_address);
    }

    pub fn get_new_address(
        &self,
        creator_address: VMAddress,
        creator_nonce: u64,
    ) -> Option<VMAddress> {
        self.new_addresses
            .get(&(creator_address, creator_nonce))
            .cloned()
    }

    pub fn validate_account(&self, account: &AccountData) {
        let is_sc = account.address.is_smart_contract_address();
        let has_code = self.check_account_has_code(account);

        assert!(
            !is_sc || has_code,
            "Account has a smart contract address but no code"
        );

        assert!(
            is_sc || !has_code,
            "Account has no smart contract address but has code"
        );
    }

    pub fn check_account_has_code(&self, account: &AccountData) -> bool {
        !account
            .contract_path
            .as_ref()
            .unwrap_or(&Vec::<u8>::new())
            .is_empty()
    }
}
