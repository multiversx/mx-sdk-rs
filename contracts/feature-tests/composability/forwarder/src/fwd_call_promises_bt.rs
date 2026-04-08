multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::{
    common::{self, CallbackData},
    vault_proxy,
};
#[multiversx_sc::module]
pub trait CallPromisesBackTransfersModule: common::CommonModule {
    #[endpoint]
    fn forward_promise_retrieve_funds_back_transfers(
        &self,
        to: ManagedAddress,
        token: EgldOrEsdtTokenIdentifier,
        token_nonce: u64,
        amount: NonZeroBigUint,
    ) {
        let gas_limit = self.blockchain().get_gas_left() - 20_000_000;
        self.tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .retrieve_funds(token, token_nonce, amount)
            .gas(gas_limit)
            .callback(self.callbacks().retrieve_funds_back_transfers_callback())
            .gas_for_callback(10_000_000)
            .register_promise();
    }

    #[promises_callback]
    fn retrieve_funds_back_transfers_callback(&self) {
        let back_transfers = self.blockchain().get_back_transfers();
        let bt_payments = back_transfers.into_payment_vec();
        for payment in bt_payments {
            self.retrieve_funds_callback_event(
                &payment.token_identifier,
                payment.token_nonce,
                &payment.amount,
            );

            let _ = self.callback_data().push(&CallbackData {
                callback_name: ManagedBuffer::from(b"retrieve_funds_callback"),
                token_identifier: payment.token_identifier,
                token_nonce: payment.token_nonce,
                token_amount: payment.amount,
                args: ManagedVec::new(),
            });
        }
    }
}
