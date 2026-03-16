multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ForwarderBlindTransferExecute: super::fwd_blind_common::ForwarderBlindCommon {
    #[endpoint(blindTransfExec)]
    #[payable]
    fn blind_transf_exec(
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
            .gas(self.tx_gas())
            .transfer_execute();
    }
}
