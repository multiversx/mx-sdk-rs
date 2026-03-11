multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ForwarderBlindAsyncV1: super::fwd_blind_common::ForwarderBlindCommon {
    #[endpoint(blindAsyncV1)]
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
            .payment(&payment)
            .callback(
                self.callbacks()
                    .blind_async_v1_callback(original_caller, &payment),
            )
            .async_call_and_exit()
    }

    #[callback]
    fn blind_async_v1_callback(
        &self,
        original_caller: ManagedAddress,
        original_payment: &PaymentVec,
        #[call_result] result: ManagedAsyncCallResult<MultiValueEncoded<ManagedBuffer>>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(results) => {
                self.async_v1_callback_ok_event(&results);
                let back_payments = self.blockchain().get_back_transfers();
                self.send_back_payments(
                    "blindAsyncV1CallbackOk",
                    &original_caller,
                    &back_payments.into_payment_vec(),
                );
            }
            ManagedAsyncCallResult::Err(err) => {
                self.async_v1_callback_error_event(err.err_code, &err.err_msg);
                self.send_back_payments(
                    "blindAsyncV1CallbackError",
                    &original_caller,
                    original_payment,
                );
            }
        }
    }

    #[event("blindAsyncV1CallbackOk")]
    fn async_v1_callback_ok_event(&self, #[indexed] results: &MultiValueEncoded<ManagedBuffer>);

    #[event("blindAsyncV1CallbackError")]
    fn async_v1_callback_error_event(&self, #[indexed] err_code: u32, err_msg: &ManagedBuffer);
}
