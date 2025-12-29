multiversx_sc::imports!();

/// Test contract for investigating the new async call framework.
#[multiversx_sc::module]
pub trait CallPromisesDirectModule {
    #[endpoint]
    #[payable("*")]
    fn promise_raw_single_token_to_user(
        &self,
        to: ManagedAddress,
        gas_limit: u64,
        extra_gas_for_callback: u64,
    ) {
        let payment = self.call_value().egld_or_single_esdt();
        self.tx()
            .to(&to)
            .payment(payment)
            .gas(gas_limit)
            .callback(self.callbacks().retrieve_esdt_callback())
            .gas_for_callback(extra_gas_for_callback)
            .register_promise();
    }
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
        self.tx()
            .to(&to)
            .raw_call(endpoint_name)
            .payment(payment)
            .arguments_raw(args.to_arg_buffer())
            .gas(gas_limit)
            .callback(self.callbacks().the_one_callback(1001, 1002u32.into()))
            .gas_for_callback(extra_gas_for_callback)
            .register_promise();
    }

    #[endpoint]
    fn promise_raw_multi_transfer(
        &self,
        to: ManagedAddress,
        endpoint_name: ManagedBuffer,
        extra_gas_for_callback: u64,
        token_payment_args: MultiValueEncoded<PaymentMultiValue>,
    ) {
        let gas_limit = (self.blockchain().get_gas_left() - extra_gas_for_callback) * 9 / 10;

        self.tx()
            .to(&to)
            .raw_call(endpoint_name)
            .payment(MultiTransfer(token_payment_args.convert_payment()))
            .gas(gas_limit)
            .callback(self.callbacks().the_one_callback(2001, 2002u32.into()))
            .gas_for_callback(extra_gas_for_callback)
            .register_promise();
    }

    #[promises_callback]
    fn the_one_callback(
        &self,
        #[call_result] result: MultiValueEncoded<ManagedBuffer>,
        arg1: usize,
        arg2: BigUint,
    ) {
        self.async_call_event_callback(arg1, arg2, &result.into_vec_of_buffers());
    }

    #[promises_callback]
    fn retrieve_esdt_callback(&self) {
        let callback_payment = self.call_value().single_esdt();
        self.async_call_esdt_event_callback(
            callback_payment.token_identifier.clone(),
            callback_payment.amount.clone(),
        );
    }

    #[event("async_call_event_callback")]
    fn async_call_event_callback(
        &self,
        #[indexed] arg1: usize,
        #[indexed] arg2: BigUint,
        arguments: &ManagedVec<Self::Api, ManagedBuffer>,
    );

    #[event("async_call_event_callback")]
    fn async_call_esdt_event_callback(
        &self,
        #[indexed] token_id: EsdtTokenIdentifier,
        #[indexed] amount: BigUint,
    );
}
