multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::common::{self, CallbackData};

#[multiversx_sc::module]
pub trait CallPromisesBackTransfersModule: common::CommonModule {
    #[proxy]
    fn vault_proxy(&self) -> vault::Proxy<Self::Api>;

    #[endpoint]
    fn forward_promise_retrieve_funds_back_transfers(
        &self,
        to: ManagedAddress,
        token: EgldOrEsdtTokenIdentifier,
        token_nonce: u64,
        amount: BigUint,
    ) {
        let gas_limit = self.blockchain().get_gas_left() - 20_000_000;
        self.vault_proxy()
            .contract(to)
            .retrieve_funds(token, token_nonce, amount)
            .with_gas_limit(gas_limit)
            .async_call_promise()
            .with_callback(self.callbacks().retrieve_funds_back_transfers_callback())
            .with_extra_gas_for_callback(10_000_000)
            .register_promise()
    }

    #[promises_callback]
    fn retrieve_funds_back_transfers_callback(&self) {
        let back_transfers = self.blockchain().get_back_transfers();
        let egld_transfer = back_transfers.total_egld_amount;

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

        for esdt_transfer in &back_transfers.esdt_payments {
            let (token, nonce, payment) = esdt_transfer.into_tuple();
            let esdt_token_id = EgldOrEsdtTokenIdentifier::esdt(token);
            self.retrieve_funds_callback_event(&esdt_token_id, nonce, &payment);

            let _ = self.callback_data().push(&CallbackData {
                callback_name: ManagedBuffer::from(b"retrieve_funds_callback"),
                token_identifier: esdt_token_id,
                token_nonce: nonce,
                token_amount: payment,
                args: ManagedVec::new(),
            });
        }
    }
}
