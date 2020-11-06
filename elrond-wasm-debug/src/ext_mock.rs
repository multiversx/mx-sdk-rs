use elrond_wasm::{Address, H256};

use crate::async_data::*;
use crate::big_int_mock::*;
use crate::big_uint_mock::*;
use crate::blockchain_mock::*;
use crate::display_util::*;

use elrond_wasm::err_msg;
use elrond_wasm::ContractHookApi;
use elrond_wasm::{BigIntApi, BigUintApi};

use num_bigint::{BigInt, BigUint};
use num_traits::cast::ToPrimitive;

use alloc::vec::Vec;

use std::collections::HashMap;
use std::fmt;

use alloc::rc::Rc;
use core::cell::RefCell;

use sha3::{Digest, Keccak256, Sha3_256};

const ADDRESS_LENGTH: usize = 32;
const TOPIC_LENGTH: usize = 32;

pub struct TxPanic {
	pub status: u64,
	pub message: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct TxInput {
	pub from: Address,
	pub to: Address,
	pub call_value: BigUint,
	pub esdt_value: BigUint,
	pub esdt_token_name: Option<Vec<u8>>,
	pub func_name: Vec<u8>,
	pub args: Vec<Vec<u8>>,
	pub gas_limit: u64,
	pub gas_price: u64,
	pub tx_hash: H256,
}

impl fmt::Display for TxInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TxInput {{ func: {}, args: {:?}, call_value: {}, esdt_token_name: {:?}, esdt_value: {:?}, from: 0x{}, to: 0x{}\n}}", 
            String::from_utf8(self.func_name.clone()).unwrap(), 
            self.args, 
            self.call_value,
            self.esdt_token_name,
            self.esdt_value,
            address_hex(&self.from), 
            address_hex(&self.to))
    }
}

impl TxInput {
	pub fn add_arg(&mut self, arg: Vec<u8>) {
		self.args.push(arg);
	}
}

#[derive(Clone, Debug)]
pub struct TxResult {
	pub result_status: u64,
	pub result_message: Vec<u8>,
	pub result_values: Vec<Vec<u8>>,
}

impl fmt::Display for TxResult {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let results_hex: Vec<String> = self
			.result_values
			.iter()
			.map(|r| format!("0x{}", hex::encode(r)))
			.collect();
		write!(
			f,
			"TxResult {{\n\tresult_status: {},\n\tresult_values:{:?}\n}}",
			self.result_status, results_hex
		)
	}
}

impl TxResult {
	pub fn empty() -> TxResult {
		TxResult {
			result_status: 0,
			result_message: Vec::new(),
			result_values: Vec::new(),
		}
	}
	pub fn print(&self) {
		println!("{}", self);
	}
}

#[derive(Debug)]
pub struct SendBalance {
	pub recipient: Address,
	pub amount: BigUint,
}

#[derive(Debug)]
pub struct TxOutput {
	pub contract_storage: HashMap<Vec<u8>, Vec<u8>>,
	pub result: TxResult,
	pub send_balance_list: Vec<SendBalance>,
	pub async_call: Option<AsyncCallTxData>,
}

impl Default for TxOutput {
	fn default() -> Self {
		TxOutput {
			contract_storage: HashMap::new(),
			result: TxResult::empty(),
			send_balance_list: Vec::new(),
			async_call: None,
		}
	}
}

impl TxOutput {
	pub fn from_panic_obj(panic_obj: &TxPanic) -> Self {
		TxOutput {
			contract_storage: HashMap::new(),
			result: TxResult {
				result_status: panic_obj.status,
				result_message: panic_obj.message.clone(),
				result_values: Vec::new(),
			},
			send_balance_list: Vec::new(),
			async_call: None,
		}
	}

	pub fn from_panic_string(_: &str) -> Self {
		TxOutput {
			contract_storage: HashMap::new(),
			result: TxResult {
				result_status: 4,
				result_message: b"panic occurred".to_vec(),
				result_values: Vec::new(),
			},
			send_balance_list: Vec::new(),
			async_call: None,
		}
	}
}

#[derive(Debug)]
pub struct TxContext {
	pub blockchain_info_box: Box<BlockchainTxInfo>,
	pub tx_input_box: Box<TxInput>,
	pub tx_output_cell: Rc<RefCell<TxOutput>>,
}

