multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct CallbackData<M: ManagedTypeApi> {
    callback_name: ManagedBuffer<M>,
    token_identifier: EgldOrEsdtTokenIdentifier<M>,
    token_nonce: u64,
    token_amount: BigUint<M>,
    args: ManagedVec<M, ManagedBuffer<M>>,
}

#[multiversx_sc::module]
pub trait CallPromisesModule {
    #[proxy]
    fn vault_proxy(&self) -> vault::Proxy<Self::Api>;

    #[endpoint]
    #[payable("*")]
    fn forward_promise_accept_funds(&self, to: ManagedAddress) {
        let payment = self.call_value().egld_or_single_esdt();
        let gas_limit = self.blockchain().get_gas_left() / 2;
        self.vault_proxy()
            .contract(to)
            .accept_funds()
            .with_egld_or_single_esdt_transfer(payment)
            .with_gas_limit(gas_limit)
            .async_call_promise()
            .register_promise()
    }

    #[endpoint]
    fn forward_promise_retrieve_funds(
        &self,
        to: ManagedAddress,
        token: EgldOrEsdtTokenIdentifier,
        token_nonce: u64,
        amount: BigUint,
    ) {
        let gas_limit = self.blockchain().get_gas_left() - 20_000_000;
        self.vault_proxy()
            .contract(to)
            .retrieve_funds(token, token_nonce, amount)
            .with_gas_limit(gas_limit)
            .async_call_promise()
            .with_callback(self.callbacks().retrieve_funds_callback())
            .with_extra_gas_for_callback(10_000_000)
            .register_promise()
    }

    #[promises_callback]
    fn retrieve_funds_callback(&self) {
        let (token, nonce, payment) = self.call_value().egld_or_single_esdt().into_tuple();
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
