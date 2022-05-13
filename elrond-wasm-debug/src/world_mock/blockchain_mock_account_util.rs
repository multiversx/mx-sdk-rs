use alloc::vec::Vec;
use elrond_wasm::types::heap::Address;

use std::{collections::HashMap, fmt::Write};

use crate::address_hex;

use super::AccountData;

const SC_ADDRESS_NUM_LEADING_ZEROS: u8 = 8;
const UNDERSCORE: u8 = b'_';
static ADDR_PREFIX: &str = "address:";
static SC_ADDR_PREFIX: &str = "sc:";
static HEX_PREFIX: &str = "0x";

use super::BlockchainMock;

impl BlockchainMock {
    pub fn add_account(&mut self, acct: AccountData) {
        let address = acct.address.clone();
        self.accounts.insert(address.clone(), acct);
        self.add_addr_mandos_string(address);
    }

    pub fn add_addr_mandos_string(&mut self, address: Address) {
        if self.addr_to_mandos_string_map.contains_key(&address) {
            return;
        }

        let addr_pretty = address_as_mandos_string(&address);
        self.addr_to_mandos_string_map.insert(address, addr_pretty);
    }

    pub fn validate_and_add_account(&mut self, acct: AccountData) {
        self.validate_account(&acct);
        self.add_account(acct);
    }

    pub fn update_accounts(&mut self, accounts: HashMap<Address, AccountData>) {
        for addr in accounts.keys() {
            self.add_addr_mandos_string(addr.clone());
        }

        self.accounts.extend(accounts.into_iter());
    }

    pub fn print_accounts(&self) {
        let mut accounts_buf = String::new();
        for (address, account) in &self.accounts {
            write!(accounts_buf, "\n\t{} -> {}", address_hex(address), account).unwrap();
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

fn address_as_mandos_string(address: &Address) -> String {
    let addr_bytes = address.as_array();
    let (string_start_index, prefix) = if is_smart_contract_address(address) {
        (SC_ADDRESS_NUM_LEADING_ZEROS as usize, SC_ADDR_PREFIX)
    } else {
        (0, ADDR_PREFIX)
    };

    let mut string_end_index = Address::len_bytes() - 1;
    while addr_bytes[string_end_index] == UNDERSCORE {
        string_end_index -= 1;
    }

    let addr_readable_part = &addr_bytes[string_start_index..=string_end_index];
    match String::from_utf8(addr_readable_part.to_vec()) {
        Ok(readable_string) => {
            let mut result = prefix.to_string();
            result.push_str(&readable_string);

            result
        },
        Err(_) => {
            let mut result = HEX_PREFIX.to_string();
            result.push_str(&hex::encode(&addr_bytes[..]));

            result
        },
    }
}
