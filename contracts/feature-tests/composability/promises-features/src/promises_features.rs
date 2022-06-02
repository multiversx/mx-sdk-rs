#![no_std]
#![allow(clippy::type_complexity)]

elrond_wasm::imports!();

/// Test contract for investigating the new async call framework.
#[elrond_wasm::contract]
pub trait PromisesFeatures {
    #[proxy]
    fn vault_proxy(&self) -> vault::Proxy<Self::Api>;

    #[proxy]
    fn self_proxy(&self) -> self::Proxy<Self::Api>;

    #[init]
    fn init(&self) {}

    #[endpoint]
    #[payable("*")]
    fn promise_single_token(
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
            .with_egld_or_single_esdt_token_transfer(
                payment.token_identifier,
                payment.token_nonce,
                payment.amount,
            )
            .with_arguments_raw(args.to_arg_buffer())
            .with_gas_limit(gas_limit)
            .async_call_promise()
            .with_extra_gas_for_callback(extra_gas_for_callback)
            .with_success_callback(b"success_callback")
            .with_error_callback(b"error_callback")
            .register_promise();
    }

    #[endpoint]
    fn promise_multi_transfer(
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
            .with_success_callback(b"success_callback")
            .with_error_callback(b"error_callback")
            .register_promise();
    }

    #[endpoint]
    fn success_callback(&self, args: MultiValueEncoded<ManagedBuffer>) {
        self.async_call_callback_data().set(true);
        let args_as_vec = args.into_vec_of_buffers();
        self.async_call_event_callback(&args_as_vec);
    }

    #[endpoint]
    fn error_callback(&self, args: MultiValueEncoded<ManagedBuffer>) {
        self.async_call_callback_data().set(false);
        let args_as_vec = args.into_vec_of_buffers();
        self.async_call_event_callback(&args_as_vec);
    }

    #[promises_callback]
    fn the_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<()>,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        self.async_call_callback_data().set(false);
        let args_as_vec = args.into_vec_of_buffers();
        self.async_call_event_callback(&args_as_vec);
    }

    #[callback]
    fn legacy_callback(&self, args: MultiValueEncoded<ManagedBuffer>) {
        self.async_call_callback_data().set(false);
        let args_as_vec = args.into_vec_of_buffers();
        self.async_call_event_callback(&args_as_vec);
    }

    #[view]
    #[storage_mapper("async_call_callback_data")]
    fn async_call_callback_data(&self) -> SingleValueMapper<bool>;

    #[event("async_call_event_callback")]
    fn async_call_event_callback(&self, arguments: &ManagedVec<Self::Api, ManagedBuffer>);
}
