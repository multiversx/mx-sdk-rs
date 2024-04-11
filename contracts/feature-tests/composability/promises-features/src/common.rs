multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct CallbackData<'a, M: ManagedTypeApi<'a>> {
    pub callback_name: ManagedBuffer<'a, M>,
    pub token_identifier: EgldOrEsdtTokenIdentifier<'a, M>,
    pub token_nonce: u64,
    pub token_amount: BigUint<'a, M>,
    pub args: ManagedVec<'a, M, ManagedBuffer<'a, M>>,
}

#[multiversx_sc::module]
pub trait CommonModule {
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
        MultiValueManagedVec<'a, Self::Api, ManagedBuffer>,
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
