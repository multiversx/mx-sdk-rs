elrond_wasm::imports!();

#[elrond_wasm::only_owner_module]
pub trait OnlyOwnerModule: super::only_owner_derived_mod::OnlyOwnerDerivedModule {
    #[endpoint]
    fn only_owner_mod_endpoint(&self) {}
}
