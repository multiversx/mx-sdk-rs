elrond_wasm::imports!();

/// This module is in the crate, but it is not included.
/// Its endpoints should not appear in the contract binary.
#[elrond_wasm::only_owner_module]
pub trait OnlyOwnerModule: super::only_owner_derived_mod::OnlyOwnerDerivedModule {
    #[endpoint]
    fn only_owner_mod_endpoint(&self) {}
}
