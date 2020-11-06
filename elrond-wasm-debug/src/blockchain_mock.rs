use elrond_wasm::{Address, H256};

use crate::big_int_mock::*;
use crate::big_uint_mock::*;
use crate::contract_map::*;
use crate::display_util::*;
use crate::ext_mock::*;

use elrond_wasm::err_msg;
use elrond_wasm::BigUintApi;
use elrond_wasm::CallableContract;
use elrond_wasm::ContractHookApi;

use num_bigint::{BigInt, BigUint};
use num_traits::{cast::ToPrimitive, Zero};

use alloc::boxed::Box;
use alloc::vec::Vec;

use std::collections::HashMap;
use std::fmt;
use std::fmt::Write;

use alloc::rc::Rc;
use core::cell::RefCell;

const ELROND_REWARD_KEY: &[u8] = b"ELRONDreward";

pub struct AccountData {
    pub address: Address,
    pub nonce: u64,
    pub balance: BigUint,
    pub storage: HashMap<Vec<u8>, Vec<u8>>,
    pub esdt: Option<HashMap<Vec<u8>, BigUint>>,
    pub contract_path: Option<Vec<u8>>,
    pub contract_owner: Option<Address>,
}

impl fmt::Display for AccountData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut storage_buf = String::new();
        let mut storage_keys: Vec<Vec<u8>> = self.storage.iter().map(|(k, _)| k.clone()).collect();
        storage_keys.sort();

        for key in &storage_keys {
            let value = self.storage.get(key).unwrap();
            write!(&mut storage_buf, "\n\t\t{} -> 0x{}", key_hex(key.as_slice()), hex::encode(value.as_slice())).unwrap();
        }

        let mut esdt_buf = String::new();
        let esdt_unwrapped = self.esdt.clone().unwrap_or_default();
        let mut esdt_keys: Vec<Vec<u8>> = esdt_unwrapped.iter().map(|(k, _)| k.clone()).collect();
        esdt_keys.sort();

        for key in &esdt_keys {
            let value = esdt_unwrapped.get(key).unwrap();
            write!(&mut esdt_buf, "\n\t\t{} -> 0x{}", key_hex(key.as_slice()), hex::encode(value.to_bytes_be())).unwrap();
        }
        
        write!(f, "AccountData {{ nonce: {}, balance: {}, storage: [{} ], esdt: [{} ] }}",
            self.nonce, 
            self.balance,
            storage_buf,
            esdt_buf)
    }
}

#[derive(Clone, Debug)]
pub struct BlockInfo {
	pub block_timestamp: u64,
	pub block_nonce: u64,
	pub block_round: u64,
	pub block_epoch: u64,
}

impl BlockInfo {
	pub fn new() -> Self {
		BlockInfo {
			block_timestamp: 0,
			block_nonce: 0,
			block_round: 0,
			block_epoch: 0,
		}
	}
}

pub struct BlockchainMock {
	pub accounts: HashMap<Address, AccountData>,
	pub new_addresses: HashMap<(Address, u64), Address>,
	pub previous_block_info: BlockInfo,
	pub current_block_info: BlockInfo,
}

