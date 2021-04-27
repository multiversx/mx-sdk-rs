elrond_wasm::imports!();

use super::internal_mod_a;
use super::internal_mod_b;

/// Contains all events that can be emitted by the contract.
#[elrond_wasm_derive::module(InteralModuleAImpl)]
pub trait InteralModuleA {
	#[module(InteralModuleAImpl)]
	fn internal_module_a(
		&self,
	) -> internal_mod_a::implementation::InteralModuleA<T, BigInt, BigUint>;

	#[module(InteralModuleBImpl)]
	fn internal_module_b(
		&self,
	) -> internal_mod_b::implementation::InteralModuleB<T, BigInt, BigUint>;

	#[view]
	fn call_mod_a(&self) {}
}
