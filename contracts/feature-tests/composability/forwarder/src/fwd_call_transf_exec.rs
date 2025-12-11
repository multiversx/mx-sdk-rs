use crate::vault_proxy;

multiversx_sc::imports!();

const PERCENTAGE_TOTAL: u64 = 10_000; // 100%

#[multiversx_sc::module]
pub trait ForwarderTransferExecuteModule {
    #[endpoint]
    #[payable]
    fn forward_transf_exec_accept_funds(&self, to: ManagedAddress) {
        let payment = self.call_value().single_optional();
        self.tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .accept_funds()
            .payment(payment)
            .transfer_execute();
    }

    #[endpoint]
    #[payable("*")]
    fn forward_transf_exec_accept_funds_with_fees(&self, to: ManagedAddress, percentage_fees: u32) {
        let payment = self.call_value().single();
        let fees = &payment.amount * percentage_fees / PERCENTAGE_TOTAL;
        let amount_to_send = &payment.amount - fees;

        self.tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .accept_funds()
            .payment(PaymentRefs::new(
                &payment.token_identifier,
                payment.token_nonce,
                &amount_to_send,
            ))
            .transfer_execute();
    }

    #[endpoint]
    #[payable("*")]
    fn forward_transf_exec_accept_funds_twice(&self, to: ManagedAddress) {
        let payment = self.call_value().single();
        let half_payment = &payment.amount / 2u32;
        let half_gas = self.blockchain().get_gas_left() / 2;

        self.tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .accept_funds()
            .payment(PaymentRefs::new(
                &payment.token_identifier,
                payment.token_nonce,
                &half_payment,
            ))
            .gas(half_gas)
            .transfer_execute();

        self.tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .accept_funds()
            .payment(PaymentRefs::new(
                &payment.token_identifier,
                payment.token_nonce,
                &half_payment,
            ))
            .gas(half_gas)
            .transfer_execute();
    }

    /// Test that the default gas provided to the transfer_execute call
    /// leaves enough in the transaction for finish to happen.
    #[endpoint]
    #[payable("*")]
    fn forward_transf_exec_accept_funds_return_values(
        &self,
        to: ManagedAddress,
    ) -> MultiValue3<u64, u64, TokenId> {
        let payment = self.call_value().single();
        let gas_left_before = self.blockchain().get_gas_left();

        self.tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .accept_funds()
            .payment(&*payment)
            .transfer_execute();

        let gas_left_after = self.blockchain().get_gas_left();

        (
            gas_left_before,
            gas_left_after,
            payment.token_identifier.clone(),
        )
            .into()
    }

    #[endpoint]
    fn transf_exec_multi_accept_funds(
        &self,
        to: ManagedAddress,
        payment_args: MultiValueEncoded<MultiValue3<EgldOrEsdtTokenIdentifier, u64, BigUint>>,
    ) {
        self.tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .accept_funds()
            .payment(payment_args.convert_payment_multi_triples())
            .transfer_execute()
    }

    #[endpoint]
    fn transf_exec_multi_reject_funds(
        &self,
        to: ManagedAddress,
        payment_args: MultiValueEncoded<MultiValue3<EgldOrEsdtTokenIdentifier, u64, BigUint>>,
    ) {
        self.tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .reject_funds()
            .payment(payment_args.convert_payment_multi_triples())
            .transfer_execute()
    }
}
