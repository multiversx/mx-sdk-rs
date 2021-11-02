elrond_wasm::imports!();

/// This module is in the crate, but it is not included.
/// Its endpoints should not appear in the contract binary.
#[elrond_wasm::module]
pub trait InternalModuleD {
    #[view]
    fn call_mod_d(&self) {}
}
