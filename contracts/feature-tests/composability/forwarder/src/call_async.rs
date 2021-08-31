elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct CallbackData<M: ManagedTypeApi> {
    callback_name: BoxedBytes,
    token_identifier: TokenIdentifier<M>,
    token_nonce: u64,
    token_amount: BigUint<M>,
    args: Vec<BoxedBytes>,
}

const PERCENTAGE_TOTAL: u64 = 10_000; // 100%

#[elrond_wasm::module]
pub trait ForwarderAsyncCallModule {
    #[proxy]
    fn vault_proxy(&self) -> vault::Proxy<Self::SendApi>;

    #[endpoint]
    #[payable("*")]
    fn forward_async_accept_funds(
        &self,
        to: Address,
        #[payment_token] token: TokenIdentifier,
        #[payment_amount] payment: BigUint,
        #[payment_nonce] token_nonce: u64,
    ) -> AsyncCall<Self::SendApi> {
        self.vault_proxy()
            .contract(to)
            .accept_funds(token, payment)
            .with_nft_nonce(token_nonce)
            .async_call()
    }

    #[endpoint]
    #[payable("*")]
    fn forward_async_accept_funds_half_payment(
        &self,
        to: Address,
        #[payment_token] token: TokenIdentifier,
        #[payment] payment: BigUint,
    ) -> AsyncCall<Self::SendApi> {
        let half_payment = payment / 2u32;
        self.vault_proxy()
            .contract(to)
            .accept_funds(token, half_payment)
            .async_call()
    }

    #[payable("*")]
    #[endpoint]
    fn forward_async_accept_funds_with_fees(
        &self,
        #[payment_token] token_id: TokenIdentifier,
        #[payment_amount] payment: BigUint,
        to: Address,
        percentage_fees: BigUint,
    ) -> AsyncCall<Self::SendApi> {
        let fees = &payment * &percentage_fees / PERCENTAGE_TOTAL;
        let amount_to_send = payment - fees;

        self.vault_proxy()
            .contract(to)
            .accept_funds(token_id, amount_to_send)
            .async_call()
    }

    #[endpoint]
    fn forward_async_retrieve_funds(
        &self,
        to: Address,
        token: TokenIdentifier,
        token_nonce: u64,
        amount: BigUint,
    ) -> AsyncCall<Self::SendApi> {
        self.vault_proxy()
            .contract(to)
            .retrieve_funds(token, token_nonce, amount, OptionalArg::None)
            .async_call()
            .with_callback(self.callbacks().retrieve_funds_callback())
    }

    #[callback]
    fn retrieve_funds_callback(
        &self,
        #[payment_token] token: TokenIdentifier,
        #[payment_nonce] nonce: u64,
        #[payment_amount] payment: BigUint,
    ) {
        self.retrieve_funds_callback_event(&token, nonce, &payment);

        let _ = self.callback_data().push(&CallbackData {
            callback_name: BoxedBytes::from(&b"retrieve_funds_callback"[..]),
            token_identifier: token,
            token_nonce: nonce,
            token_amount: payment,
            args: Vec::new(),
        });
    }

    #[event("retrieve_funds_callback")]
    fn retrieve_funds_callback_event(
        &self,
        #[indexed] token: &TokenIdentifier,
        #[indexed] nonce: u64,
        #[indexed] payment: &BigUint,
    );

    #[endpoint]
    fn send_funds_twice(
        &self,
        to: &Address,
        token_identifier: &TokenIdentifier,
        amount: &BigUint,
    ) -> AsyncCall<Self::SendApi> {
        self.vault_proxy()
            .contract(to.clone())
            .accept_funds(token_identifier.clone(), amount.clone())
            .async_call()
            .with_callback(
                self.callbacks()
                    .send_funds_twice_callback(to, token_identifier, amount),
            )
    }

    #[callback]
    fn send_funds_twice_callback(
        &self,
        to: &Address,
        token_identifier: &TokenIdentifier,
        cb_amount: &BigUint,
    ) -> AsyncCall<Self::SendApi> {
        self.vault_proxy()
            .contract(to.clone())
            .accept_funds(token_identifier.clone(), cb_amount.clone())
            .async_call()
    }

    #[endpoint]
    fn multi_transfer_via_async(
        &self,
        to: Address,
        #[var_args] token_payments: VarArgs<MultiArg3<TokenIdentifier, u64, BigUint>>,
    ) {
        let mut all_token_payments = Vec::new();

        for multi_arg in token_payments.into_vec() {
            let (token_name, token_nonce, amount) = multi_arg.into_tuple();
            let payment = EsdtTokenPayment {
                token_name,
                token_nonce,
                amount,
                token_type: EsdtTokenType::Invalid, // not used
            };

            all_token_payments.push(payment);
        }

        self.send().transfer_multiple_esdt_via_async_call(
            &to,
            &all_token_payments,
            b"accept_multi_funds_echo",
        );
    }

    #[view]
    #[storage_mapper("callback_data")]
    fn callback_data(&self) -> VecMapper<Self::Storage, CallbackData<Self::TypeManager>>;

    #[view]
    fn callback_data_at_index(
        &self,
        index: usize,
    ) -> MultiResult5<BoxedBytes, TokenIdentifier, u64, BigUint, MultiResultVec<BoxedBytes>> {
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
