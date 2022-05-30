elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct CallbackData<M: ManagedTypeApi> {
    callback_name: ManagedBuffer<M>,
    token_identifier: EgldOrEsdtTokenIdentifier<M>,
    token_nonce: u64,
    token_amount: BigUint<M>,
    args: ManagedVec<M, ManagedBuffer<M>>,
}

const PERCENTAGE_TOTAL: u64 = 10_000; // 100%

#[elrond_wasm::module]
pub trait ForwarderAsyncCallModule {
    #[proxy]
    fn vault_proxy(&self) -> vault::Proxy<Self::Api>;

    #[endpoint]
    #[payable("*")]
    fn forward_async_accept_funds(&self, to: ManagedAddress) {
        let (token, token_nonce, payment) = self.call_value().payment_as_tuple();
        self.vault_proxy()
            .contract(to)
            .accept_funds()
            .add_token_transfer(token, token_nonce, payment)
            .async_call()
            .call_and_exit()
    }

    #[endpoint]
    #[payable("*")]
    fn forward_async_accept_funds_half_payment(&self, to: ManagedAddress) {
        let payment = self.call_value().egld_or_single_esdt();
        let half_payment = payment.amount / 2u32;
        self.vault_proxy()
            .contract(to)
            .accept_funds()
            .add_token_transfer(payment.token_identifier, 0, half_payment)
            .async_call()
            .call_and_exit()
    }

    #[payable("*")]
    #[endpoint]
    fn forward_async_accept_funds_with_fees(&self, to: ManagedAddress, percentage_fees: BigUint) {
        let payment = self.call_value().egld_or_single_esdt();
        let fees = &payment.amount * &percentage_fees / PERCENTAGE_TOTAL;
        let amount_to_send = payment.amount - fees;

        self.vault_proxy()
            .contract(to)
            .accept_funds()
            .add_token_transfer(payment.token_identifier, 0, amount_to_send)
            .async_call()
            .call_and_exit()
    }

    #[endpoint]
    fn forward_async_retrieve_funds(
        &self,
        to: ManagedAddress,
        token: EgldOrEsdtTokenIdentifier,
        token_nonce: u64,
        amount: BigUint,
    ) {
        self.vault_proxy()
            .contract(to)
            .retrieve_funds(
                token,
                token_nonce,
                amount,
                OptionalValue::<ManagedBuffer>::None,
            )
            .async_call()
            .with_callback(self.callbacks().retrieve_funds_callback())
            .call_and_exit()
    }

    #[callback]
    fn retrieve_funds_callback(&self) {
        let (token, nonce, payment) = self.call_value().payment_as_tuple();
        self.retrieve_funds_callback_event(&token, nonce, &payment);

        let _ = self.callback_data().push(&CallbackData {
            callback_name: ManagedBuffer::from(b"retrieve_funds_callback"),
            token_identifier: token,
            token_nonce: nonce,
            token_amount: payment,
            args: ManagedVec::new(),
        });
    }

    #[event("retrieve_funds_callback")]
    fn retrieve_funds_callback_event(
        &self,
        #[indexed] token: &EgldOrEsdtTokenIdentifier,
        #[indexed] nonce: u64,
        #[indexed] payment: &BigUint,
    );

    #[endpoint]
    fn send_funds_twice(
        &self,
        to: &ManagedAddress,
        token_identifier: &EgldOrEsdtTokenIdentifier,
        amount: &BigUint,
    ) {
        self.vault_proxy()
            .contract(to.clone())
            .accept_funds()
            .add_token_transfer(token_identifier.clone(), 0, amount.clone())
            .async_call()
            .with_callback(
                self.callbacks()
                    .send_funds_twice_callback(to, token_identifier, amount),
            )
            .call_and_exit()
    }

    #[callback]
    fn send_funds_twice_callback(
        &self,
        to: &ManagedAddress,
        token_identifier: &EgldOrEsdtTokenIdentifier,
        cb_amount: &BigUint,
    ) {
        self.vault_proxy()
            .contract(to.clone())
            .accept_funds()
            .add_token_transfer(token_identifier.clone(), 0, cb_amount.clone())
            .async_call()
            .call_and_exit()
    }

    #[endpoint]
    fn send_async_accept_multi_transfer(
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

        // TODO: use proxy here
        self.send().transfer_multiple_esdt_via_async_call(
            &to,
            &all_token_payments,
            b"accept_funds",
        );
    }

    #[view]
    #[storage_mapper("callback_data")]
    fn callback_data(&self) -> VecMapper<CallbackData<Self::Api>>;

    #[view]
    fn callback_data_at_index(
        &self,
        index: usize,
    ) -> MultiValue5<
        ManagedBuffer,
        EgldOrEsdtTokenIdentifier,
        u64,
        BigUint,
        MultiValueManagedVec<Self::Api, ManagedBuffer>,
    > {
        let cb_data = self.callback_data().get(index);
        (
            cb_data.callback_name,
            cb_data.token_identifier,
            cb_data.token_nonce,
            cb_data.token_amount,
            cb_data.args.into(),
        )
            .into()
    }

    #[endpoint]
    fn clear_callback_data(&self) {
        self.callback_data().clear();
    }
}
