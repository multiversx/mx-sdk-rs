mx_sc::imports!();

#[mx_sc::module]
pub trait OnlyAdminDerivedTestModule {
    #[view]
    fn call_derived_not_admin_only(&self) {}
}
