multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::{
    common::{self, CallbackData},
    vault_proxy,
};

const PERCENTAGE_TOTAL: u64 = 10_000; // 100%

#[multiversx_sc::module]
pub trait ForwarderAsyncCallModule: common::CommonModule {
    #[endpoint]
    fn echo_args_async(&self, to: ManagedAddress, args: MultiValueEncoded<ManagedBuffer>) {
        self.tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .echo_arguments(args)
            .callback(self.callbacks().echo_args_callback())
            .async_call_and_exit();
    }

    #[callback]
    fn echo_args_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<MultiValueEncoded<ManagedBuffer>>,
    ) -> MultiValueEncoded<ManagedBuffer> {
        match result {
            ManagedAsyncCallResult::Ok(results) => {
                let mut cb_result =
                    ManagedVec::from_single_item(ManagedBuffer::new_from_bytes(b"success"));
                cb_result.append_vec(results.into_vec_of_buffers());

                cb_result.into()
            }
            ManagedAsyncCallResult::Err(err) => {
                let mut cb_result =
                    ManagedVec::from_single_item(ManagedBuffer::new_from_bytes(b"error"));
                cb_result.push(ManagedBuffer::new_from_bytes(
                    &err.err_code.to_be_bytes()[..],
                ));
                cb_result.push(err.err_msg);

                cb_result.into()
            }
        }
    }

    #[endpoint]
    #[payable("*")]
    fn forward_async_accept_funds(&self, to: ManagedAddress) {
        let payment = self.call_value().all();
        self.tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .accept_funds()
            .payment(payment)
            .async_call_and_exit()
    }

    /// TODO: not tested, investigate
    #[endpoint]
    #[payable("*")]
    fn forward_async_accept_funds_half_payment(&self, to: ManagedAddress) {
        let payment = self.call_value().single();
        let half_payment = &payment.amount / 2u32;
        self.tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .accept_funds()
            .payment(PaymentRefs::new(
                &payment.token_identifier,
                payment.token_nonce,
                &half_payment,
            ))
            .async_call_and_exit()
    }

    #[payable("*")]
    #[endpoint]
    fn forward_async_accept_funds_with_fees(&self, to: ManagedAddress, percentage_fees: u32) {
        let payment = self.call_value().single();
        let fees = &payment.amount * percentage_fees / PERCENTAGE_TOTAL;
        let amount_to_send = &payment.amount - &fees;

        self.tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .accept_funds()
            .payment(PaymentRefs::new(
                &payment.token_identifier,
                payment.token_nonce,
                &amount_to_send,
            ))
            .async_call_and_exit();
    }

    #[endpoint]
    fn forward_async_retrieve_funds(
        &self,
        to: ManagedAddress,
        token: EgldOrEsdtTokenIdentifier,
        token_nonce: u64,
        amount: BigUint,
    ) {
        self.tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .retrieve_funds(token, token_nonce, amount)
            .callback(self.callbacks().retrieve_funds_callback())
            .async_call_and_exit()
    }

    #[endpoint]
    #[payable]
    fn forward_async_reject_funds(&self, to: ManagedAddress) {
        let payment = self.call_value().all();
        self.tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .reject_funds()
            .payment(MultiTransfer(payment))
            .callback(self.callbacks().retrieve_funds_callback())
            .async_call_and_exit()
    }

    #[callback]
    fn retrieve_funds_callback(&self) {
        let (token, nonce, payment) = self.call_value().egld_or_single_esdt().into_tuple();
        self.retrieve_funds_callback_event(&token, nonce, &payment);

        let _ = self.callback_data().push(&CallbackData {
            callback_name: ManagedBuffer::from(b"retrieve_funds_callback"),
            token_identifier: token,
            token_nonce: nonce,
            token_amount: payment,
            args: ManagedVec::new(),
        });
    }

    #[endpoint]
    fn send_funds_twice(
        &self,
        to: &ManagedAddress,
        token_identifier: &EgldOrEsdtTokenIdentifier,
        amount: &BigUint,
    ) {
        self.tx()
            .to(to)
            .typed(vault_proxy::VaultProxy)
            .accept_funds()
            .egld_or_single_esdt(token_identifier, 0u64, amount)
            .callback(
                self.callbacks()
                    .send_funds_twice_callback(to, token_identifier, amount),
            )
            .async_call_and_exit();
    }

    #[callback]
    fn send_funds_twice_callback(
        &self,
        to: &ManagedAddress,
        token_identifier: &EgldOrEsdtTokenIdentifier,
        cb_amount: &BigUint,
    ) {
        self.tx()
            .to(to)
            .typed(vault_proxy::VaultProxy)
            .accept_funds()
            .egld_or_single_esdt(token_identifier, 0u64, cb_amount)
            .async_call_and_exit();
    }

    #[endpoint]
    fn send_async_accept_multi_transfer(
        &self,
        to: ManagedAddress,
        payment_args: MultiValueEncoded<MultiValue3<EgldOrEsdtTokenIdentifier, u64, BigUint>>,
    ) {
        self.tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .accept_funds()
            .payment(payment_args.convert_payment_multi_triples())
            .async_call_and_exit();
    }

    #[endpoint]
    fn send_async_reject_multi_transfer(
        &self,
        to: ManagedAddress,
        payment_args: MultiValueEncoded<MultiValue3<EgldOrEsdtTokenIdentifier, u64, BigUint>>,
    ) {
        self.tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .reject_funds()
            .payment(payment_args.convert_payment_multi_triples())
            .async_call_and_exit();
    }
}
