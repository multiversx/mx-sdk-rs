elrond_wasm::imports!();

#[elrond_wasm::module]
pub trait OnlyOwnerDerivedModule {
    #[view]
    fn call_derived_not_owner_only(&self) {}
}
