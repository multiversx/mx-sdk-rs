#![no_std]
#![allow(clippy::type_complexity)]

use elrond_wasm::api::ESDT_MULTI_TRANSFER_FUNC_NAME;

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
        #[payment_token] token: TokenIdentifier,
        #[payment] payment: BigUint,
        endpoint_name: ManagedBuffer,
        gas_limit: u64,
        extra_gas_for_callback: u64,
        #[var_args] args: MultiValueEncoded<ManagedBuffer>,
    ) {
        self.send()
            .contract_call::<()>(to, endpoint_name)
            .add_token_transfer(token, 0, payment)
            .with_arguments_raw(args.to_arg_buffer())
            .with_gas_limit(gas_limit)
            .with_extra_gas_for_callback(extra_gas_for_callback)
            .with_success_callback(b"success_callback")
            .with_error_callback(b"error_callback")
            .register_promise();
    }

    #[endpoint]
    fn forwarder_multi_transfer_via_promise(
        &self,
        to: ManagedAddress,
        endpoint_name: ManagedBuffer,
        extra_gas_for_callback: u64,
        #[var_args] token_payments: MultiValueEncoded<MultiValue3<TokenIdentifier, u64, BigUint>>,
    ) {
        let mut arg_buffer = ManagedArgBuffer::new_empty();
        arg_buffer.push_arg(to);
        arg_buffer.push_arg(token_payments.raw_len() / 3);

        for multi_arg in token_payments.into_iter() {
            let (token_identifier, token_nonce, amount) = multi_arg.into_tuple();

            arg_buffer.push_arg(token_identifier);
            arg_buffer.push_arg(token_nonce);
            arg_buffer.push_arg(amount);
        }

        if !endpoint_name.is_empty() {
            arg_buffer.push_arg_raw(endpoint_name);
        }

        let gas_limit = (self.blockchain().get_gas_left() - extra_gas_for_callback) * 9 / 10;

        Self::Api::send_api_impl().create_async_call_raw(
            &ManagedAddress::from_raw_handle(
                Self::Api::blockchain_api_impl().get_sc_address_handle(),
            ),
            &BigUint::zero(),
            &ManagedBuffer::new_from_bytes(ESDT_MULTI_TRANSFER_FUNC_NAME),
            b"success_callback",
            b"error_callback",
            gas_limit,
            extra_gas_for_callback,
            &arg_buffer,
        );
    }

    #[endpoint]
    fn success_callback(&self, #[var_args] args: MultiValueEncoded<ManagedBuffer>) {
        self.async_call_callback_data().set(true);
        let args_as_vec = args.into_vec_of_buffers();
        self.async_call_event_callback(&args_as_vec);
    }

    #[endpoint]
    fn error_callback(&self, #[var_args] args: MultiValueEncoded<ManagedBuffer>) {
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
