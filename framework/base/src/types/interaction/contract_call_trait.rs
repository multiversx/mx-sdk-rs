use crate::codec::{multi_types::IgnoreValue, TopDecodeMulti, TopEncodeMulti};

use crate::{
    api::CallTypeApi, contract_base::ExitCodecErrorHandler, err_msg, types::ManagedBuffer,
};

use super::{AsyncCall, ContractCallNoPayment, ContractCallWithEgld, ManagedArgBuffer};

/// Defines a contract call object, which is the basis for all calls to other contracts.
///
/// Its implementations differ on the type of payment that gets sent with the call.
pub trait ContractCall<SA>: Sized
where
    SA: CallTypeApi + 'static,
{
    type OriginalResult: TopEncodeMulti;

    /// Converts any ESDT transfers into builtin function calls,
    /// thus reducing it to a simple transaction with optional EGLD value.
    #[doc(hidden)]
    fn into_normalized(self) -> ContractCallWithEgld<SA, Self::OriginalResult>;

    /// Mutable access to the common base.
    #[doc(hidden)]
    fn get_mut_basic(&mut self) -> &mut ContractCallNoPayment<SA, Self::OriginalResult>;

    /// Used by the generated proxies to add arguments to a call.
    #[doc(hidden)]
    fn proxy_arg<T: TopEncodeMulti>(&mut self, endpoint_arg: &T) {
        let h = ExitCodecErrorHandler::<SA>::from(err_msg::CONTRACT_CALL_ENCODE_ERROR);
        let Ok(()) =
            endpoint_arg.multi_encode_or_handle_err(&mut self.get_mut_basic().arg_buffer, h);
    }

    /// For cases where we build the contract call by hand.
    ///
    /// No serialization occurs, just direct conversion to ManagedBuffer.
    fn push_raw_argument<RawArg: Into<ManagedBuffer<SA>>>(&mut self, raw_arg: RawArg) {
        self.get_mut_basic().arg_buffer.push_arg_raw(raw_arg.into())
    }

    /// For cases where we build the contract call by hand.
    fn with_raw_arguments(mut self, raw_argument_buffer: ManagedArgBuffer<SA>) -> Self {
        self.get_mut_basic().arg_buffer = raw_argument_buffer;
        self
    }

    /// Sets an explicit gas limit to the call.
    #[inline]
    fn with_gas_limit(mut self, gas_limit: u64) -> Self {
        self.get_mut_basic().explicit_gas_limit = gas_limit;
        self
    }

    fn into_call_data_string(self) -> ManagedBuffer<SA> {
        self.into_normalized().to_call_data_string()
    }

    /// Converts to a legacy async call.
    #[inline]
    fn async_call(self) -> AsyncCall<SA> {
        self.into_normalized().async_call()
    }

    /// Converts to an async promise.
    #[inline]
    #[cfg(feature = "promises")]
    fn async_call_promise(self) -> super::AsyncCallPromises<SA> {
        self.into_normalized().async_call_promise()
    }

    /// Executes immediately, synchronously, and returns contract call result.
    /// Only works if the target contract is in the same shard.
    #[inline]
    fn execute_on_dest_context<RequestedResult>(self) -> RequestedResult
    where
        RequestedResult: TopDecodeMulti,
    {
        self.into_normalized().execute_on_dest_context()
    }

    /// Executes immediately, synchronously.
    ///
    /// The result (if any) is ignored.
    ///
    /// Deprecated and will be removed soon. Use `execute_on_dest_context::<IgnoreValue>(...)` instead.
    #[deprecated(
        since = "0.36.1",
        note = "Redundant method, use `let _: IgnoreValue = contract_call.execute_on_dest_context(...)` instead"
    )]
    fn execute_on_dest_context_ignore_result(self) {
        let _ = self.execute_on_dest_context::<IgnoreValue>();
    }

    /// Executes immediately, synchronously, and returns contract call result.
    ///
    /// Performs a readonly call.    
    #[inline]
    fn execute_on_dest_context_readonly<RequestedResult>(self) -> RequestedResult
    where
        RequestedResult: TopDecodeMulti,
    {
        self.into_normalized().execute_on_dest_context_readonly()
    }

    /// Executes immediately, synchronously, and returns contract call result.
    ///
    /// Performs call on the same context, i.e. the target contract will operate with the data from this contract.    
    #[inline]
    fn execute_on_same_context<RequestedResult>(self) -> RequestedResult
    where
        RequestedResult: TopDecodeMulti,
    {
        self.into_normalized().execute_on_same_context()
    }

    /// Immediately launches a transfer-execute call.
    ///
    /// This is similar to an async call, but there is no callback
    /// and there can be more than one such call per transaction.
    fn transfer_execute(self);
}
