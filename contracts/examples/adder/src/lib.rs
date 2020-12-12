#![no_std]

imports!();

/// One of the simplest smart contracts possible,
/// it holds a single variable in storage, which anyone can increment.
#[elrond_wasm_derive::contract(AdderImpl)]
pub trait Adder {
	#[view(getSum)]
	#[storage_get_mut("sum")]
	fn get_mut_sum(&self) -> mut_storage!(BigInt);

	#[storage_set("sum")]
	fn set_sum(&self, sum: &BigInt);

	#[init]
	fn init(&self, initial_value: &BigInt) {
		self.set_sum(initial_value);
	}

	/// Add desired amount to the storage variable.
	#[endpoint]
	fn add(&self, value: &BigInt) -> SCResult<()> {
		let mut sum = self.get_mut_sum();
		*sum += value;
		Ok(())
	}
}
