#![no_std]

elrond_wasm::imports!();

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
	) -> SCResult<()> {

		self.set_test(42);

		Ok(())
	}

	#[endpoint(onERC1155BatchReceived)]
	fn on_erc1155_batch_received(
		&self,
		_operator: Address,
		_from: Address,
		_type_ids: Vec<BigUint>,
		_values: Vec<BigUint>,
		_data: &[u8],
	) -> SCResult<()> {

		Ok(())
	}

	#[storage_set("test")]
	fn set_test(&self, val: u32);

	#[storage_get("test")]
	fn get_test(&self) -> u32;
}
