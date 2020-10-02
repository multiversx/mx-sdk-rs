


use elrond_wasm::{H256, Address};

use crate::big_int_mock::*;
use crate::big_uint_mock::*;
use crate::contract_map::*;
use crate::ext_mock::*;
use crate::display_util::*;

use elrond_wasm::ContractHookApi;
use elrond_wasm::CallableContract;
use elrond_wasm::BigUintApi;
use elrond_wasm::err_msg;

use num_bigint::{BigInt, BigUint};
use num_traits::cast::ToPrimitive;

use alloc::boxed::Box;
use alloc::vec::Vec;

use std::collections::HashMap;
use std::fmt;
use std::fmt::Write;

use core::cell::RefCell;
use alloc::rc::Rc;

const ADDRESS_LENGTH: usize = 32;
const KEY_LENGTH: usize = 32;
const TOPIC_LENGTH: usize = 32;

pub struct AccountData {
    pub address: Address,
    pub nonce: u64,
    pub balance: BigUint,
    pub storage: HashMap<Vec<u8>, Vec<u8>>,
    pub contract_path: Option<Vec<u8>>,
    pub contract_owner: Option<Address>,
}

impl fmt::Display for AccountData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut storage_buf = String::new();
        let mut keys: Vec<Vec<u8>> = self.storage.iter().map(|(k, _)| k.clone()).collect();
        keys.sort();
        for key in &keys {
            let value = self.storage.get(key).unwrap();
            write!(&mut storage_buf, "\n\t\t{} -> 0x{}", key_hex(key.as_slice()), hex::encode(value.as_slice())).unwrap();
        }
        
        write!(f, "AccountData {{ nonce: {}, balance: {}, storage: [{} ] }}",
            self.nonce, 
            self.balance,
            storage_buf)
    }
}

pub struct BlockchainMock {
    pub accounts: HashMap<Address, AccountData>,
    pub new_addresses: HashMap<(Address, u64), Address>,
}

impl BlockchainMock {
    pub fn new() -> Self {
        BlockchainMock {
            accounts: HashMap::new(),
            new_addresses: HashMap::new(),
        }
    }

    pub fn add_account(&mut self, acct: AccountData) {
        self.accounts.insert(acct.address.clone(), acct);
    }

    pub fn print_accounts(&self) {
        let mut accounts_buf = String::new();
        for (address, account) in &self.accounts {
            write!(&mut accounts_buf, "\n\t{} -> {}", address_hex(address), account).unwrap();
        }
        println!("Accounts: {}", &accounts_buf);
    }

    pub fn put_new_address(&mut self, creator_address: Address, creator_nonce: u64, new_address: Address) {
        self.new_addresses.insert((creator_address, creator_nonce), new_address);
    }

    fn get_new_address(&self, creator_address: Address, creator_nonce: u64) -> Option<Address> {
        self.new_addresses
            .get(&(creator_address, creator_nonce))
            .map(|addr_ref| addr_ref.clone())
    }

    pub fn get_contract_path(&self, contract_address: &Address) -> Vec<u8> {
        if let Some(account) = self.accounts.get(&contract_address) {
            if let Some(contract_path) = &account.contract_path {
                contract_path.clone()
            } else {
                panic!("Recipient account is not a smart contract");
            }
        } else {
            panic!("Account not found");
        }
    }

    #[allow(mutable_borrow_reservation_conflict)] // TODO: refactor
    pub fn create_account_after_deploy(&mut self, tx: &TxInput, contract_path: Vec<u8>) {
        if let Some(sender) = self.accounts.get(&tx.from) {
            if let Some(new_address) = self.get_new_address(tx.from.clone(), sender.nonce) {
                let old_value = self.accounts.insert(new_address.clone(), AccountData{
                    address: new_address.clone(),
                    nonce: 0,
                    balance: 0u32.into(),
                    storage: HashMap::new(),
                    contract_path: Some(contract_path),
                    contract_owner: Some(sender.address.clone()),
                });
                if old_value.is_some() {
                    panic!("Account already exists at deploy address.");
                }
            } else {
                panic!("Missing new address. Only explicit new deploy addresses supported.");
            }
        } else {
            panic!("Unknown deployer");
        }
        
    }
}

pub fn execute_tx(
    tx_context: TxContext,
    contract_identifier: &Vec<u8>,
    contract_map: &ContractMap<TxContext>) -> TxOutput {

    let func_name = tx_context.tx_input.func_name.clone();
    let contract_inst = contract_map.new_contract_instance(contract_identifier, tx_context);
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        contract_inst.call(func_name.as_slice());
        let context = contract_inst.into_api();
        context.into_output()
    }));
    match result {
        Ok(tx_result) => tx_result,
        Err(panic_any) => panic_result(panic_any),
    }
}

fn panic_result(panic_any: Box<dyn std::any::Any + std::marker::Send>) -> TxOutput {
    if let Some(panic_obj) = panic_any.downcast_ref::<TxPanic>() {
        return TxOutput::from_panic_obj(panic_obj);
    }

    if let Some(panic_string) = panic_any.downcast_ref::<String>() {
        return TxOutput::from_panic_string(panic_string.as_str());
    }

    panic!("panic happened: unknown type.")
}
