imports!();

use super::internal_mod_a::*;

/// Contains all events that can be emitted by the contract.
#[elrond_wasm_derive::module(InteralModuleBImpl)]
pub trait InteralModuleB {
	#[module(InteralModuleAImpl)]
	fn internal_module_a(&self) -> InteralModuleAImpl<T, BigInt, BigUint>;

	#[view]
	fn call_mod_b(&self) {}
}
