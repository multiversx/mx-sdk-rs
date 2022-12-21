mx_sc::imports!();
mx_sc::derive_imports!();

#[mx_sc::module]
pub trait EventsModule {
    #[event("buy-token")]
    fn buy_token_event(&self, #[indexed] user: &ManagedAddress, amount: &BigUint);

    #[event("sell-token")]
    fn sell_token_event(&self, #[indexed] user: &ManagedAddress, amount: &BigUint);
}
