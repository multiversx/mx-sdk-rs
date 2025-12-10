multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ForwarderRawAsync: super::forwarder_raw_common::ForwarderRawCommon {
    #[endpoint]
    #[payable("*")]
    fn forward_async_call(
        &self,
        to: ManagedAddress,
        endpoint_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let opt_payment = self.call_value().single_optional();
        self.tx()
            .to(to)
            .raw_call(endpoint_name)
            .arguments_raw(args.to_arg_buffer())
            .payment(opt_payment)
            .async_call_and_exit()
    }

    #[endpoint]
    #[payable("*")]
    fn forward_async_call_half_payment(
        &self,
        to: ManagedAddress,
        endpoint_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let payment = self.call_value().single();
        self.tx()
            .to(to)
            .raw_call(endpoint_name)
            .arguments_raw(args.to_arg_buffer())
            .payment(PaymentRefs::new(
                &payment.token_identifier,
                0u64,
                &(&payment.amount / 2u32),
            ))
            .async_call_and_exit()
    }

    #[endpoint]
    fn forward_async_retrieve_multi_transfer_funds(
        &self,
        to: ManagedAddress,
        token_payments: MultiValueEncoded<MultiValue3<EsdtTokenIdentifier, u64, BigUint>>,
    ) {
        let mut arg_buffer = ManagedArgBuffer::new();
        for multi_arg in token_payments.into_iter() {
            let (token_identifier, token_nonce, amount) = multi_arg.into_tuple();

            arg_buffer.push_arg(token_identifier);
            arg_buffer.push_arg(token_nonce);
            arg_buffer.push_arg(amount);
        }

        self.tx()
            .to(&to)
            .raw_call("retrieve_funds_multi")
            .arguments_raw(arg_buffer)
            .async_call_and_exit();
    }

    #[endpoint]
    fn forwarder_async_send_and_retrieve_multi_transfer_funds(
        &self,
        to: ManagedAddress,
        payment_args: MultiValueEncoded<MultiValue3<EsdtTokenIdentifier, u64, BigUint>>,
    ) {
        self.tx()
            .raw_call("burn_and_create_retrieve_async")
            .to(&to)
            .payment(&payment_args.convert_payment_multi_triples())
            .async_call_and_exit()
    }
}
