

use elrond_wasm::{H256, Address};

use crate::big_int_mock::*;
use crate::big_uint_mock::*;

use elrond_wasm::ContractHookApi;
use elrond_wasm::CallableContract;
use elrond_wasm::ContractFactory;
use elrond_wasm::BigUintApi;
use elrond_wasm::err_msg;

use num_bigint::{BigInt, BigUint};
use num_traits::cast::ToPrimitive;

use alloc::boxed::Box;
use alloc::vec::Vec;

use std::collections::HashMap;
use std::fmt;
use std::fmt::Write;

// use core::cell::RefCell;
// use alloc::rc::Rc;

use sha3::{Sha3_256, Keccak256, Digest};

const ADDRESS_LENGTH: usize = 32;
const KEY_LENGTH: usize = 32;
const TOPIC_LENGTH: usize = 32;

fn address_hex(address: &H256) -> alloc::string::String {
    alloc::format!("0x{}", hex::encode(address.as_bytes()))
}

fn key_hex(address: &[u8]) -> alloc::string::String {
    alloc::format!("0x{}", hex::encode(address))
}

pub struct AccountData {
    pub address: Address,
    pub nonce: u64,
    pub balance: BigUint,
    pub storage: HashMap<Vec<u8>, Vec<u8>>,
    pub contract: Option<Vec<u8>>,
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

pub struct TxData {
    pub from: Address,
    pub to: Address,
    pub call_value: BigUint,
    pub func_name: Vec<u8>,
    pub new_contract: Option<Vec<u8>>,
    pub args: Vec<Vec<u8>>,
}

impl fmt::Display for TxData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TxData {{ func: {}, args: {:?}, call_value: {}, from: 0x{}, to: 0x{}\n}}", 
            String::from_utf8(self.func_name.clone()).unwrap(), 
            self.args, 
            self.call_value,
            address_hex(&self.from), 
            address_hex(&self.to))
    }
}

#[derive(Clone)]
pub struct TxResult {
    pub result_status: i32,
    pub result_values: Vec<Vec<u8>>,
}

impl fmt::Display for TxResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let results_hex: Vec<String> = self.result_values.iter().map(|r| format!("0x{}", hex::encode(r))).collect();
        write!(f, "TxResult {{\n\tresult_status: {},\n\tresult_values:{:?}\n}}", self.result_status, results_hex)
    }
}

impl TxResult {
    pub fn empty() -> TxResult {
        TxResult {
            result_status: 0,
            result_values: Vec::new(),
        }
    }
    pub fn print(&self) {
        println!("{}", self);
    }
}

// pub trait ContractFactory {
//     fn new_contract<'t>(&self, state_ref: &'t mut ArwenMockState) -> Box<dyn CallableContract + 't>;
// }

// pub struct ContactFactoryImpl<C: CallableContract> {
//     phantom_data: core::marker::PhantomData<C>,
// }

// impl<C: CallableContract> ContactFactoryImpl<C> {
//     pub fn new() -> Self {
//         ContactFactoryImpl{ phantom_data: core::marker::PhantomData }
//     }
// }

// impl<C: CallableContract> ContractFactory for ContactFactoryImpl<C> {
//     fn new_contract<'t>(&self, state_ref: &'t mut ArwenMockState) -> Box<dyn CallableContract + 't> {
//         Box::new(C::new(state_ref))
//     }
// }

pub struct ContractFactories(HashMap<Vec<u8>, Box<dyn ContractFactory<ArwenMockState>>>);

impl ContractFactories {
    pub fn new() -> Self {
        ContractFactories(HashMap::new())
    }

    pub fn register_contract(&mut self,
        path: &str,
        contract_factory: Box<dyn ContractFactory<ArwenMockState>>) {

        self.0.insert(path.as_bytes().to_vec(), contract_factory);
    }

    pub fn new_contract<'t>(&self, path: &str, state_ref: &'t mut ArwenMockState) -> Box<dyn CallableContract + 't> {
        let factory = self.0.get(&path.as_bytes().to_vec()).unwrap();
        factory.new_contract(state_ref)
    }
}

pub struct ArwenMockState {
    pub current_tx: Option<TxData>,
    pub current_result: TxResult,
    pub accounts: HashMap<Address, AccountData>,
    pub new_addresses: HashMap<(Address, u64), Address>,
}

impl ArwenMockState {
    pub fn new() -> ArwenMockState {
        ArwenMockState{
            current_tx: None,
            current_result: TxResult::empty(),
            accounts: HashMap::new(),
            new_addresses: HashMap::new(),
        }
    }

