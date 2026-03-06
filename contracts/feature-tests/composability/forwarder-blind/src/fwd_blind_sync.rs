multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ForwarderBlindSync: super::fwd_blind_common::ForwarderBlindCommon {
    #[endpoint]
    #[payable]
    fn blind_sync_call(
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

        if !back_transfers.is_empty() {
            self.tx()
                .to(self.blockchain().get_caller())
                .payment(back_transfers.into_payment_vec())
                .transfer();
        }

        self.sync_ok(raw_results.into());
    }

    #[event("blind_sync_ok")]
    fn sync_ok(&self, #[indexed] results: MultiValueEncoded<ManagedBuffer>);
}
