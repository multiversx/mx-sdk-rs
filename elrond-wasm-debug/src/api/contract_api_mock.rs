use super::big_int_api_mock::*;
use super::big_uint_api_mock::*;
use crate::async_data::*;
use crate::{SendBalance, TxContext};
use elrond_wasm::{Address, ArgBuffer, BoxedBytes, CodeMetadata, H256};

impl elrond_wasm::api::ContractHookApi<RustBigInt, RustBigUint> for TxContext {
	type Storage = Self;

	fn get_storage_raw(&self) -> Self::Storage {
		self.clone()
	}

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
