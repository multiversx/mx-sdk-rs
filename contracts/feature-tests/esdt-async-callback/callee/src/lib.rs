#![no_std]

use elrond_wasm::HexCallDataSerializer;

imports!();

const ESDT_TRANSFER_STRING: &[u8] = b"ESDTTransfer";

#[elrond_wasm_derive::contract(CalleeImpl)]
pub trait Callee {
	#[init]
	fn init(&self) {}

	#[endpoint(requestEsdt)]
	fn request_esdt(&self, token_identifier: BoxedBytes, amount: BigUint) {
		let mut serializer = HexCallDataSerializer::new(ESDT_TRANSFER_STRING);
		serializer.push_argument_bytes(token_identifier.as_slice());
		serializer.push_argument_bytes(amount.to_bytes_be().as_slice());

		self.async_call(
			&self.get_caller(),
			&BigUint::zero(),
			serializer.as_slice(),
		);
	}
}
