elrond_wasm::imports!();

/// Contains all events that can be emitted by the contract.
#[elrond_wasm::module]
pub trait InternalModuleC {
	#[view]
	fn call_mod_c(&self) {}
}
