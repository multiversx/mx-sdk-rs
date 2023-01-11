multiversx_sc::imports!();

#[multiversx_sc::module]
#[only_owner]
pub trait OnlyOwnerTestModule: super::only_owner_derived_mod::OnlyOwnerDerivedTestModule {
    #[endpoint]
    fn only_owner_mod_endpoint(&self) {}
}
