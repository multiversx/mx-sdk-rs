use crate::vault_proxy;

multiversx_sc::imports!();

/// Not directly related to promises, but this contract already has the setup for VM 1.5.
#[multiversx_sc::module]
pub trait BackTransfersModule {
    #[endpoint]
    fn forward_sync_retrieve_funds_bt_multi(
        &self,
        to: ManagedAddress,
        transfers: MultiValueEncoded<EgldOrEsdtTokenPaymentMultiValue>,
    ) {
        let back_transfers = self
            .tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .retrieve_funds_multi(&transfers)
            .returns(ReturnsBackTransfersMulti)
            .sync_call();

        self.back_transfers_multi_event(MultiValueEncoded::from_vec(back_transfers));
    }

    #[endpoint]
    fn forward_sync_retrieve_funds_bt_multi_reset_twice(
        &self,
        to: ManagedAddress,
        transfers: MultiValueEncoded<EgldOrEsdtTokenPaymentMultiValue>,
    ) {
        let back_transfers = self
            .tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .retrieve_funds_multi(&transfers)
            .returns(ReturnsBackTransfersMultiReset)
            .sync_call();

        self.back_transfers_multi_event(MultiValueEncoded::from_vec(back_transfers));

        let back_transfers = self
            .tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .retrieve_funds_multi(&transfers)
            .returns(ReturnsBackTransfersMultiReset)
            .sync_call();

        self.back_transfers_multi_event(MultiValueEncoded::from_vec(back_transfers));
    }

    #[endpoint]
    fn forward_sync_retrieve_funds_bt_multi_twice(
        &self,
        to: ManagedAddress,
        transfers: MultiValueEncoded<EgldOrEsdtTokenPaymentMultiValue>,
    ) {
        let back_transfers = self
            .tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .retrieve_funds_multi(&transfers)
            .returns(ReturnsBackTransfersMulti)
            .sync_call();

        self.back_transfers_multi_event(MultiValueEncoded::from_vec(back_transfers));

        let back_transfers = self
            .tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .retrieve_funds_multi(&transfers)
            .returns(ReturnsBackTransfersMulti)
            .sync_call();

        self.back_transfers_multi_event(MultiValueEncoded::from_vec(back_transfers));
    }

    #[event("back_transfers_multi")]
    fn back_transfers_multi_event(
        &self,
        #[indexed] back_transfers: MultiValueEncoded<EgldOrEsdtTokenPaymentMultiValue>,
    );
}
