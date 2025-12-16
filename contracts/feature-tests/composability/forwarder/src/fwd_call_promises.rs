multiversx_sc::imports!();

use crate::{
    common::{self, CallbackData},
    vault_proxy,
};

#[multiversx_sc::module]
pub trait CallPromisesModule: common::CommonModule {
    #[endpoint]
    #[payable("*")]
    fn forward_promise_accept_funds(&self, to: ManagedAddress) {
        let payment = self.call_value().all();
        let gas_limit = self.blockchain().get_gas_left() / 2;

        self.tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .accept_funds()
            .gas(gas_limit)
            .payment(payment)
            .register_promise();
    }

    #[endpoint]
    fn forward_promise_retrieve_funds(
        &self,
        to: ManagedAddress,
        token: EgldOrEsdtTokenIdentifier,
        token_nonce: u64,
        amount: BigUint,
    ) {
        let gas_limit = self.blockchain().get_gas_left() - 20_000_000;

        self.tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .retrieve_funds(token, token_nonce, amount)
            .gas(gas_limit)
            .callback(self.callbacks().retrieve_funds_callback())
            .gas_for_callback(10_000_000)
            .register_promise();
    }

    #[promises_callback]
    fn retrieve_funds_callback(&self) {
        self.promises_callback_event();

        let call_value = self.call_value().all();
        for payment in &*call_value {
            self.retrieve_funds_callback_event(
                &payment.token_identifier,
                payment.token_nonce,
                &payment.amount,
            );

            let _ = self.callback_data().push(&CallbackData {
                callback_name: ManagedBuffer::from(b"retrieve_funds_callback"),
                token_identifier: payment.token_identifier.clone(),
                token_nonce: payment.token_nonce,
                token_amount: payment.amount.clone(),
                args: ManagedVec::new(),
            });
        }
    }

    #[event("promises_callback")]
    fn promises_callback_event(&self);

    #[endpoint]
    #[payable("*")]
    fn forward_payment_callback(&self, to: ManagedAddress) {
        let payment = self.call_value().all();
        let gas_limit = self.blockchain().get_gas_left() / 2;

        self.tx()
            .to(&to)
            .gas(gas_limit)
            .payment(payment)
            .callback(self.callbacks().transfer_callback())
            .register_promise();
    }

    #[endpoint]
    #[payable("*")]
    fn forward_payment_gas_for_callback(&self, to: ManagedAddress) {
        let payment = self.call_value().all();
        let half_gas = self.blockchain().get_gas_left() / 3;

        self.tx()
            .to(&to)
            .gas(half_gas)
            .payment(payment)
            .callback(self.callbacks().transfer_callback())
            .gas_for_callback(half_gas)
            .register_promise();
    }

    #[promises_callback]
    fn transfer_callback(&self, #[call_result] result: MultiValueEncoded<ManagedBuffer>) {
        self.callback_result(result);

        let call_value = self.call_value().all();
        for payment in &*call_value {
            self.retrieve_funds_callback_event(
                &payment.token_identifier,
                payment.token_nonce,
                &payment.amount,
            );

            let _ = self.callback_data().push(&CallbackData {
                callback_name: ManagedBuffer::from(b"transfer_callback"),
                token_identifier: payment.token_identifier.clone(),
                token_nonce: payment.token_nonce,
                token_amount: payment.amount.clone(),
                args: ManagedVec::new(),
            });
        }
    }

    #[event("callback_result")]
    fn callback_result(&self, #[indexed] result: MultiValueEncoded<ManagedBuffer>);
}
