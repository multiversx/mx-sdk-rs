elrond_wasm::imports!();

use super::internal_mod_b::*;

/// Contains all events that can be emitted by the contract.
#[elrond_wasm_derive::module(InteralModuleAImpl)]
pub trait InteralModuleA {
	#[module(InteralModuleAImpl)]
	fn internal_module_a(&self) -> InteralModuleAImpl<T, BigInt, BigUint>;

	#[module(InteralModuleBImpl)]
	fn internal_module_b(&self) -> InteralModuleBImpl<T, BigInt, BigUint>;

	#[view]
	fn call_mod_a(&self) {}
}
