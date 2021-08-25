elrond_wasm::imports!();

const PERCENTAGE_TOTAL: u64 = 10_000; // 100%

#[elrond_wasm::module]
pub trait ForwarderSyncCallModule {
    #[proxy]
    fn vault_proxy(&self) -> vault::Proxy<Self::SendApi>;

    #[endpoint]
    #[payable("*")]
    fn echo_arguments_sync(&self, to: Address, #[var_args] args: VarArgs<BoxedBytes>) {
        let half_gas = self.blockchain().get_gas_left() / 2;

        let result = self
            .vault_proxy()
            .contract(to)
            .echo_arguments(args)
            .with_gas_limit(half_gas)
            .execute_on_dest_context();

        self.execute_on_dest_context_result_event(result.as_slice());
    }

    #[endpoint]
    #[payable("*")]
    fn echo_arguments_sync_range(
        &self,
        to: Address,
        start: usize,
        end: usize,
        #[var_args] args: VarArgs<BoxedBytes>,
    ) {
        let half_gas = self.blockchain().get_gas_left() / 2;

        let result = self
            .vault_proxy()
            .contract(to)
            .echo_arguments(args)
            .with_gas_limit(half_gas)
            .execute_on_dest_context_custom_range(|_, _| (start, end));

        self.execute_on_dest_context_result_event(result.as_slice());
    }

    #[endpoint]
    #[payable("*")]
    fn echo_arguments_sync_twice(&self, to: Address, #[var_args] args: VarArgs<BoxedBytes>) {
        let one_third_gas = self.blockchain().get_gas_left() / 3;

        let result = self
            .vault_proxy()
            .contract(to.clone())
            .echo_arguments(args.clone())
            .with_gas_limit(one_third_gas)
            .execute_on_dest_context();

        self.execute_on_dest_context_result_event(result.as_slice());

        let result = self
            .vault_proxy()
            .contract(to)
            .echo_arguments(args)
            .with_gas_limit(one_third_gas)
            .execute_on_dest_context();

        self.execute_on_dest_context_result_event(result.as_slice());
    }

    #[event("echo_arguments_sync_result")]
    fn execute_on_dest_context_result_event(&self, result: &[BoxedBytes]);

    #[endpoint]
    #[payable("*")]
    fn forward_sync_accept_funds(
        &self,
        to: Address,
        #[payment_token] token: TokenIdentifier,
        #[payment_amount] payment: BigUint,
        #[payment_nonce] token_nonce: u64,
    ) {
        let half_gas = self.blockchain().get_gas_left() / 2;

        let result: MultiResult4<TokenIdentifier, BoxedBytes, BigUint, u64> = self
            .vault_proxy()
            .contract(to)
            .accept_funds_echo_payment(token, payment, token_nonce)
            .with_gas_limit(half_gas)
            .execute_on_dest_context();

        let (token_identifier, token_type_str, token_payment, token_nonce) = result.into_tuple();
        self.accept_funds_sync_result_event(
            &token_identifier,
            token_type_str.as_slice(),
            &token_payment,
            token_nonce,
        );
    }

    #[payable("*")]
    #[endpoint]
    fn forward_sync_accept_funds_with_fees(
        &self,
        #[payment_token] token_id: TokenIdentifier,
        #[payment_amount] payment: BigUint,
        to: Address,
        percentage_fees: BigUint,
    ) {
        let fees = &payment * &percentage_fees / PERCENTAGE_TOTAL.into();
        let amount_to_send = payment - fees;

        self.vault_proxy()
            .contract(to)
            .accept_funds(token_id, amount_to_send)
            .execute_on_dest_context();
    }

    #[event("accept_funds_sync_result")]
    fn accept_funds_sync_result_event(
        &self,
        #[indexed] token_identifier: &TokenIdentifier,
        #[indexed] token_type: &[u8],
        #[indexed] token_payment: &BigUint,
        #[indexed] token_nonce: u64,
    );

    #[endpoint]
    #[payable("*")]
    fn forward_sync_accept_funds_then_read(
        &self,
        to: Address,
        #[payment_token] token: TokenIdentifier,
        #[payment_amount] payment: BigUint,
        #[payment_nonce] token_nonce: u64,
    ) -> usize {
        let _ = self
            .vault_proxy()
            .contract(to.clone())
            .with_nft_nonce(token_nonce)
            .accept_funds(token, payment)
            .execute_on_dest_context();

        self.vault_proxy()
            .contract(to)
            .call_counts(b"accept_funds")
            .execute_on_dest_context()
    }

    #[endpoint]
    fn forward_sync_retrieve_funds(
        &self,
        to: Address,
        token: TokenIdentifier,
        token_nonce: u64,
        amount: BigUint,
    ) {
        self.vault_proxy()
            .contract(to)
            .retrieve_funds(token, token_nonce, amount, OptionalArg::None)
            .execute_on_dest_context()
    }

    #[endpoint]
    fn forward_sync_accept_funds_multi_transfer(
        &self,
        to: Address,
        #[var_args] token_payments: VarArgs<MultiArg3<TokenIdentifier, u64, BigUint>>,
    ) {
        let mut all_token_payments = Vec::new();

        for multi_arg in token_payments.into_vec() {
            let (token_name, token_nonce, amount) = multi_arg.into_tuple();
            let payment = EsdtTokenPayment::from(token_name, token_nonce, amount);
            all_token_payments.push(payment);
        }

        self.vault_proxy()
            .contract(to)
            .accept_funds_multi_transfer()
            .with_multi_token_transfer(self.call_value().get_all_esdt_transfers())
            .execute_on_dest_context();
    }
}
