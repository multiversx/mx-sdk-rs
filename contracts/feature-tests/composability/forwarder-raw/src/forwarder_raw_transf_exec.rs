multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ForwarderRawTransferExecute {
    #[endpoint]
    #[payable]
    fn forward_direct_transfer(&self, to: ManagedAddress) {
        let payments = self.call_value().all();
        self.tx().to(&to).payment(payments).transfer();
    }

    #[endpoint]
    #[payable]
    fn forward_transf_exec(
        &self,
        to: ManagedAddress,
        endpoint_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let payment = self.call_value().all();
        self.tx()
            .to(to)
            .raw_call(endpoint_name)
            .arguments_raw(args.to_arg_buffer())
            .payment(payment)
            .gas(self.blockchain().get_gas_left() / 2)
            .transfer_execute();
    }
}
