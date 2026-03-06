multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ForwarderBlindAsyncV1 {
    #[endpoint]
    #[payable]
    fn blind_async_v1(
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
            .callback(self.callbacks().blind_async_v1_callback(original_caller))
            .async_call_and_exit()
    }

    #[callback]
    fn blind_async_v1_callback(
        &self,
        original_caller: ManagedAddress,
        #[call_result] result: ManagedAsyncCallResult<MultiValueEncoded<ManagedBuffer>>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(results) => {
                let back_payments = self.call_value().all();
                self.async_v1_callback_ok_event(&results);
                if !back_payments.is_empty() {
                    self.tx()
                        .to(original_caller)
                        .payment(back_payments)
                        .transfer();
                }
            }
            ManagedAsyncCallResult::Err(err) => {
                self.async_v1_callback_error_event(err.err_code, &err.err_msg);
            }
        }
    }

    #[event("blind_async_v1_callback_ok")]
    fn async_v1_callback_ok_event(&self, #[indexed] results: &MultiValueEncoded<ManagedBuffer>);

    #[event("blind_async_v1_callback_error")]
    fn async_v1_callback_error_event(&self, #[indexed] err_code: u32, err_msg: &ManagedBuffer);
}
