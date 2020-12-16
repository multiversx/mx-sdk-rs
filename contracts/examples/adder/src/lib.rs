#![no_std]

imports!();

#[elrond_wasm_derive::contract(AdderImpl)]
pub trait Adder {
	#[view(getSum)]
	#[storage_get("sum")]
	fn get_sum(&self) -> BigInt;

	#[storage_set("sum")]
	fn set_sum(&self, sum: &BigInt);

	#[init]
	fn init(&self, initial_value: &BigInt) {
		self.set_sum(initial_value);
	}

	#[endpoint]
	fn add(&self, value: &BigInt) -> SCResult<()> {
		let mut sum = self.get_sum();
		sum += value;
		self.set_sum(&sum);

		Ok(())
	}
}
