#![no_std]

elrond_wasm::imports!();

const ACCEPTED_TRANSFER_ANSWER: u32 = 0xbc197c81;

#[elrond_wasm_derive::contract(Erc1155UserMockImpl)]
pub trait Erc1155UserMock {
	#[init]
	fn init(&self) {}

	#[endpoint(onERC1155Received)]
	fn on_erc1155_received(
		&self,
		_operator: Address,
		_from: Address,
		_type_id: BigUint,
		_value: BigUint,
		_data: &[u8],
	) -> SCResult<u32> {

		Ok(ACCEPTED_TRANSFER_ANSWER)
	}

	#[endpoint(onERC1155BatchReceived)]
	fn on_erc1155_batch_received(
		&self,
		_operator: Address,
		_from: Address,
		_type_ids: &[BigUint],
		_values: &[BigUint],
		_data: &[u8],
	) -> SCResult<u32> {

		Ok(ACCEPTED_TRANSFER_ANSWER)
	}
}
