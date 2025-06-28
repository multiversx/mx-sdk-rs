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
        amount: BigUint,
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
        let egld_transfer = back_transfers.egld_sum();

        if egld_transfer != BigUint::zero() {
            let egld_token_id = EgldOrEsdtTokenIdentifier::egld();
            self.retrieve_funds_callback_event(&egld_token_id, 0, &egld_transfer);

            let _ = self.callback_data().push(&CallbackData {
                callback_name: ManagedBuffer::from(b"retrieve_funds_callback"),
                token_identifier: egld_token_id,
                token_nonce: 0,
                token_amount: egld_transfer,
                args: ManagedVec::new(),
            });
        }

        for esdt_transfer in back_transfers.payments {
            // let esdt_token_id =
            //     EgldOrEsdtTokenIdentifier::esdt(esdt_transfer.token_identifier.clone());
            self.retrieve_funds_callback_event(
                &esdt_transfer.token_identifier,
                esdt_transfer.token_nonce,
                &esdt_transfer.amount,
            );

            let _ = self.callback_data().push(&CallbackData {
                callback_name: ManagedBuffer::from(b"retrieve_funds_callback"),
                token_identifier: esdt_transfer.token_identifier,
                token_nonce: esdt_transfer.token_nonce,
                token_amount: esdt_transfer.amount.clone(),
                args: ManagedVec::new(),
            });
        }
    }
}
