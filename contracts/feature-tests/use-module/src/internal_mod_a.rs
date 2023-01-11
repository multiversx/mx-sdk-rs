multiversx_sc::imports!();

/// Example of a module that lies in the same crate.
/// It also includes another module, also from the same crate.
#[multiversx_sc::module]
pub trait InternalModuleA:
    super::internal_mod_b::InternalModuleB + super::internal_mod_init::InternalModuleInit
{
    #[view]
    fn call_mod_a(&self) {}

    #[view]
    #[label("module-external-view")]
    fn external_view_mod_a(&self) {}
}
