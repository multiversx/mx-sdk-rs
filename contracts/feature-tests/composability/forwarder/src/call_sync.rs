multiversx_sc::imports!();

const PERCENTAGE_TOTAL: u64 = 10_000; // 100%

#[multiversx_sc::module]
pub trait ForwarderSyncCallModule {
    #[proxy]
    fn vault_proxy(&self) -> vault::Proxy<Self::Api>;

    #[endpoint]
    #[payable("*")]
    fn echo_arguments_sync(&self, to: ManagedAddress, args: MultiValueEncoded<ManagedBuffer>) {
        let half_gas = self.blockchain().get_gas_left() / 2;

        let result: MultiValueEncoded<ManagedBuffer> = self
            .vault_proxy()
            .contract(to)
            .echo_arguments(args)
            .with_gas_limit(half_gas)
            .execute_on_dest_context();

        self.execute_on_dest_context_result_event(&result.into_vec_of_buffers());
    }

    #[endpoint]
    #[payable("*")]
    fn echo_arguments_sync_twice(
        &self,
        to: ManagedAddress,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let one_third_gas = self.blockchain().get_gas_left() / 3;

        let result: MultiValueEncoded<ManagedBuffer> = self
            .vault_proxy()
            .contract(to.clone())
            .echo_arguments(&args)
            .with_gas_limit(one_third_gas)
            .execute_on_dest_context();

        self.execute_on_dest_context_result_event(&result.into_vec_of_buffers());

        let result: MultiValueEncoded<ManagedBuffer> = self
            .vault_proxy()
            .contract(to)
            .echo_arguments(&args)
            .with_gas_limit(one_third_gas)
            .execute_on_dest_context();

        self.execute_on_dest_context_result_event(&result.into_vec_of_buffers());
    }

    #[event("echo_arguments_sync_result")]
    fn execute_on_dest_context_result_event(&self, result: &ManagedVec<Self::Api, ManagedBuffer>);

    #[endpoint]
    #[payable("*")]
    fn forward_sync_accept_funds(&self, to: ManagedAddress) {
        let payment = self.call_value().egld_or_single_esdt();
        let half_gas = self.blockchain().get_gas_left() / 2;

        let result: MultiValue2<BigUint, MultiValueEncoded<EsdtTokenPaymentMultiValue>> = self
            .vault_proxy()
            .contract(to)
            .accept_funds_echo_payment()
            .with_egld_or_single_esdt_transfer(payment)
            .with_gas_limit(half_gas)
            .execute_on_dest_context();
        let (egld_value, esdt_transfers_multi) = result.into_tuple();

        self.accept_funds_sync_result_event(&egld_value, &esdt_transfers_multi);
    }

    #[payable("*")]
    #[endpoint]
    fn forward_sync_accept_funds_with_fees(&self, to: ManagedAddress, percentage_fees: BigUint) {
        let (token_id, payment) = self.call_value().egld_or_single_fungible_esdt();
        let fees = &payment * &percentage_fees / PERCENTAGE_TOTAL;
        let amount_to_send = payment - fees;

        let () = self
            .vault_proxy()
            .contract(to)
            .accept_funds()
            .with_egld_or_single_esdt_transfer((token_id, 0, amount_to_send))
            .execute_on_dest_context();
    }

    #[event("accept_funds_sync_result")]
    fn accept_funds_sync_result_event(
        &self,
        #[indexed] egld_value: &BigUint,
        #[indexed] multi_esdt: &MultiValueEncoded<EsdtTokenPaymentMultiValue>,
    );

    #[endpoint]
    #[payable("*")]
    fn forward_sync_accept_funds_then_read(&self, to: ManagedAddress) -> usize {
        let payment = self.call_value().egld_or_single_esdt();
        self.vault_proxy()
            .contract(to.clone())
            .accept_funds()
            .with_egld_or_single_esdt_transfer(payment)
            .execute_on_dest_context::<()>();

        self.vault_proxy()
            .contract(to)
            .call_counts(b"accept_funds")
            .execute_on_dest_context::<SingleValue<usize>>()
            .into()
    }

    #[endpoint]
    fn forward_sync_retrieve_funds(
        &self,
        to: ManagedAddress,
        token: EgldOrEsdtTokenIdentifier,
        token_nonce: u64,
        amount: BigUint,
    ) {
        self.vault_proxy()
            .contract(to)
            .retrieve_funds(token, token_nonce, amount)
            .execute_on_dest_context()
    }

    #[payable("*")]
    #[endpoint]
    fn forward_sync_retrieve_funds_with_accept_func(
        &self,
        to: ManagedAddress,
        token: TokenIdentifier,
        amount: BigUint,
    ) {
        let payments = self.call_value().all_esdt_transfers();

        self.vault_proxy()
            .contract(to)
            .retrieve_funds_with_transfer_exec(
                token,
                amount,
                OptionalValue::<ManagedBuffer>::Some(b"accept_funds_func".into()),
            )
            .with_multi_token_transfer(payments.clone_value())
            .execute_on_dest_context::<()>();
    }

    #[payable("*")]
    #[endpoint]
    fn accept_funds_func(&self) {}

    #[endpoint]
    fn forward_sync_accept_funds_multi_transfer(
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

        let () = self
            .vault_proxy()
            .contract(to)
            .accept_funds()
            .with_multi_token_transfer(all_token_payments)
            .execute_on_dest_context();
    }
}