impl TxContext {
	pub fn new(blockchain_info: BlockchainTxInfo, tx_input: TxInput, tx_output: TxOutput) -> Self {
		TxContext {
			blockchain_info_box: Box::new(blockchain_info),
			tx_input_box: Box::new(tx_input),
			tx_output_cell: Rc::new(RefCell::new(tx_output)),
		}
	}

	pub fn into_output(self) -> TxOutput {
		let ref_cell = Rc::try_unwrap(self.tx_output_cell).unwrap();
		ref_cell.replace(TxOutput::default())
	}

	pub fn dummy() -> Self {
		TxContext {
			blockchain_info_box: Box::new(BlockchainTxInfo {
				previous_block_info: BlockInfo::new(),
				current_block_info: BlockInfo::new(),
				contract_balance: 0u32.into(),
				contract_owner: None,
			}),
			tx_input_box: Box::new(TxInput {
				from: Address::zero(),
				to: Address::zero(),
				call_value: 0u32.into(),
				esdt_value: 0u32.into(),
				esdt_token_name: None,
				func_name: Vec::new(),
				args: Vec::new(),
				gas_limit: 0,
				gas_price: 0,
				tx_hash: b"dummy...........................".into(),
			}),
			tx_output_cell: Rc::new(RefCell::new(TxOutput::default())),
		}
	}
}

impl Clone for TxContext {
	fn clone(&self) -> Self {
		TxContext {
			blockchain_info_box: self.blockchain_info_box.clone(),
			tx_input_box: self.tx_input_box.clone(),
			tx_output_cell: Rc::clone(&self.tx_output_cell),
		}
	}
}

impl elrond_wasm::ContractHookApi<RustBigInt, RustBigUint> for TxContext {
	fn get_sc_address(&self) -> Address {
		self.tx_input_box.to.clone()
	}

	fn get_owner_address(&self) -> Address {
		self.blockchain_info_box
			.contract_owner
			.clone()
			.unwrap_or_else(|| panic!("contract owner address not set"))
	}

	fn get_caller(&self) -> Address {
		self.tx_input_box.from.clone()
	}

	fn get_balance(&self, address: &Address) -> RustBigUint {
		if address != &self.get_sc_address() {
			panic!("get balance not yet implemented for accounts other than the contract itself");
		}
		self.blockchain_info_box.contract_balance.clone().into()
	}

	fn storage_store_slice_u8(&self, key: &[u8], value: &[u8]) {
		// TODO: extract magic strings somewhere
		if key.starts_with(&b"ELROND"[..]) {
			panic!(TxPanic {
				status: 10,
				message: b"cannot write to storage under Elrond reserved key".to_vec(),
			});
		}

		let mut tx_output = self.tx_output_cell.borrow_mut();
		tx_output
			.contract_storage
			.insert(key.to_vec(), value.to_vec());
	}

	fn storage_load_vec_u8(&self, key: &[u8]) -> Vec<u8> {
		let tx_output = self.tx_output_cell.borrow();
		match tx_output.contract_storage.get(&key.to_vec()) {
			None => Vec::with_capacity(0),
			Some(value) => value.clone(),
		}
	}

	#[inline]
	fn storage_load_len(&self, key: &[u8]) -> usize {
		self.storage_load_vec_u8(key).len()
	}

	fn storage_store_bytes32(&self, key: &[u8], value: &[u8; 32]) {
		self.storage_store_slice_u8(key, &value[..].to_vec());
	}

	fn storage_load_bytes32(&self, key: &[u8]) -> [u8; 32] {
		let value = self.storage_load_vec_u8(key);
		let mut res = [0u8; 32];
		let offset = 32 - value.len();
		if !value.is_empty() {
			res[offset..].clone_from_slice(&value[..]);
		}
		res
	}

	fn storage_store_big_uint(&self, key: &[u8], value: &RustBigUint) {
		self.storage_store_slice_u8(key, &value.to_bytes_be());
	}

	fn storage_load_big_uint(&self, key: &[u8]) -> RustBigUint {
		let value = self.storage_load_vec_u8(key);
		let bi = BigInt::from_bytes_be(num_bigint::Sign::Plus, value.as_slice());
		bi.into()
	}