    fn create_account_if_deploy(&mut self, tx: &mut TxData) {
        if let Some(tx_contract) = &tx.new_contract {
            if let Some(sender) = self.accounts.get(&tx.from) {
                if let Some(new_address) = self.get_new_address(tx.from.clone(), sender.nonce) {
                    if self.accounts.contains_key(&new_address) {
                        panic!("Account already exists at deploy address.");
                    }
                    self.accounts.insert(new_address.clone(), AccountData{
                        address: new_address.clone(),
                        nonce: 0,
                        balance: 0u32.into(),
                        storage: HashMap::new(),
                        contract: Some(tx_contract.clone()),
                    });
                    tx.to = new_address;
                } else {
                    panic!("Missing new address. Only explicit new deploy addresses supported.");
                }
            } else {
                panic!("Unknown deployer");
            }
        }
    }

    pub fn set_result_status(&mut self, status: i32) {
        self.current_result.result_status = status;
    }
    
    pub fn add_result(&mut self, result: Vec<u8>) {
        self.current_result.result_values.push(result);
    }
    
    pub fn clear_result(&mut self) {
        self.current_result = TxResult::empty();
    }
    
    fn get_result(&self) -> TxResult {
        self.current_result.clone()
    }

    fn get_new_address(&self, creator_address: Address, creator_nonce: u64) -> Option<Address> {
        self.new_addresses
            .get(&(creator_address, creator_nonce))
            .map(|addr_ref| addr_ref.clone())
    }
}

impl ArwenMockState {
    // pub fn get_contract(&self) -> Box<dyn CallableContract> {
    //     let tx_ref = self.current_tx.as_ref().unwrap();
    //     if let Some(account) = self.accounts.get(&tx_ref.to) {
    //         if let Some(contract_identifier) = &account.contract {
    //             if let Some(new_contract_closure) = self.contract_factories.get(contract_identifier) {
    //                 new_contract_closure(&mut self)
    //             } else {
    //                 panic!("Unknown contract");
    //             }
    //         } else {
    //             panic!("Recipient account is not a smart contract");
    //         }
    //     } else {
    //         panic!("Account not found");
    //     }
    // }

    // pub fn execute_tx(&self, mut tx: TxData) -> TxResult {
    //     {
    //         let mut state = self.state_ref.borrow_mut();
    //         state.create_account_if_deploy(&mut tx);    
    //         state.current_tx = Some(tx);
    //         state.clear_result();
    //     }
        
    //     let func_name = {
    //         let state = self.state_ref.borrow();
    //         state.current_tx.as_ref().unwrap().func_name.clone()
    //     };
        
    //     {
    //     let mut contract = self.get_contract();

    //     // contract call
    //     // important: state cannot be borrowed at this point
    //     contract.call(&func_name[..]);
    //     }
        
    //     let state = self.state_ref.borrow();
    //     state.get_result()
    // }

    /// To be used for writing small tests.
    // pub fn set_dummy_tx(&self, addr: &Address) {
    //     let mut tx = TxData {
    //         func_name: Vec::new(),
    //         new_contract: None,
    //         args: Vec::new(),
    //         call_value: 0u32.into(),
    //         from: addr.clone(),
    //         to: addr.clone(),
    //     };

    //     {
    //         // let mut state = self.state_ref.borrow_mut();
    //         state.create_account_if_deploy(&mut tx);    
    //         state.current_tx = Some(tx);
    //         state.clear_result();
    //     }
    // }

    pub fn add_account(&mut self, acct: AccountData) {
        // let mut state = self.state_ref.borrow_mut();
        self.accounts.insert(acct.address.clone(), acct);
    }

    pub fn print_accounts(&self) {
        // let state = self.state_ref.borrow();
        let mut accounts_buf = String::new();
        for (address, account) in &self.accounts {
            write!(&mut accounts_buf, "\n\t{} -> {}", address_hex(address), account).unwrap();
        }
        println!("Accounts: {}", &accounts_buf);
    }

    pub fn put_new_address(&mut self, creator_address: Address, creator_nonce: u64, new_address: Address) {
        // let mut state = self.state_ref.borrow_mut();
        self.new_addresses.insert((creator_address, creator_nonce), new_address);
    }

    // pub fn get_contract_factory(&self,
    //     path: &str) -> Option<Box<dyn ContractFactory<ArwenMockState> + 'static>> {

    //     self.contract_factories.get(path.as_bytes().to_vec())
    // }

    // pub fn clear_state(&self) {
    //     let mut state = self.state_ref.borrow_mut();
    //     state.contract_factories.clear();
    //     state.accounts.clear();
    //     state.new_addresses.clear();
    //     state.current_tx = None;
    // }
}

