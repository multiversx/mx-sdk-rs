elrond_wasm::imports!();

/// Example of a module that lies in the same crate.
#[elrond_wasm::module]
pub trait OnlyOwnerDerivedModule {
    #[view]
    fn call_derived_not_owner_only(&self) {}
}