	fn storage_store_big_int(&self, key: &[u8], value: &RustBigInt) {
		self.storage_store_slice_u8(key, &value.to_signed_bytes_be());
	}

	fn storage_load_big_int(&self, key: &[u8]) -> RustBigInt {
		let value = self.storage_load_vec_u8(key);
		let bi = BigInt::from_signed_bytes_be(value.as_slice());
		bi.into()
	}

	fn storage_store_i64(&self, key: &[u8], value: i64) {
		self.storage_store_big_int(key, &RustBigInt::from(value));
	}

	fn storage_store_u64(&self, key: &[u8], value: u64) {
		self.storage_store_big_uint(key, &RustBigUint::from(value));
	}

	fn storage_load_i64(&self, key: &[u8]) -> i64 {
		let bi = self.storage_load_big_int(key);
		if let Some(v) = bi.0.to_i64() {
			v
		} else {
			panic!(TxPanic {
				status: 10,
				message: b"storage value out of range".to_vec(),
			})
		}
	}

	fn storage_load_u64(&self, key: &[u8]) -> u64 {
		let bu = self.storage_load_big_uint(key);
		if let Some(v) = bu.0.to_u64() {
			v
		} else {
			panic!(TxPanic {
				status: 10,
				message: b"storage value out of range".to_vec(),
			})
		}
	}

	#[inline]
	fn get_call_value_big_uint(&self) -> RustBigUint {
		self.tx_input_box.call_value.clone().into()
	}

	#[inline]
	fn get_esdt_value_big_uint(&self) -> RustBigUint {
		self.tx_input_box.esdt_value.clone().into()
	}

	#[inline]
	fn get_esdt_token_name(&self) -> Option<Vec<u8>> {
		self.tx_input_box.esdt_token_name.clone()
	}

	fn send_tx(&self, to: &Address, amount: &RustBigUint, _message: &str) {
		let mut tx_output = self.tx_output_cell.borrow_mut();
		tx_output.send_balance_list.push(SendBalance {
			recipient: to.clone(),
			amount: amount.value(),
		})
	}

	fn async_call(&self, to: &Address, amount: &RustBigUint, data: &[u8]) {
		let mut tx_output = self.tx_output_cell.borrow_mut();
		tx_output.async_call = Some(AsyncCallTxData {
			to: to.clone(),
			call_value: amount.value(),
			call_data: data.to_vec(),
			tx_hash: self.get_tx_hash(),
		});
	}

	fn get_tx_hash(&self) -> H256 {
		self.tx_input_box.tx_hash.clone()
	}

	fn get_gas_left(&self) -> u64 {
		self.tx_input_box.gas_limit
	}

	fn get_block_timestamp(&self) -> u64 {
		self.blockchain_info_box.current_block_info.block_timestamp
	}

	fn get_block_nonce(&self) -> u64 {
		self.blockchain_info_box.current_block_info.block_nonce
	}

	fn get_block_round(&self) -> u64 {
		self.blockchain_info_box.current_block_info.block_round
	}

	fn get_block_epoch(&self) -> u64 {
		self.blockchain_info_box.current_block_info.block_epoch
	}

	fn get_block_random_seed(&self) -> Box<[u8; 48]> {
		Box::new([0u8; 48])
	}

	fn get_prev_block_timestamp(&self) -> u64 {
		self.blockchain_info_box.previous_block_info.block_timestamp
	}

	fn get_prev_block_nonce(&self) -> u64 {
		self.blockchain_info_box.previous_block_info.block_nonce
	}

	fn get_prev_block_round(&self) -> u64 {
		self.blockchain_info_box.previous_block_info.block_round
	}

	fn get_prev_block_epoch(&self) -> u64 {
		self.blockchain_info_box.previous_block_info.block_epoch
	}

	fn get_prev_block_random_seed(&self) -> Box<[u8; 48]> {
		Box::new([0u8; 48])
	}

	fn sha256(&self, data: &[u8]) -> H256 {
		let mut hasher = Sha3_256::new();
		hasher.input(data);
		let hash: [u8; 32] = hasher.result().into();
		hash.into()
	}

	fn keccak256(&self, data: &[u8]) -> H256 {
		let mut hasher = Keccak256::new();
		hasher.input(data);
		let hash: [u8; 32] = hasher.result().into();
		hash.into()
	}
}

