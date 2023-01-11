multiversx_sc::imports!();

/// Test contract for investigating the new async call framework.
#[multiversx_sc::module]
pub trait CallPromisesDirectModule {
    #[proxy]
    fn vault_proxy(&self) -> vault::Proxy<Self::Api>;

    #[endpoint]
    #[payable("*")]
    fn promise_raw_single_token(
        &self,
        to: ManagedAddress,
        endpoint_name: ManagedBuffer,
        gas_limit: u64,
        extra_gas_for_callback: u64,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let payment = self.call_value().egld_or_single_esdt();
        self.send()
            .contract_call::<()>(to, endpoint_name)
            .with_egld_or_single_esdt_transfer(payment)
            .with_raw_arguments(args.to_arg_buffer())
            .with_gas_limit(gas_limit)
            .async_call_promise()
            .with_extra_gas_for_callback(extra_gas_for_callback)
            .with_callback(self.callbacks().the_one_callback(1001, 1002))
            .register_promise();
    }

    #[endpoint]
    fn promise_raw_multi_transfer(
        &self,
        to: ManagedAddress,
        endpoint_name: ManagedBuffer,
        extra_gas_for_callback: u64,
        token_payment_args: MultiValueEncoded<EsdtTokenPaymentMultiValue>,
    ) {
        let mut token_payments_vec = ManagedVec::new();
        for token_payment_arg in token_payment_args {
            token_payments_vec.push(token_payment_arg.into_esdt_token_payment());
        }

        let gas_limit = (self.blockchain().get_gas_left() - extra_gas_for_callback) * 9 / 10;

        self.send()
            .contract_call::<()>(to, endpoint_name)
            .with_multi_token_transfer(token_payments_vec)
            .with_gas_limit(gas_limit)
            .async_call_promise()
            .with_extra_gas_for_callback(extra_gas_for_callback)
            .with_callback(self.callbacks().the_one_callback(2001, 2002))
            .register_promise();
    }

    #[promises_callback]
    fn the_one_callback(
        &self,
        #[call_result] result: MultiValueEncoded<ManagedBuffer>,
        arg1: usize,
        arg2: usize,
    ) {
        self.async_call_event_callback(arg1, arg2, &result.into_vec_of_buffers());
    }

    #[event("async_call_event_callback")]
    fn async_call_event_callback(
        &self,
        #[indexed] arg1: usize,
        #[indexed] arg2: usize,
        arguments: &ManagedVec<Self::Api, ManagedBuffer>,
    );
}
