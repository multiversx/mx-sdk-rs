use elrond_wasm::{Address, ArgBuffer, BoxedBytes, CodeMetadata, H256};

use crate::async_data::*;
use crate::big_int_mock::*;
use crate::big_uint_mock::*;
use crate::blockchain_mock::*;
use crate::display_util::*;

use elrond_wasm::api::ErrorApi;
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
	pub esdt_token_name: Vec<u8>,
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
				esdt_token_name: Vec::new(),
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

	#[inline]
	fn get_call_value_big_uint(&self) -> RustBigUint {
		self.tx_input_box.call_value.clone().into()
	}

	#[inline]
	fn get_esdt_value_big_uint(&self) -> RustBigUint {
		self.tx_input_box.esdt_value.clone().into()
	}

	#[inline]
	fn get_esdt_token_name(&self) -> Vec<u8> {
		self.tx_input_box.esdt_token_name.clone()
	}

	fn send_tx(&self, to: &Address, amount: &RustBigUint, _data: &[u8]) {
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

	fn deploy_contract(
		&self,
		_gas: u64,
		_amount: &RustBigUint,
		_code: &BoxedBytes,
		_code_metadata: CodeMetadata,
		_arg_buffer: &ArgBuffer,
	) -> Address {
		panic!("deploy_contract not yet implemented")
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
		self.blockchain_info_box
			.current_block_info
			.block_random_seed
			.clone()
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
		self.blockchain_info_box
			.previous_block_info
			.block_random_seed
			.clone()
	}

	// TODO: Remove underscores when implementing

	fn execute_on_dest_context(
		&self,
		_gas: u64,
		_address: &Address,
		_value: &RustBigUint,
		_function: &[u8],
		_arg_buffer: &ArgBuffer,
	) {
		panic!("execute_on_dest_context not implemented yet!");
	}

	fn execute_on_dest_context_by_caller(
		&self,
		_gas: u64,
		_address: &Address,
		_value: &RustBigUint,
		_function: &[u8],
		_arg_buffer: &ArgBuffer,
	) {
		panic!("execute_on_dest_context_by_caller not implemented yet!");
	}

	fn execute_on_same_context(
		&self,
		_gas: u64,
		_address: &Address,
		_value: &RustBigUint,
		_function: &[u8],
		_arg_buffer: &ArgBuffer,
	) {
		panic!("execute_on_same_context not implemented yet!");
	}
}

impl elrond_wasm::ContractIOApi<RustBigInt, RustBigUint> for TxContext {}
