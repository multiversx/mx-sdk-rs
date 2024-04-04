#![no_std]

multiversx_sc::imports!();

pub mod self_proxy;
pub mod vault_proxy;

/// Test contract for investigating async calls.
#[multiversx_sc::contract]
pub trait RecursiveCaller {
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

        self.tx()
            .to(to)
            .typed(vault_proxy::VaultProxy)
            .accept_funds()
            .egld_or_single_esdt(token_identifier, 0, amount)
            .async_call()
            .with_callback(self.callbacks().recursive_send_funds_callback(
                to,
                token_identifier,
                amount,
                counter,
            ))
            .call_and_exit();
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
            let self_address = self.blockchain().get_sc_address();
            self.tx()
                .to(&self_address)
                .typed(self_proxy::RecursiveCallerProxy)
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
