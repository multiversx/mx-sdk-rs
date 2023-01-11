multiversx_sc::imports!();

/// Example of a module that lies in the same crate.
#[multiversx_sc::module]
pub trait InternalModuleB {
    #[view]
    fn call_mod_b(&self) {}

    #[view]
    #[label("module-external-view")]
    fn external_view_mod_b(&self) {}

    #[event("eventInModule")]
    fn event_in_module(&self, #[indexed] arg: u32);
}
