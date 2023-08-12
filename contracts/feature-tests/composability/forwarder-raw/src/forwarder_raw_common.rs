multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ForwarderRawCommon {
    #[view]
    #[storage_mapper("callback_args")]
    fn callback_args(&self) -> VecMapper<ManagedVec<Self::Api, ManagedBuffer>>;

    #[view]
    #[storage_mapper("callback_payments")]
    fn callback_payments(&self) -> VecMapper<(EgldOrEsdtTokenIdentifier, u64, BigUint)>;

    #[view]
    fn callback_payments_triples(
        &self,
    ) -> MultiValueEncoded<MultiValue3<EgldOrEsdtTokenIdentifier, u64, BigUint>> {
        let mut result = MultiValueEncoded::new();
        for payment_tuple in self.callback_payments().iter() {
            result.push(payment_tuple.into());
        }
        result
    }

    #[endpoint]
    fn clear_callback_info(&self) {
        self.callback_args().clear();
        self.callback_payments().clear();
    }

    /// Used in the elrond-go tests, do not remove.
    #[view]
    fn callback_args_at_index(&self, index: usize) -> MultiValueEncoded<ManagedBuffer> {
        let cb_args = self.callback_args().get(index);
        cb_args.into()
    }

    /// Used in the elrond-go tests, do not remove.
    #[view]
    fn callback_payment_at_index(
        &self,
        index: usize,
    ) -> MultiValue3<EgldOrEsdtTokenIdentifier, u64, BigUint> {
        self.callback_payments().get(index).into()
    }

    #[event("execute_on_dest_context_result")]
    fn execute_on_dest_context_result(&self, result: ManagedVec<Self::Api, ManagedBuffer>);

    #[event("execute_on_same_context_result")]
    fn execute_on_same_context_result(&self, result: ManagedVec<Self::Api, ManagedBuffer>);
}
