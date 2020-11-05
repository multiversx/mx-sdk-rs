#![no_std]

imports!();

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

	#[endpoint]
	fn add(&self, value: &BigInt) -> SCResult<()> {
		let mut sum = self.get_mut_sum();
		*sum += value;
		Ok(())
	}
}
