elrond_wasm::imports!();

const PERCENTAGE_TOTAL: u64 = 10_000; // 100%

#[elrond_wasm::module]
pub trait ForwarderTransferExecuteModule {
    #[proxy]
    fn vault_proxy(&self) -> vault::Proxy<Self::Api>;

    #[endpoint]
    #[payable("*")]
    fn forward_transf_exec_accept_funds(&self, to: ManagedAddress) {
        let (token, token_nonce, payment) = self.call_value().payment_as_tuple();
        self.vault_proxy()
            .contract(to)
            .accept_funds()
            .add_token_transfer(token, token_nonce, payment)
            .transfer_execute();
    }

    #[endpoint]
    #[payable("*")]
    fn forward_transf_execu_accept_funds_with_fees(
        &self,
        to: ManagedAddress,
        percentage_fees: BigUint,
    ) {
        let (token_id, payment) = self.call_value().single_fungible_esdt_or_egld_payment();
        let fees = &payment * &percentage_fees / PERCENTAGE_TOTAL;
        let amount_to_send = payment - fees;

        self.vault_proxy()
            .contract(to)
            .accept_funds()
            .add_token_transfer(token_id, 0, amount_to_send)
            .transfer_execute();
    }

    #[endpoint]
    #[payable("*")]
    fn forward_transf_exec_accept_funds_twice(&self, to: ManagedAddress) {
        let (token, token_nonce, payment) = self.call_value().payment_as_tuple();
        let half_payment = payment / 2u32;
        let half_gas = self.blockchain().get_gas_left() / 2;

        self.vault_proxy()
            .contract(to.clone())
            .accept_funds()
            .add_token_transfer(token.clone(), token_nonce, half_payment.clone())
            .with_gas_limit(half_gas)
            .transfer_execute();

        self.vault_proxy()
            .contract(to)
            .accept_funds()
            .add_token_transfer(token, token_nonce, half_payment)
            .with_gas_limit(half_gas)
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
        let (token, token_nonce, payment) = self.call_value().payment_as_tuple();
        let gas_left_before = self.blockchain().get_gas_left();

        self.vault_proxy()
            .contract(to)
            .accept_funds()
            .add_token_transfer(token.clone(), token_nonce, payment)
            .transfer_execute();

        let gas_left_after = self.blockchain().get_gas_left();

        (gas_left_before, gas_left_after, BigUint::zero(), token).into()
    }

    #[endpoint]
    fn transf_exec_multi_accept_funds(
        &self,
        to: ManagedAddress,
        token_payments: MultiValueEncoded<MultiValue3<TokenIdentifier, u64, BigUint>>,
    ) {
        let mut all_token_payments = ManagedVec::new();

        for multi_arg in token_payments.into_iter() {
            let (token_identifier, token_nonce, amount) = multi_arg.into_tuple();
            let payment = EsdtTokenPayment::new(token_identifier, token_nonce, amount);

            all_token_payments.push(payment);
        }

        self.vault_proxy()
            .contract(to)
            .accept_funds()
            .with_multi_token_transfer(all_token_payments)
            .transfer_execute()
    }

    #[endpoint]
    fn forward_transf_exec_reject_funds_multi_transfer(
        &self,
        to: ManagedAddress,
        token_payments: MultiValueEncoded<MultiValue3<TokenIdentifier, u64, BigUint>>,
    ) {
        let mut all_token_payments = ManagedVec::new();

        for multi_arg in token_payments.into_iter() {
            let (token_identifier, token_nonce, amount) = multi_arg.into_tuple();
            let payment = EsdtTokenPayment::new(token_identifier, token_nonce, amount);

            all_token_payments.push(payment);
        }

        self.vault_proxy()
            .contract(to)
            .accept_funds()
            .with_multi_token_transfer(all_token_payments)
            .transfer_execute()
    }

    #[endpoint]
    fn transf_exec_multi_reject_funds(
        &self,
        to: ManagedAddress,
        token_payments: MultiValueEncoded<MultiValue3<TokenIdentifier, u64, BigUint>>,
    ) {
        let mut all_token_payments = ManagedVec::new();

        for multi_arg in token_payments.into_iter() {
            let (token_identifier, token_nonce, amount) = multi_arg.into_tuple();
            let payment = EsdtTokenPayment::new(token_identifier, token_nonce, amount);

            all_token_payments.push(payment);
        }

        self.vault_proxy()
            .contract(to)
            .reject_funds()
            .with_multi_token_transfer(all_token_payments)
            .transfer_execute()
    }
}
