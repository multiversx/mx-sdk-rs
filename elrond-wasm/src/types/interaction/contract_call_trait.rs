use elrond_codec::{multi_types::IgnoreValue, TopDecodeMulti, TopEncodeMulti};

use crate::{
    api::CallTypeApi, contract_base::ExitCodecErrorHandler, err_msg, types::ManagedBuffer,
};

use super::{
    contract_call_full::ContractCallFull, AsyncCall, ContractCallNoPayment, ManagedArgBuffer,
};

pub trait ContractCallTrait<SA>: Sized
where
    SA: CallTypeApi + 'static,
{
    type OriginalResult: TopEncodeMulti;

    #[doc(hidden)]
    fn into_contract_call_full(self) -> ContractCallFull<SA, Self::OriginalResult>;

    fn get_mut_basic(&mut self) -> &mut ContractCallNoPayment<SA, Self::OriginalResult>;

    #[doc(hidden)]
    fn proxy_arg<T: TopEncodeMulti>(&mut self, endpoint_arg: &T) {
        let h = ExitCodecErrorHandler::<SA>::from(err_msg::CONTRACT_CALL_ENCODE_ERROR);
        let Ok(()) =
            endpoint_arg.multi_encode_or_handle_err(&mut self.get_mut_basic().arg_buffer, h);
    }

    /// For cases where we build the contract call by hand.
    ///
    /// No serialization occurs, just direct conversion to ManagedBuffer.
    fn push_raw_arg<RawArg: Into<ManagedBuffer<SA>>>(&mut self, raw_arg: RawArg) {
        self.get_mut_basic().arg_buffer.push_arg_raw(raw_arg.into())
    }

    /// For cases where we build the contract call by hand.
    fn with_arguments_raw(mut self, raw_argument_buffer: ManagedArgBuffer<SA>) -> Self {
        self.get_mut_basic().arg_buffer = raw_argument_buffer;
        self
    }

    #[inline]
    fn with_gas_limit(mut self, gas_limit: u64) -> Self {
        self.get_mut_basic().explicit_gas_limit = gas_limit;
        self
    }

    fn async_call(self) -> AsyncCall<SA> {
        self.into_contract_call_full()
            .convert_to_esdt_transfer_call()
            .async_call()
    }

    #[cfg(feature = "promises")]
    fn async_call_promise(self) -> super::AsyncCallPromises<SA> {
        self.into_contract_call_full()
            .convert_to_esdt_transfer_call()
            .async_call_promise()
    }

    /// Executes immediately, synchronously, and returns contract call result.
    /// Only works if the target contract is in the same shard.
    fn execute_on_dest_context<RequestedResult>(self) -> RequestedResult
    where
        RequestedResult: TopDecodeMulti,
    {
        self.into_contract_call_full()
            .convert_to_esdt_transfer_call()
            .execute_on_dest_context()
    }

    /// Executes immediately, synchronously.
    ///
    /// The result (if any) is ignored.
    ///
    /// Deprecated and will be removed soon. Use `let _: IgnoreValue = contract_call.execute_on_dest_context(...)` instead.
    #[deprecated(
        since = "0.36.1",
        note = "Redundant method, use `let _: IgnoreValue = contract_call.execute_on_dest_context(...)` instead"
    )]
    fn execute_on_dest_context_ignore_result(self) {
        let _ = self.execute_on_dest_context::<IgnoreValue>();
    }

    fn execute_on_dest_context_readonly<RequestedResult>(self) -> RequestedResult
    where
        RequestedResult: TopDecodeMulti,
    {
        self.into_contract_call_full()
            .convert_to_esdt_transfer_call()
            .execute_on_dest_context_readonly()
    }

    fn execute_on_same_context<RequestedResult>(self) -> RequestedResult
    where
        RequestedResult: TopDecodeMulti,
    {
        self.into_contract_call_full()
            .convert_to_esdt_transfer_call()
            .execute_on_same_context()
    }

    /// Immediately launches a transfer-execute call.
    ///
    /// This is similar to an async call, but there is no callback
    /// and there can be more than one such call per transaction.
    fn transfer_execute(self) {
        self.into_contract_call_full().transfer_execute();
    }
}
