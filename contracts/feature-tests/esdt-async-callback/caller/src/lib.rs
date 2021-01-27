#![no_std]

use elrond_wasm::HexCallDataSerializer;

imports!();

const CALLEE_FUNCTION_NAME: &[u8] = b"requestEsdt";

#[elrond_wasm_derive::contract(CallerImpl)]
pub trait Caller {
	#[init]
	fn init(&self, callee_address: Address) {
		self.set_callee_address(&callee_address);
	}

	#[endpoint(requestEsdtFromOtherContract)]
	fn request_esdt_from_other_contract(&self, token_identifier: BoxedBytes, amount: BigUint) {
		let mut serializer = HexCallDataSerializer::new(CALLEE_FUNCTION_NAME);
		serializer.push_argument_bytes(token_identifier.as_slice());
		serializer.push_argument_bytes(amount.to_bytes_be().as_slice());

		self.async_call(
			&self.get_callee_address(),
			&BigUint::zero(),
			serializer.as_slice(),
		);
	}

	#[callback_raw]
	fn callback_raw(&self, result: Vec<Vec<u8>>) {
		let token_identifier = BoxedBytes::from(self.get_esdt_token_name());
		let amount = self.get_esdt_value_big_uint();

		let err_code_vec = &result[0];

		match u32::dep_decode(&mut err_code_vec.as_slice()) {
			core::result::Result::Ok(err_code) => {
				if err_code == 0 {
					self.set_last_fulfilled_request(&(token_identifier, amount));
				}
			},
			core::result::Result::Err(_) => {},
		}
	}

	#[view(getCalleeAddress)]
	#[storage_get("calleeAddress")]
	fn get_callee_address(&self) -> Address;

	#[storage_set("calleeAddress")]
	fn set_callee_address(&self, callee_address: &Address);

	#[view(getLastFulfilledRequest)]
	#[storage_get("lastFulfilledRequest")]
	fn get_last_fulfilled_request(&self) -> (BoxedBytes, BigUint);

	#[storage_set("lastFulfilledRequest")]
	fn set_last_fulfilled_request(&self, token_identifier_amount_pair: &(BoxedBytes, BigUint));
}
