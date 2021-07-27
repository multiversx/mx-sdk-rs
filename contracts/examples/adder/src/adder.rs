#![no_std]

elrond_wasm::imports!();

/// One of the simplest smart contracts possible,
/// it holds a single variable in storage, which anyone can increment.
#[elrond_wasm_derive::contract]
pub trait Adder {
	#[view(getSum)]
	#[storage_mapper("sum")]
	fn sum(&self) -> SingleValueMapper<Self::Storage, Self::BigInt>;

	#[init]
	fn init(&self, initial_value: Self::BigInt) {
		self.sum().set(&initial_value);
	}

	/// Add desired amount to the storage variable.
	#[endpoint]
	fn add(&self, value: Self::BigInt) -> SCResult<()> {
		self.sum().update(|sum| *sum += value);

		Ok(())
	}
}