impl elrond_wasm::ContractHookApi<RustBigInt, RustBigUint> for ArwenMockState {
    fn get_sc_address(&self) -> Address {
        // let state = self.state_ref.borrow();
        match &self.current_tx {
            None => panic!("Tx not initialized!"),
            Some(tx) => tx.to.clone(),
        }
    }

    fn get_owner_address(&self) -> Address {
        panic!("get_sc_address not yet implemented")
    }

    fn get_caller(&self) -> Address {
        // let state = self.state_ref.borrow();
        match &self.current_tx {
            None => panic!("Tx not initialized!"),
            Some(tx) => tx.from.clone(),
        }
    }

    fn get_balance(&self, _address: &Address) -> RustBigUint {
        panic!("get_balance not yet implemented")
    }

    fn storage_store(&mut self, key: &[u8], value: &[u8]) {
        let sc_address = self.get_sc_address();
        // let mut state = self.state_ref.borrow_mut();
        match self.accounts.get_mut(&sc_address) {
            None => panic!("Account not found!"),
            Some(acct) => {
                acct.storage.insert(key.to_vec(), value.to_vec());
            }
        }
    }

    fn storage_load(&self, key: &[u8]) -> Vec<u8> {
        // let state = self.state_ref.borrow();
        match &self.current_tx {
            None => panic!("Tx not initialized!"),
            Some(tx) => {
                match self.accounts.get(&tx.to) {
                    None => panic!("Account not found!"),
                    Some(acct) => {
                        match acct.storage.get(&key.to_vec()) {
                            None => Vec::with_capacity(0),
                            Some(value) => {
                                value.clone()
                            },
                        }
                    }
                }
            }
        }
    }

    #[inline]
    fn storage_load_len(&self, key: &[u8]) -> usize {
        self.storage_load(key).len()
    }

    fn storage_store_bytes32(&mut self, key: &[u8], value: &[u8; 32]) {
        let mut vector = Vec::with_capacity(32);
        for i in value.iter() {
            vector.push(*i);
        }
        self.storage_store(key, &vector);
    }
    
    fn storage_load_bytes32(&self, key: &[u8]) -> [u8; 32] {
        let value = self.storage_load(key);
        let mut res = [0u8; 32];
        let offset = 32 - value.len();
        if !value.is_empty() {
            res[offset..(value.len()-1 + offset)].clone_from_slice(&value[..value.len()-1]);
            // for i in 0..value.len()-1 {
            //     res[offset+i] = value[i];
            // }
        }
        res
    }

    fn storage_store_big_uint(&mut self, key: &[u8], value: &RustBigUint) {
        self.storage_store(key, &value.to_bytes_be());
    }

    fn storage_load_big_uint(&self, key: &[u8]) -> RustBigUint {
        let value = self.storage_load(key);
        let bi = BigInt::from_bytes_be(num_bigint::Sign::Plus, value.as_slice());
        bi.into()
    }

    fn storage_store_big_int(&mut self, key: &[u8], value: &RustBigInt) {
        self.storage_store(key, &value.to_signed_bytes_be());
    }

    fn storage_load_big_int(&self, key: &[u8]) -> RustBigInt {
        let value = self.storage_load(key);
        let bi = BigInt::from_signed_bytes_be(value.as_slice());
        bi.into()
    }

    fn storage_store_i64(&mut self, key: &[u8], value: i64) {
        self.storage_store_big_int(key, &RustBigInt::from(value));
    }

    fn storage_load_i64(&self, key: &[u8]) -> Option<i64> {
        let bi = self.storage_load_big_int(key);
        bi.value().to_i64()
    }

    #[inline]
    fn get_call_value_big_uint(&self) -> RustBigUint {
        // let state = self.state_ref.borrow();
        match &self.current_tx {
            None => panic!("Tx not initialized!"),
            Some(tx) => tx.call_value.clone().into(),
        }
    }

    fn send_tx(&mut self, to: &Address, amount: &RustBigUint, _message: &str) {
        let owner = self.get_sc_address();
        // let mut state = self.state_ref.borrow_mut();
        match self.accounts.get_mut(&owner) {
            None => panic!("Account not found!"),
            Some(acct) => {
                acct.balance -= amount.value();
            }
        }
        match self.accounts.get_mut(to) {
            None => panic!("Account not found!"),
            Some(acct) => {
                acct.balance += amount.value();
            }
        }
    }

    fn async_call(&self, _to: &Address, _amount: &RustBigUint, _data: &[u8]) {
        panic!("async_call not yet implemented");
    }

    fn get_tx_hash(&self) -> H256 {
        panic!("get_tx_hash not yet implemented");
    }

