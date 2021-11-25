elrond_wasm::imports!();

const PERCENTAGE_TOTAL: u64 = 10_000; // 100%

#[elrond_wasm::module]
pub trait ForwarderSyncCallModule {
    #[proxy]
    fn vault_proxy(&self) -> vault::Proxy<Self::Api>;

    #[endpoint]
    #[payable("*")]
    fn echo_arguments_sync(
        &self,
        to: ManagedAddress,
        #[var_args] args: ManagedVarArgs<ManagedBuffer>,
    ) {
        let half_gas = self.blockchain().get_gas_left() / 2;

        let result = self
            .vault_proxy()
            .contract(to)
            .echo_arguments(args)
            .with_gas_limit(half_gas)
            .execute_on_dest_context();

        self.execute_on_dest_context_result_event(&result.into_vec_of_buffers());
    }

    #[endpoint]
    #[payable("*")]
    fn echo_arguments_sync_range(
        &self,
        to: ManagedAddress,
        start: usize,
        end: usize,
        #[var_args] args: ManagedVarArgs<ManagedBuffer>,
    ) {
        let half_gas = self.blockchain().get_gas_left() / 2;

        let result = self
            .vault_proxy()
            .contract(to)
            .echo_arguments(args)
            .with_gas_limit(half_gas)
            .execute_on_dest_context_custom_range(|_, _| (start, end));

        self.execute_on_dest_context_result_event(&result.into_vec_of_buffers());
    }

    #[endpoint]
    #[payable("*")]
    fn echo_arguments_sync_twice(
        &self,
        to: ManagedAddress,
        #[var_args] args: ManagedVarArgs<ManagedBuffer>,
    ) {
        let one_third_gas = self.blockchain().get_gas_left() / 3;

        let result = self
            .vault_proxy()
            .contract(to.clone())
            .echo_arguments(args.clone())
            .with_gas_limit(one_third_gas)
            .execute_on_dest_context();

        self.execute_on_dest_context_result_event(&result.into_vec_of_buffers());

        let result = self
            .vault_proxy()
            .contract(to)
            .echo_arguments(args)
            .with_gas_limit(one_third_gas)
            .execute_on_dest_context();

        self.execute_on_dest_context_result_event(&result.into_vec_of_buffers());
    }

    #[event("echo_arguments_sync_result")]
    fn execute_on_dest_context_result_event(&self, result: &ManagedVec<Self::Api, ManagedBuffer>);

    #[endpoint]
    #[payable("*")]
    fn forward_sync_accept_funds(
        &self,
        to: ManagedAddress,
        #[payment_token] token: TokenIdentifier,
        #[payment_amount] payment: BigUint,
        #[payment_nonce] token_nonce: u64,
    ) {
        let half_gas = self.blockchain().get_gas_left() / 2;

        let result: MultiResult4<TokenIdentifier, ManagedBuffer, BigUint, u64> = self
            .vault_proxy()
            .contract(to)
            .accept_funds_echo_payment(token, payment, token_nonce)
            .with_gas_limit(half_gas)
            .execute_on_dest_context();

        let (token_identifier, token_type_str, token_payment, token_nonce) = result.into_tuple();
        self.accept_funds_sync_result_event(
            &token_identifier,
            &token_type_str,
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
        to: ManagedAddress,
        percentage_fees: BigUint,
    ) {
        let fees = &payment * &percentage_fees / PERCENTAGE_TOTAL;
        let amount_to_send = payment - fees;

        self.vault_proxy()
            .contract(to)
            .accept_funds(token_id, 0, amount_to_send)
            .execute_on_dest_context();
    }

    #[event("accept_funds_sync_result")]
    fn accept_funds_sync_result_event(
        &self,
        #[indexed] token_identifier: &TokenIdentifier,
        #[indexed] token_type: &ManagedBuffer,
        #[indexed] token_payment: &BigUint,
        #[indexed] token_nonce: u64,
    );

    #[endpoint]
    #[payable("*")]
    fn forward_sync_accept_funds_then_read(
        &self,
        to: ManagedAddress,
        #[payment_token] token: TokenIdentifier,
        #[payment_amount] payment: BigUint,
        #[payment_nonce] token_nonce: u64,
    ) -> usize {
        let _ = self
            .vault_proxy()
            .contract(to.clone())
            .accept_funds(token, token_nonce, payment)
            .execute_on_dest_context();

        self.vault_proxy()
            .contract(to)
            .call_counts(b"accept_funds")
            .execute_on_dest_context()
    }

    #[endpoint]
    fn forward_sync_retrieve_funds(
        &self,
        to: ManagedAddress,
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
        to: ManagedAddress,
        #[var_args] token_payments: ManagedVarArgs<MultiArg3<TokenIdentifier, u64, BigUint>>,
    ) {
        let mut all_token_payments = ManagedVec::new();

        for multi_arg in token_payments.into_iter() {
            let (token_identifier, token_nonce, amount) = multi_arg.into_tuple();
            let payment = EsdtTokenPayment::new(token_identifier, token_nonce, amount);
            all_token_payments.push(payment);
        }

        self.vault_proxy()
            .contract(to)
            .accept_funds_multi_transfer()
            .with_multi_token_transfer(all_token_payments)
            .execute_on_dest_context();
    }
}
