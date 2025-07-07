use crate::vault_proxy;

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait BackTransfersModule {
    #[endpoint]
    fn forward_sync_retrieve_funds_bt_multi(
        &self,
        to: ManagedAddress,
        transfers: MultiValueEncoded<EgldOrEsdtTokenPaymentMultiValue>,
    ) {
        let bt_multi = self
            .tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .retrieve_funds_multi(&transfers)
            .returns(ReturnsBackTransfers)
            .sync_call();

        let egld_sum = bt_multi.egld_sum();
        if egld_sum > 0u32 {
            self.back_transfers_egld_event(egld_sum);
        }
        self.back_transfers_multi_event(bt_multi.into_multi_value());

        let mut balances_after = MultiValueEncoded::new();
        for transfer in transfers {
            let payment = transfer.into_inner();
            let balance = self
                .blockchain()
                .get_sc_balance(&payment.token_identifier, payment.token_nonce);
            let balance_info =
                EgldOrEsdtTokenPayment::new(payment.token_identifier, payment.token_nonce, balance);
            balances_after.push(EgldOrEsdtTokenPaymentMultiValue::from(balance_info));
        }
        self.balances_after(balances_after);
    }

    /// Highlights the behavior when calling back transfers **without** reset.
    #[endpoint]
    fn forward_sync_retrieve_funds_bt_multi_twice(
        &self,
        to: ManagedAddress,
        transfers: MultiValueEncoded<EgldOrEsdtTokenPaymentMultiValue>,
    ) {
        self.tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .retrieve_funds_multi(&transfers)
            .sync_call();

        let back_transfers = self
            .tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .retrieve_funds_multi(&transfers)
            .returns(ReturnsBackTransfers)
            .sync_call();

        self.back_transfers_multi_event(back_transfers.into_multi_value());
    }

    /// Highlights the behavior when calling back transfers **with** reset.
    #[endpoint]
    fn forward_sync_retrieve_funds_bt_multi_twice_reset(
        &self,
        to: ManagedAddress,
        transfers: MultiValueEncoded<EgldOrEsdtTokenPaymentMultiValue>,
    ) {
        self.tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .retrieve_funds_multi(&transfers)
            .sync_call();

        let back_transfers = self
            .tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .retrieve_funds_multi(&transfers)
            .returns(ReturnsBackTransfersReset)
            .sync_call();

        self.back_transfers_multi_event(back_transfers.into_multi_value());
    }

    #[event("back_transfers_multi_event")]
    fn back_transfers_multi_event(
        &self,
        #[indexed] back_transfers: MultiValueEncoded<EgldOrEsdtTokenPaymentMultiValue>,
    );

    #[event("back_transfers_egld_event")]
    fn back_transfers_egld_event(&self, #[indexed] egld_value: BigUint);

    #[event]
    fn balances_after(
        &self,
        #[indexed] balances_after: MultiValueEncoded<EgldOrEsdtTokenPaymentMultiValue>,
    );
}
