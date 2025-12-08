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
    fn forward_transf_execu_accept_funds_with_fees(
        &self,
        to: ManagedAddress,
        percentage_fees: BigUint,
    ) {
        let (token_id, payment) = self.call_value().egld_or_single_fungible_esdt();
        let fees = &payment * &percentage_fees / PERCENTAGE_TOTAL;
        let amount_to_send = payment - fees;

        self.tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .accept_funds()
            .egld_or_single_esdt(&token_id, 0u64, &amount_to_send)
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
    ) -> MultiValue4<u64, u64, BigUint, EgldOrEsdtTokenIdentifier> {
        let payment = self.call_value().egld_or_single_esdt();
        let payment_token = payment.token_identifier.clone();
        let gas_left_before = self.blockchain().get_gas_left();

        self.tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .accept_funds()
            .payment(payment)
            .transfer_execute();

        let gas_left_after = self.blockchain().get_gas_left();

        (
            gas_left_before,
            gas_left_after,
            BigUint::zero(),
            payment_token,
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