impl elrond_wasm::ContractIOApi<RustBigInt, RustBigUint> for TxContext {
	fn get_num_arguments(&self) -> i32 {
		self.tx_input_box.args.len() as i32
	}

	fn check_not_payable(&self) {
		if self.get_call_value_big_uint() > 0 {
			self.signal_error(err_msg::NON_PAYABLE);
		}
	}

	fn get_argument_len(&self, arg_index: i32) -> usize {
		let arg = self.get_argument_vec_u8(arg_index);
		arg.len()
	}

	fn copy_argument_to_slice(&self, _arg_index: i32, _slice: &mut [u8]) {
		panic!("copy_argument_to_slice not yet implemented")
	}

	fn get_argument_vec_u8(&self, arg_index: i32) -> Vec<u8> {
		let arg_idx_usize = arg_index as usize;
		if arg_idx_usize >= self.tx_input_box.args.len() {
			panic!("Tx arg index out of range");
		}
		self.tx_input_box.args[arg_idx_usize].clone()
	}

	fn get_argument_boxed_slice_u8(&self, arg_index: i32) -> Box<[u8]> {
		self.get_argument_vec_u8(arg_index).into_boxed_slice()
	}

	fn get_argument_bytes32(&self, arg_index: i32) -> [u8; 32] {
		let arg = self.get_argument_vec_u8(arg_index);
		let mut res = [0u8; 32];
		let offset = 32 - arg.len();
		res[offset..].copy_from_slice(&arg[..]);
		res
	}

	fn get_argument_big_int(&self, arg_index: i32) -> RustBigInt {
		let bytes = self.get_argument_vec_u8(arg_index);
		RustBigInt::from_signed_bytes_be(&bytes)
	}

	fn get_argument_big_uint(&self, arg_index: i32) -> RustBigUint {
		let bytes = self.get_argument_vec_u8(arg_index);
		RustBigUint::from_bytes_be(&bytes[..])
	}

	fn get_argument_i64(&self, arg_index: i32) -> i64 {
		let bytes = self.get_argument_vec_u8(arg_index);
		let bi = BigInt::from_signed_bytes_be(&bytes);
		if let Some(v) = bi.to_i64() {
			v
		} else {
			panic!(TxPanic {
				status: 10,
				message: b"argument out of range".to_vec(),
			})
		}
	}

	fn get_argument_u64(&self, arg_index: i32) -> u64 {
		let bytes = self.get_argument_vec_u8(arg_index);
		let bu = BigUint::from_bytes_be(&bytes);
		if let Some(v) = bu.to_u64() {
			v
		} else {
			panic!(TxPanic {
				status: 10,
				message: b"argument out of range".to_vec(),
			})
		}
	}

	fn finish_slice_u8(&self, slice: &[u8]) {
		let mut v = vec![0u8; slice.len()];
		v.copy_from_slice(slice);
		let mut tx_output = self.tx_output_cell.borrow_mut();
		tx_output.result.result_values.push(v)
	}

	fn finish_bytes32(&self, bytes: &[u8; 32]) {
		self.finish_slice_u8(&*bytes);
	}

	fn finish_big_int(&self, bi: &RustBigInt) {
		self.finish_slice_u8(bi.to_signed_bytes_be().as_slice());
	}

	fn finish_big_uint(&self, bu: &RustBigUint) {
		self.finish_slice_u8(bu.to_bytes_be().as_slice());
	}

	fn finish_big_int_raw(&self, _handle: i32) {
		panic!("cannot call finish_big_int_raw in debug mode");
	}

	fn finish_big_uint_raw(&self, _handle: i32) {
		panic!("cannot call finish_big_uint_raw in debug mode");
	}

	fn finish_i64(&self, value: i64) {
		self.finish_big_int(&value.into());
	}

	fn finish_u64(&self, value: u64) {
		self.finish_big_uint(&value.into());
	}

	fn signal_error(&self, message: &[u8]) -> ! {
		panic!(TxPanic {
			status: 4,
			message: message.to_vec()
		})
	}

	fn write_log(&self, _topics: &[[u8; 32]], _data: &[u8]) {
		// does nothing yet
		// TODO: implement at some point
	}
}
