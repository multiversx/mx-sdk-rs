mx_sc::imports!();

/// Example of a module that lies in the same crate.
#[mx_sc::module]
pub trait InternalModuleC {
    #[view]
    fn call_mod_c(&self) {}
}
