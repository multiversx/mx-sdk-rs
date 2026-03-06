use crate::vault_proxy;

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ForwarderFallibleModule {
    #[endpoint]
    fn sync_call_fallible(
        &self,
        to: ManagedAddress,
        endpoint_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let half_gas = self.blockchain().get_gas_left() / 2;
        let result = self
            .tx()
            .to(&to)
            .gas(half_gas)
            .raw_call(endpoint_name)
            .arguments_raw(args.into_arg_buffer())
            .returns(ReturnsHandledOrError::new().returns(ReturnsRawResult))
            .sync_call_fallible();

        match result {
            Ok(success) => {
                self.sync_call_fallible_success(success);
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
        let result = self
            .tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .accept_funds()
            .payment(payment_args.convert_payment_multi_triples())
            .returns(ReturnsHandledOrError::new())
            .sync_call_fallible();

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
            .tx()
            .to(&to)
            .gas(half_gas)
            .typed(vault_proxy::VaultProxy)
            .reject_funds()
            .payment(payment_args.convert_payment_multi_triples())
            .returns(ReturnsHandledOrError::new())
            .sync_call_fallible();

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

    #[endpoint]
    fn transfer_fallible(
        &self,
        to: ManagedAddress,
        payments: MultiValueEncoded<MultiValue3<EgldOrEsdtTokenIdentifier, u64, BigUint>>,
    ) -> bool {
        self.tx()
            .to(&to)
            .payment(payments.convert_payment_multi_triples())
            .transfer_fallible()
            .is_ok()
    }

    /// Receiver needs to be an endpoint with no arguments, for simplicity.
    #[endpoint]
    fn transfer_execute_fallible(
        &self,
        to: ManagedAddress,
        endpoint_name: ManagedBuffer,
        payments: MultiValueEncoded<MultiValue3<EgldOrEsdtTokenIdentifier, u64, BigUint>>,
    ) -> bool {
        let half_gas = self.blockchain().get_gas_left() / 2;
        self.tx()
            .to(&to)
            .payment(payments.convert_payment_multi_triples())
            .gas(half_gas)
            .raw_call(endpoint_name)
            .transfer_execute_fallible()
            .is_ok()
    }
}
