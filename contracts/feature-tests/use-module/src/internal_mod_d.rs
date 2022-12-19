mx_sc::imports!();

/// This module is in the crate, but it is not included.
/// Its endpoints should not appear in the contract binary.
#[mx_sc::module]
pub trait InternalModuleD {
    #[view]
    fn call_mod_d(&self) {}
}
