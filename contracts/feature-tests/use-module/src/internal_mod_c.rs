elrond_wasm::imports!();

/// Contains all events that can be emitted by the contract.
#[elrond_wasm_derive::module(InteralModuleCImpl)]
pub trait InternalModuleC {
	#[view]
	fn call_mod_c(&self) {}
}
