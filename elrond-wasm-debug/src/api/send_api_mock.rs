use super::big_uint_api_mock::*;
use crate::async_data::AsyncCallTxData;
use crate::{SendBalance, TxContext, TxPanic};
use elrond_wasm::api::{ContractHookApi, SendApi};
use elrond_wasm::types::{Address, ArgBuffer, BoxedBytes, CodeMetadata};
use num_bigint::BigUint;

impl TxContext {
	fn get_available_balance(&self) -> BigUint {
		// start with the pre-existing balance
		let mut available_balance = self.blockchain_info_box.contract_balance.clone();

		// add amount received received
		available_balance += &self.tx_input_box.call_value;
		let tx_output = self.tx_output_cell.borrow();

		// already sent
		for send_balance in &tx_output.send_balance_list {
			available_balance -= &send_balance.amount;
		}

		available_balance
	}
}

impl SendApi<RustBigUint> for TxContext {
	fn direct_egld(&self, to: &Address, amount: &RustBigUint, _data: &[u8]) {
		if &amount.value() > &self.get_available_balance() {
			panic!(TxPanic {
				status: 10,
				message: b"failed transfer (insufficient funds)".to_vec(),
			});
		}

		let mut tx_output = self.tx_output_cell.borrow_mut();
		tx_output.send_balance_list.push(SendBalance {
			recipient: to.clone(),
			amount: amount.value(),
		})
	}

	fn direct_esdt_explicit_gas(
		&self,
		_to: &Address,
		_token: &[u8],
		_amount: &RustBigUint,
		_gas: u64,
		_data: &[u8],
	) {
		panic!()
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
