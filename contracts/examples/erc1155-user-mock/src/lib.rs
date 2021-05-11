#![no_std]

elrond_wasm::imports!();

#[elrond_wasm_derive::contract]
pub trait Erc1155UserMock {
	#[init]
	fn init(&self) {}

	#[endpoint(onERC1155Received)]
	fn on_erc1155_received(
		&self,
		_operator: Address,
		_from: Address,
		_type_id: Self::BigUint,
		_value: Self::BigUint,
		_data: &[u8],
	) -> SCResult<()> {
		Ok(())
	}

	#[endpoint(onERC1155BatchReceived)]
	fn on_erc1155_batch_received(
		&self,
		_operator: Address,
		_from: Address,
		_type_ids: Vec<Self::BigUint>,
		_values: Vec<Self::BigUint>,
		_data: &[u8],
	) -> SCResult<()> {
		Ok(())
	}
}
