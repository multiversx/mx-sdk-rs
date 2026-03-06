multiversx_sc::imports!();

const ASYNC_V2_CALLBACK_GAS: u64 = 500_000;

#[multiversx_sc::module]
pub trait ForwarderBlindAsyncV2: super::fwd_blind_common::ForwarderBlindCommon {
    #[endpoint]
    #[payable]
    fn blind_async_v2(
        &self,
        to: ManagedAddress,
        endpoint_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let original_caller = self.blockchain().get_caller();
        let payment = self.call_value().all();
        self.tx()
            .to(to)
            .raw_call(endpoint_name)
            .arguments_raw(args.to_arg_buffer())
            .payment(payment)
            .gas(self.tx_gas() - ASYNC_V2_CALLBACK_GAS)
            .callback(self.callbacks().blind_async_v2_callback(original_caller))
            .gas_for_callback(ASYNC_V2_CALLBACK_GAS)
            .register_promise();
    }

    #[promises_callback]
    fn blind_async_v2_callback(
        &self,
        original_caller: ManagedAddress,
        #[call_result] result: ManagedAsyncCallResult<MultiValueEncoded<ManagedBuffer>>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(results) => {
                let back_payments = self.call_value().all();
                self.async_v2_callback_ok_event(&results);
                if !back_payments.is_empty() {
                    self.tx()
                        .to(original_caller)
                        .payment(back_payments)
                        .transfer();
                }
            }
            ManagedAsyncCallResult::Err(err) => {
                self.async_v2_callback_error_event(err.err_code, &err.err_msg);
            }
        }
    }

    #[event("async_v2_callback_ok")]
    fn async_v2_callback_ok_event(&self, #[indexed] results: &MultiValueEncoded<ManagedBuffer>);

    #[event("async_v2_callback_error")]
    fn async_v2_callback_error_event(&self, #[indexed] err_code: u32, err_msg: &ManagedBuffer);
}
