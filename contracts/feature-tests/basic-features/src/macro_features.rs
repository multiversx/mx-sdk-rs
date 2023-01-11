multiversx_sc::imports!();

/// Various macros provided by multiversx-sc.
#[multiversx_sc::module]
pub trait Macros {
    #[only_owner]
    #[endpoint]
    fn only_owner_endpoint(&self) {}

    #[only_user_account]
    #[endpoint]
    fn only_user_account_endpoint(&self) {}

    #[view]
    fn require_equals(&self, a: u32, b: u32) {
        require!(a == b, "a must equal b");
    }

    #[view]
    fn sc_panic(&self) {
        sc_panic!("sc_panic test");
    }

    // TODO: add panic formatting here?
}
