#![no_std]

elrond_wasm::imports!();

/// Test contract for investigating async calls.
#[elrond_wasm::contract]
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
        token_identifier: &TokenIdentifier,
        amount: &BigUint,
        counter: u32,
    ) -> AsyncCall {
        self.recursive_send_funds_event(to, token_identifier, amount, counter);

        self.vault_proxy()
            .contract(to.clone())
            .accept_funds(token_identifier.clone(), 0, amount.clone())
            .async_call()
            .with_callback(self.callbacks().recursive_send_funds_callback(
                to,
                token_identifier,
                amount,
                counter,
            ))
    }

    #[callback]
    fn recursive_send_funds_callback(
        &self,
        to: &ManagedAddress,
        token_identifier: &TokenIdentifier,
        amount: &BigUint,
        counter: u32,
    ) -> OptionalResult<AsyncCall> {
        self.recursive_send_funds_callback_event(to, token_identifier, amount, counter);

        if counter > 1 {
            OptionalResult::Some(
                self.self_proxy()
                    .contract(self.blockchain().get_sc_address())
                    .recursive_send_funds(to, token_identifier, amount, counter - 1)
                    .async_call(),
            )
        } else {
            OptionalResult::None
        }
    }

    #[event("recursive_send_funds")]
    fn recursive_send_funds_event(
        &self,
        #[indexed] to: &ManagedAddress,
        #[indexed] token_identifier: &TokenIdentifier,
        #[indexed] amount: &BigUint,
        counter: u32,
    );

    #[event("recursive_send_funds_callback")]
    fn recursive_send_funds_callback_event(
        &self,
        #[indexed] to: &ManagedAddress,
        #[indexed] token_identifier: &TokenIdentifier,
        #[indexed] amount: &BigUint,
        counter: u32,
    );
}
