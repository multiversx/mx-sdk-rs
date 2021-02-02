use super::big_uint_api_mock::*;
use crate::{SendBalance, TxContext, TxPanic};
use elrond_wasm::api::SendApi;
use elrond_wasm::types::{Address, TokenIdentifier};
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

	fn direct_esdt_explicit_gas(&self, to: &Address, token: &[u8], amount: &RustBigUint, gas: u64, data: &[u8]) {
		panic!()
	}
}
