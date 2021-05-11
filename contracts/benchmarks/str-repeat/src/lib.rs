#![no_std]

elrond_wasm::imports!();

#[elrond_wasm_derive::contract]
pub trait StrRepeat {
	#[init]
	fn init(&self) {}

	#[endpoint]
	fn repeat(&self, string: &[u8], num_repeats: usize) -> Vec<u8> {
		let mut result = Vec::<u8>::new();
		for _ in 0..num_repeats {
			result.extend_from_slice(string);
		}
		result
	}
}
