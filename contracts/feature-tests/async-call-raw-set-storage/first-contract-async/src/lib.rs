#![no_std]
#![allow(unused_attributes)]

elrond_wasm::imports!();

use elrond_wasm::HexCallDataSerializer;

const SECOND_CONTRACT_ENDPOINT_NAME: &[u8] = b"callMe";

#[elrond_wasm_derive::contract(FirstContractAsyncImpl)]
pub trait FirstContractAsync {
	#[init]
	fn init(&self) {}

	#[endpoint(callSecondContract)]
	fn call_second_contract(&self, callee: Address, arg: u32) {
		let mut serializer = HexCallDataSerializer::new(SECOND_CONTRACT_ENDPOINT_NAME);
		serializer.push_argument_bytes(&arg.to_be_bytes()[..]);

		self.send().async_call_raw(&callee, &BigUint::zero(), serializer.as_slice());
	}

	// callback

	#[callback_raw]
	fn callback_raw(&self, #[var_args] result: AsyncCallResult<u32>) {
		match result {
			AsyncCallResult::Ok(val) => self.set_callback_value(val),
			AsyncCallResult::Err(_) => self.set_callback_value(404)
		}
	}

	// storage

	#[storage_get("callbackValue")]
	fn get_callback_value(&self) -> u32;

	#[storage_set("callbackValue")]
	fn set_callback_value(&self, val: u32);
}
