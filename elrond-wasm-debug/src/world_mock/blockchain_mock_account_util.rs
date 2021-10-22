use alloc::vec::Vec;
use elrond_wasm::types::Address;

use std::fmt::Write;

use crate::address_hex;

use super::AccountData;

const SC_ADDRESS_NUM_LEADING_ZEROS: u8 = 8;

use super::BlockchainMock;

impl BlockchainMock {
    pub fn add_account(&mut self, acct: AccountData) {
        self.accounts.insert(acct.address.clone(), acct);
    }

    pub fn validate_and_add_account(&mut self, acct: AccountData) {
        self.validate_account(&acct);
        self.add_account(acct);
    }

    pub fn print_accounts(&self) {
        let mut accounts_buf = String::new();
        for (address, account) in &self.accounts {
            write!(
                &mut accounts_buf,
                "\n\t{} -> {}",
                address_hex(address),
                account
            )
            .unwrap();
        }
        println!("Accounts: {}", &accounts_buf);
    }

    pub fn put_new_address(
        &mut self,
        creator_address: Address,
        creator_nonce: u64,
        new_address: Address,
    ) {
        self.new_addresses
            .insert((creator_address, creator_nonce), new_address);
    }

    pub fn get_new_address(&self, creator_address: Address, creator_nonce: u64) -> Option<Address> {
        self.new_addresses
            .get(&(creator_address, creator_nonce))
            .cloned()
    }

    pub fn validate_account(&self, account: &AccountData) {
        let is_sc = is_smart_contract_address(&account.address);
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

pub fn is_smart_contract_address(address: &Address) -> bool {
    address
        .as_bytes()
        .iter()
        .take(SC_ADDRESS_NUM_LEADING_ZEROS.into())
        .all(|item| item == &0u8)
}
