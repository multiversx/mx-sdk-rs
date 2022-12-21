use mx_sc_modules::only_admin;

mx_sc::imports!();

#[mx_sc::module]
#[only_admin]
pub trait OnlyAdminTestModule:
    super::only_admin_derived_mod::OnlyAdminDerivedTestModule + only_admin::OnlyAdminModule
{
    #[endpoint]
    fn only_admin_mod_endpoint(&self) {}
}