impl BlockchainMock {
    pub fn new() -> Self {
        BlockchainMock {
            accounts: HashMap::new(),
            new_addresses: HashMap::new(),
            previous_block_info: BlockInfo::new(),
            current_block_info: BlockInfo::new(),
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

    pub fn subtract_tx_payment(&mut self, address: &Address, call_value: &BigUint) {
        let sender_account = self.accounts
            .get_mut(address)
            .unwrap_or_else(|| panic!("Sender account not found"));
        assert!(&sender_account.balance >= call_value, "Not enough balance to send tx payment");
        sender_account.balance -= call_value;
    }

    pub fn subtract_tx_gas(&mut self, address: &Address, gas_limit: u64, gas_price: u64) {
        let sender_account = self.accounts
            .get_mut(address)
            .unwrap_or_else(|| panic!("Sender account not found"));
        let gas_cost = BigUint::from(gas_limit) * BigUint::from(gas_price);
        assert!(sender_account.balance >= gas_cost, "Not enough balance to pay gas upfront");
        sender_account.balance -= &gas_cost;
    }

    pub fn increase_balance(&mut self, address: &Address, amount: &BigUint) {
        let account = self.accounts
            .get_mut(address)
            .unwrap_or_else(|| panic!("Receiver account not found"));
        account.balance += amount;
    }

    pub fn send_balance(&mut self, contract_address: &Address, send_balance_list: &[SendBalance]) {
        for send_balance in send_balance_list {
            self.subtract_tx_payment(contract_address, &send_balance.amount);
            self.increase_balance(&send_balance.recipient, &send_balance.amount);
        }
    }

    pub fn substract_esdt_balance(&mut self, address: &Address, esdt_token_name: &[u8], value: &BigUint) {
        let sender_account = self.accounts
            .get_mut(address)
            .unwrap_or_else(|| panic!("Sender account not found"));
        
        let esdt =  sender_account.esdt.as_mut()
            .unwrap_or_else(|| panic!("Account has no esdt tokens"));
        
        let esdt_balance = esdt.get_mut(esdt_token_name)
            .unwrap_or_else(|| panic!("Account has no esdt tokens with that name"));

        assert!(*esdt_balance >= *value, "Not enough esdt balance");
        *esdt_balance -= value;
    }

    pub fn increase_esdt_balance(&mut self, address: &Address, esdt_token_name: &[u8], value: &BigUint) {
        let account = self.accounts
            .get_mut(address)
            .unwrap_or_else(|| panic!("Receiver account not found"));

        if account.esdt.is_none() {
            let mut new_esdt = HashMap::<Vec<u8>, BigUint>::new();
            new_esdt.insert(esdt_token_name.to_vec(), value.clone());

            account.esdt = Some(new_esdt);
        }
        else {
            let esdt =  account.esdt.as_mut().unwrap();

            if esdt.contains_key(esdt_token_name) {
                let esdt_balance = esdt.get_mut(esdt_token_name).unwrap();
                *esdt_balance += value;
            }
            else {
                esdt.insert(esdt_token_name.to_vec(), value.clone());
            }
        }
    }

    pub fn increase_nonce(&mut self, address: &Address) {
        let account = self.accounts
            .get_mut(address)
            .unwrap_or_else(|| panic!("Account not found"));
        account.nonce += 1;
    }

    pub fn create_account_after_deploy(&mut self,
        tx_input: &TxInput,
        new_storage: HashMap<Vec<u8>, Vec<u8>>,
        contract_path: Vec<u8>) -> Address {

        let sender = self.accounts.get(&tx_input.from)
            .unwrap_or_else(|| panic!("Unknown deployer"));
        let sender_nonce_before_tx = sender.nonce - 1;
        let new_address = self.get_new_address(tx_input.from.clone(), sender_nonce_before_tx)
            .unwrap_or_else(|| panic!("Missing new address. Only explicit new deploy addresses supported"));
        let mut esdt = HashMap::<Vec<u8>, BigUint>::new();
        let mut esdt_opt: Option<HashMap<Vec<u8>, BigUint>> = None;
        
        if tx_input.esdt_token_name.is_some()
        {
            esdt.insert(tx_input.esdt_token_name.clone().unwrap(), tx_input.esdt_value.clone());
            esdt_opt = Some(esdt);
        }

        let old_value = self.accounts.insert(new_address.clone(), AccountData{
            address: new_address.clone(),
            nonce: 0,
            balance: tx_input.call_value.clone(),
            storage: new_storage,
            esdt: esdt_opt,
            contract_path: Some(contract_path),
            contract_owner: Some(tx_input.from.clone()),
        });
        if old_value.is_some() {
            panic!("Account already exists at deploy address.");
        }

        new_address
    }

    pub fn increase_validator_reward(&mut self, address: &Address, amount: &BigUint) {
        let account = self.accounts
            .get_mut(address)
            .unwrap_or_else(|| panic!("Account not found"));
        account.balance += amount;
        let mut storage_v_rew = if let Some(old_storage_value) = account.storage.get(ELROND_REWARD_KEY){
            BigUint::from_bytes_be(old_storage_value)
        } else {
            BigUint::zero()
        };
        storage_v_rew += amount;
        account.storage.insert(ELROND_REWARD_KEY.to_vec(), storage_v_rew.to_bytes_be());
    }
}

pub fn execute_tx(
	tx_context: TxContext,
	contract_identifier: &Vec<u8>,
	contract_map: &ContractMap<TxContext>,
) -> TxOutput {
	let func_name = tx_context.tx_input_box.func_name.clone();
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

/// Some data to get copied for the tx.
/// Would be nice maybe at some point to have a reference to the full blockchain mock in the tx context,
/// but for now, copying some data is enough.
#[derive(Clone, Debug)]
pub struct BlockchainTxInfo {
	pub previous_block_info: BlockInfo,
	pub current_block_info: BlockInfo,
	pub contract_balance: BigUint,
	pub contract_owner: Option<Address>,
}

impl BlockchainMock {
	pub fn create_tx_info(&self, contract_address: &Address) -> BlockchainTxInfo {
		if let Some(contract) = self.accounts.get(contract_address) {
			BlockchainTxInfo {
				previous_block_info: self.previous_block_info.clone(),
				current_block_info: self.current_block_info.clone(),
				contract_balance: contract.balance.clone(),
				contract_owner: contract.contract_owner.clone(),
			}
		} else {
			BlockchainTxInfo {
				previous_block_info: self.previous_block_info.clone(),
				current_block_info: self.current_block_info.clone(),
				contract_balance: 0u32.into(),
				contract_owner: None,
			}
		}
	}
}