    fn get_gas_left(&self) -> i64 {
        1000000000
    }

    fn get_block_timestamp(&self) -> u64 {
        0
    }

    fn get_block_nonce(&self) -> u64 {
        0
    }

    fn get_block_round(&self) -> u64 {
        0
    }

    fn get_block_epoch(&self) -> u64 {
        0
    }

    fn sha256(&self, data: &[u8]) -> [u8; 32] {
        let mut hasher = Sha3_256::new();
        hasher.input(data);
        hasher.result().into()
    }

    fn keccak256(&self, data: &[u8]) -> [u8; 32] {
        let mut hasher = Keccak256::new();
        hasher.input(data);
        hasher.result().into()
    }
}

impl ArwenMockState {
    fn get_argument_vec(&self, arg_index: i32) -> Vec<u8> {
        let arg_idx_usize: usize = arg_index as usize;
        match &self.current_tx {
            None => panic!("Tx not initialized!"),
            Some(tx) => {
                if arg_idx_usize >= tx.args.len() {
                    panic!("Tx arg index out of range");
                }
                tx.args[arg_idx_usize].clone()
            },
        }
    }
}

impl elrond_wasm::ContractIOApi<RustBigInt, RustBigUint> for ArwenMockState {

    fn get_num_arguments(&self) -> i32 {
        // let state = self.state_ref.borrow();
        let nr_args = match &self.current_tx {
            None => panic!("Tx not initialized!"),
            Some(tx) => tx.args.len(),
        };
        nr_args as i32
    }

    fn check_not_payable(&self) {
        if self.get_call_value_big_uint() > 0 {
            self.signal_error(err_msg::NON_PAYABLE);
        }
    }

    fn get_argument_len(&self, arg_index: i32) -> usize {
        // let state = self.state_ref.borrow();
        let arg = self.get_argument_vec(arg_index);
        arg.len()
    }

    fn copy_argument_to_slice(&self, _arg_index: i32, _slice: &mut [u8]) {
        panic!("copy_argument_to_slice not yet implemented")
    }

    fn get_argument_vec(&self, arg_index: i32) -> Vec<u8> {
        // let state = self.state_ref.borrow();
        self.get_argument_vec(arg_index)
    }

    fn get_argument_bytes32(&self, arg_index: i32) -> [u8; 32] {
        // let state = self.state_ref.borrow();
        let arg = self.get_argument_vec(arg_index);
        let mut res = [0u8; 32];
        let offset = 32 - arg.len();
        res[offset..(arg.len()-1 + offset)].clone_from_slice(&arg[..arg.len()-1]);
        res
    }
    
    fn get_argument_big_int(&self, arg_index: i32) -> RustBigInt {
        // let state = self.state_ref.borrow();
        let bytes = self.get_argument_vec(arg_index);
        BigInt::from_signed_bytes_be(&bytes).into()
    }

    #[inline]
    fn get_argument_big_uint(&self, _arg_index: i32) -> RustBigUint {
        panic!("get_argument_big_uint not yet implemented")
    }

    #[inline]
    fn get_argument_i64(&self, arg_index: i32) -> i64 {
        // let state = self.state_ref.borrow();
        let bytes = self.get_argument_vec(arg_index);
        let bi = BigInt::from_signed_bytes_be(&bytes);
        if let Some(v) = bi.to_i64() {
            v
        } else {
            panic!("Argument does not fit in an i64.")
        }
    }

    fn finish_slice_u8(&mut self, slice: &[u8]) {
        // let mut state = self.state_ref.borrow_mut();
        let mut v = vec![0u8; slice.len()];
        v.copy_from_slice(slice);
        self.add_result(v);
    }

    fn finish_bytes32(&mut self, bytes: &[u8; 32]) {
        self.finish_slice_u8(&*bytes);
    }

    #[inline]
    fn finish_big_int(&mut self, bi: &RustBigInt) {
        // let mut state = self.state_ref.borrow_mut();
        self.add_result(bi.to_signed_bytes_be());
    }

    #[inline]
    fn finish_big_uint(&mut self, bu: &RustBigUint) {
        // let mut state = self.state_ref.borrow_mut();
        self.add_result(bu.to_bytes_be());
    }
    
    #[inline]
    fn finish_i64(&mut self, value: i64) {
        self.finish_big_int(&value.into());
    }

    fn signal_error(&self, message: &[u8]) -> ! {
        let s = std::str::from_utf8(message);
        panic!("signal_error was called with message: {}", s.unwrap())
    }

    fn write_log(&self, _topics: &[[u8;32]], _data: &[u8]) {
        println!("write_log not yet implemented");
    }
}
