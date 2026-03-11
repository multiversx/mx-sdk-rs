multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ForwarderBlindSync: super::fwd_blind_common::ForwarderBlindCommon {
    #[endpoint(blindSync)]
    #[payable]
    fn blind_sync(
        &self,
        to: ManagedAddress,
        endpoint_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let payment = self.call_value().all();
        let (back_transfers, raw_results) = self
            .tx()
            .to(to)
            .raw_call(endpoint_name)
            .arguments_raw(args.to_arg_buffer())
            .payment(payment)
            .gas(self.tx_gas())
            .returns(ReturnsBackTransfers)
            .returns(ReturnsRawResult)
            .sync_call();

        self.send_back_payments(
            "blindSync",
            &self.blockchain().get_caller(),
            &back_transfers.into_payment_vec(),
        );

        self.sync_ok(raw_results.into());
    }

    #[endpoint(blindSyncFallible)]
    #[payable]
    fn blind_sync_fallible(
        &self,
        to: ManagedAddress,
        endpoint_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let payment = self.call_value().all();
        let result = self
            .tx()
            .to(to)
            .raw_call(endpoint_name)
            .arguments_raw(args.to_arg_buffer())
            .payment(&payment)
            .gas(self.tx_gas())
            .returns(
                ReturnsHandledOrError::new()
                    .returns(ReturnsBackTransfers)
                    .returns(ReturnsRawResult),
            )
            .sync_call_fallible();

        match result {
            Ok((back_transfers, raw_results)) => {
                self.sync_ok(raw_results.into());
                self.send_back_payments(
                    "blindSyncOk",
                    &self.blockchain().get_caller(),
                    &back_transfers.into_payment_vec(),
                );
            }
            Err(err_code) => {
                self.sync_error(err_code);
                self.send_back_payments(
                    "blindSyncError",
                    &self.blockchain().get_caller(),
                    &payment,
                );
            }
        }
    }

    #[event("blindSyncOk")]
    fn sync_ok(&self, #[indexed] results: MultiValueEncoded<ManagedBuffer>);

    #[event("blindSyncError")]
    fn sync_error(&self, #[indexed] err_code: u32);
}
