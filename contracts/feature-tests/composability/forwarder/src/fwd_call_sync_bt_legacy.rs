#![allow(deprecated)]

use crate::vault_proxy;

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait BackTransfersLegacyModule {
    #[endpoint]
    fn forward_sync_retrieve_funds_bt_legacy(
        &self,
        to: ManagedAddress,
        token: EgldOrEsdtTokenIdentifier,
        token_nonce: u64,
        amount: NonZeroBigUint,
    ) {
        let back_transfers = self
            .tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .retrieve_funds(token, token_nonce, amount)
            .returns(ReturnsBackTransfersLegacy)
            .sync_call();

        require!(
            back_transfers.esdt_payments.len() == 1 || back_transfers.total_egld_amount != 0,
            "Only one ESDT payment expected"
        );

        self.back_transfers_event(
            &back_transfers.total_egld_amount,
            &back_transfers.esdt_payments.into_multi_value(),
        );
    }

    #[endpoint]
    fn forward_sync_retrieve_funds_bt_legacy_reset_twice(
        &self,
        to: ManagedAddress,
        token: EgldOrEsdtTokenIdentifier,
        token_nonce: u64,
        amount: NonZeroBigUint,
    ) {
        let back_transfers = self
            .tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .retrieve_funds(token.clone(), token_nonce, amount.clone())
            .returns(ReturnsBackTransfersLegacyReset)
            .sync_call();

        require!(
            back_transfers.esdt_payments.len() == 1 || back_transfers.total_egld_amount != 0,
            "Only one ESDT payment expected"
        );

        self.back_transfers_event(
            &back_transfers.total_egld_amount,
            &back_transfers.esdt_payments.into_multi_value(),
        );

        let back_transfers = self
            .tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .retrieve_funds(token, token_nonce, amount)
            .returns(ReturnsBackTransfersLegacyReset)
            .sync_call();

        require!(
            back_transfers.esdt_payments.len() == 1 || back_transfers.total_egld_amount != 0,
            "Only one ESDT payment expected"
        );

        self.back_transfers_event(
            &back_transfers.total_egld_amount,
            &back_transfers.esdt_payments.into_multi_value(),
        );
    }

    #[endpoint]
    fn forward_sync_retrieve_funds_bt_legacy_twice(
        &self,
        to: ManagedAddress,
        token: EgldOrEsdtTokenIdentifier,
        token_nonce: u64,
        amount: NonZeroBigUint,
    ) {
        let back_transfers = self
            .tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .retrieve_funds(token.clone(), token_nonce, amount.clone())
            .returns(ReturnsBackTransfersLegacy)
            .sync_call();

        require!(
            back_transfers.esdt_payments.len() == 1 || back_transfers.total_egld_amount != 0,
            "Only one ESDT payment expected"
        );

        self.back_transfers_event(
            &back_transfers.total_egld_amount,
            &back_transfers.esdt_payments.into_multi_value(),
        );

        let back_transfers = self
            .tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .retrieve_funds(token, token_nonce, amount)
            .returns(ReturnsBackTransfersLegacy)
            .sync_call();

        require!(
            back_transfers.esdt_payments.len() == 1 || back_transfers.total_egld_amount != 0,
            "Only one ESDT payment expected"
        );

        self.back_transfers_event(
            &back_transfers.total_egld_amount,
            &back_transfers.esdt_payments.into_multi_value(),
        );
    }

    #[event("back_transfers")]
    fn back_transfers_event(
        &self,
        #[indexed] egld_value: &BigUint,
        #[indexed] multi_esdt: &MultiValueEncoded<EsdtTokenPaymentMultiValue>,
    );
}
