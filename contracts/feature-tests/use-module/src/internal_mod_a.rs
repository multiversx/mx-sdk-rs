elrond_wasm::imports!();

/// Contains all events that can be emitted by the contract.
#[elrond_wasm::module]
pub trait InternalModuleA: super::internal_mod_b::InternalModuleB {
	#[view]
	fn call_mod_a(&self) {}
}
