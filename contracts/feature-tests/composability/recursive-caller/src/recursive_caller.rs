#![no_std]

multiversx_sc::imports!();

/// Test contract for investigating async calls.
#[multiversx_sc::contract]
pub trait RecursiveCaller {
    #[proxy]
    fn vault_proxy(&self) -> vault::Proxy<Self::Api>;

    #[proxy]
    fn self_proxy(&self) -> self::Proxy<Self::Api>;

    #[init]
    fn init(&self) {}

    #[endpoint]
    fn recursive_send_funds(
        &self,
        to: &ManagedAddress,
        token_identifier: &EgldOrEsdtTokenIdentifier,
        amount: &BigUint,
        counter: u32,
    ) {
        self.recursive_send_funds_event(to, token_identifier, amount, counter);

        self.vault_proxy()
            .contract(to.clone())
            .accept_funds()
            .with_egld_or_single_esdt_transfer((token_identifier.clone(), 0, amount.clone()))
            .async_call()
            .with_callback(self.callbacks().recursive_send_funds_callback(
                to,
                token_identifier,
                amount,
                counter,
            ))
            .call_and_exit()
    }

    #[callback]
    fn recursive_send_funds_callback(
        &self,
        to: &ManagedAddress,
        token_identifier: &EgldOrEsdtTokenIdentifier,
        amount: &BigUint,
        counter: u32,
    ) {
        self.recursive_send_funds_callback_event(to, token_identifier, amount, counter);

        if counter > 1 {
            self.self_proxy()
                .contract(self.blockchain().get_sc_address())
                .recursive_send_funds(to, token_identifier, amount, counter - 1)
                .async_call()
                .call_and_exit()
        }
    }

    #[event("recursive_send_funds")]
    fn recursive_send_funds_event(
        &self,
        #[indexed] to: &ManagedAddress,
        #[indexed] token_identifier: &EgldOrEsdtTokenIdentifier,
        #[indexed] amount: &BigUint,
        counter: u32,
    );

    #[event("recursive_send_funds_callback")]
    fn recursive_send_funds_callback_event(
        &self,
        #[indexed] to: &ManagedAddress,
        #[indexed] token_identifier: &EgldOrEsdtTokenIdentifier,
        #[indexed] amount: &BigUint,
        counter: u32,
    );
}
