multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ForwarderBlindTransferExecute: super::fwd_blind_common::ForwarderBlindCommon {
    #[endpoint(blindTransfExec)]
    #[payable]
    fn blind_transf_exec(
        &self,
        to: ManagedAddress,
        function_call: FunctionCall,
    ) {
        let payment = self.call_value().all();
        self.tx()
            .to(to)
            .raw_call(function_call.function_name)
            .arguments_raw(function_call.arg_buffer)
            .payment(payment)
            .gas(self.tx_gas())
            .transfer_execute();
    }
}
