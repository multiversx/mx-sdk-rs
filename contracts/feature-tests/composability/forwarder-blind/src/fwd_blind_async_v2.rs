use crate::fwd_blind_common::GAS_OVERHEAD;

multiversx_sc::imports!();

const ASYNC_V2_CALLBACK_GAS: u64 = 3_000_000;

#[multiversx_sc::module]
pub trait ForwarderBlindAsyncV2: super::fwd_blind_common::ForwarderBlindCommon {
    #[endpoint(blindAsyncV2)]
    #[payable]
    fn blind_async_v2(
        &self,
        to: ManagedAddress,
        endpoint_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let original_caller = self.blockchain().get_caller();
        let payment = self.call_value().all();

        const RESERVED_GAS: u64 = GAS_OVERHEAD + ASYNC_V2_CALLBACK_GAS;
        require!(
            self.blockchain().get_gas_left() > RESERVED_GAS,
            "not enough gas for forwarding with async callback"
        );
        let fwd_gas = self.blockchain().get_gas_left() - RESERVED_GAS;

        self.tx()
            .to(to)
            .raw_call(endpoint_name)
            .arguments_raw(args.to_arg_buffer())
            .payment(&payment)
            .gas(fwd_gas)
            .callback(
                self.callbacks()
                    .blind_async_v2_callback(original_caller, &payment),
            )
            .gas_for_callback(ASYNC_V2_CALLBACK_GAS)
            .register_promise();
    }

    #[promises_callback]
    fn blind_async_v2_callback(
        &self,
        original_caller: ManagedAddress,
        original_payment: &PaymentVec,
        #[call_result] result: ManagedAsyncCallResult<MultiValueEncoded<ManagedBuffer>>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(results) => {
                self.async_v2_callback_ok_event(&results);
                let back_payments = self.call_value().all();
                self.send_back_payments(&original_caller, &back_payments);
            }
            ManagedAsyncCallResult::Err(err) => {
                self.async_v2_callback_error_event(err.err_code, &err.err_msg);
                self.send_back_payments(&original_caller, original_payment);
            }
        }
    }

    #[event("blindAsyncV2CallbackOk")]
    fn async_v2_callback_ok_event(&self, #[indexed] results: &MultiValueEncoded<ManagedBuffer>);

    #[event("blindAsyncV2CallbackError")]
    fn async_v2_callback_error_event(&self, #[indexed] err_code: u32, err_msg: &ManagedBuffer);
}
