multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ForwarderFallibleModule {
    #[proxy]
    fn vault_proxy(&self) -> vault::Proxy<Self::Api>;

    #[endpoint]
    fn sync_call_fallible(
        &self,
        to: ManagedAddress,
        endpoint_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let half_gas = self.blockchain().get_gas_left() / 2;

        let result: Result<MultiValueEncoded<ManagedBuffer>, u32> = self
            .tx()
            .to(&to)
            .gas(half_gas)
            .raw_call(endpoint_name)
            .arguments_raw(args.to_arg_buffer())
            .execute_on_dest_context_fallible();

        match result {
            Ok(success) => {
                self.sync_call_fallible_success(success.into_vec_of_buffers());
            }
            Err(error_code) => {
                self.sync_call_fallible_error(error_code);
            }
        }
    }

    #[endpoint]
    fn forward_sync_fallible_accept_funds_multi_transfer(
        &self,
        to: ManagedAddress,
        payment_args: MultiValueEncoded<MultiValue3<EgldOrEsdtTokenIdentifier, u64, BigUint>>,
    ) -> bool {
        let result: Result<(), u32> = self
            .vault_proxy()
            .contract(to)
            .accept_funds()
            .payment(payment_args.convert_payment_multi_triples())
            .execute_on_dest_context_fallible();

        self.log_result(result);

        result.is_ok()
    }

    #[endpoint]
    fn forward_sync_reject_funds_multi_transfer(
        &self,
        to: ManagedAddress,
        payment_args: MultiValueEncoded<MultiValue3<EgldOrEsdtTokenIdentifier, u64, BigUint>>,
    ) -> bool {
        let half_gas = self.blockchain().get_gas_left() / 2;

        let result = self
            .vault_proxy()
            .contract(to)
            .reject_funds()
            .gas(half_gas)
            .payment(payment_args.convert_payment_multi_triples())
            .execute_on_dest_context_fallible::<()>();

        self.log_result(result);

        result.is_ok()
    }

    fn log_result(&self, result: Result<(), u32>) {
        match result {
            Ok(()) => {
                self.sync_call_fallible_success(ManagedVec::new());
            }
            Err(error_code) => {
                self.sync_call_fallible_error(error_code);
            }
        }
    }

    #[event("sync_call_fallible_success")]
    fn sync_call_fallible_success(&self, result: ManagedVec<Self::Api, ManagedBuffer>);

    #[event("sync_call_fallible_error")]
    fn sync_call_fallible_error(&self, error_code: u32);
}
