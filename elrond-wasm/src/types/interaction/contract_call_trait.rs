use elrond_codec::{TopDecodeMulti, TopEncodeMulti};

use crate::{
    api::CallTypeApi,
    contract_base::{ExitCodecErrorHandler, SendRawWrapper},
    err_msg,
    io::{ArgErrorHandler, ArgId, ManagedResultArgLoader},
    types::{ManagedBuffer, ManagedVec},
};

use super::{contract_call_full::ContractCallFull, AsyncCall, ContractCallNoPayment};

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

    /// Provided for cases where we build the contract call by hand.
    ///
    /// No serialization occurs, just direct conversion to ManagedBuffer.
    fn push_raw_arg<RawArg: Into<ManagedBuffer<SA>>>(&mut self, raw_arg: RawArg) {
        self.get_mut_basic().arg_buffer.push_arg_raw(raw_arg.into())
    }

    #[inline]
    fn with_gas_limit(mut self, gas_limit: u64) -> Self {
        self.get_mut_basic().explicit_gas_limit = gas_limit;
        self
    }

    fn async_call(self) -> AsyncCall<SA> {
        let contract_call_full = self
            .into_contract_call_full()
            .convert_to_esdt_transfer_call();
        AsyncCall {
            to: contract_call_full.basic.to,
            egld_payment: contract_call_full.egld_payment,
            endpoint_name: contract_call_full.basic.endpoint_name,
            arg_buffer: contract_call_full.basic.arg_buffer,
            callback_call: None,
        }
    }

    #[cfg(feature = "promises")]
    fn async_call_promise(self) -> super::AsyncCallPromises<SA> {
        let contract_call_full = self
            .into_contract_call_full()
            .convert_to_esdt_transfer_call();
        super::AsyncCallPromises {
            to: contract_call_full.basic.to,
            egld_payment: contract_call_full.egld_payment,
            endpoint_name: contract_call_full.basic.endpoint_name,
            arg_buffer: contract_call_full.basic.arg_buffer,
            explicit_gas_limit: contract_call_full.basic.explicit_gas_limit,
            extra_gas_for_callback: 0,
            callback_call: None,
        }
    }

    /// Executes immediately, synchronously, and returns contract call result.
    /// Only works if the target contract is in the same shard.
    fn execute_on_dest_context<RequestedResult>(self) -> RequestedResult
    where
        RequestedResult: TopDecodeMulti,
    {
        let contract_call_full = self
            .into_contract_call_full()
            .convert_to_esdt_transfer_call();
        let raw_result = SendRawWrapper::<SA>::new().execute_on_dest_context_raw(
            contract_call_full.resolve_gas_limit(),
            &contract_call_full.basic.to,
            &contract_call_full.egld_payment,
            &contract_call_full.basic.endpoint_name,
            &contract_call_full.basic.arg_buffer,
        );

        SendRawWrapper::<SA>::new().clean_return_data();

        decode_result(raw_result)
    }

    fn execute_on_dest_context_readonly<RequestedResult>(self) -> RequestedResult
    where
        RequestedResult: TopDecodeMulti,
    {
        let contract_call_full = self
            .into_contract_call_full()
            .convert_to_esdt_transfer_call();
        let raw_result = SendRawWrapper::<SA>::new().execute_on_dest_context_readonly_raw(
            contract_call_full.resolve_gas_limit(),
            &contract_call_full.basic.to,
            &contract_call_full.basic.endpoint_name,
            &contract_call_full.basic.arg_buffer,
        );

        SendRawWrapper::<SA>::new().clean_return_data();

        decode_result(raw_result)
    }

    fn execute_on_same_context<RequestedResult>(self) -> RequestedResult
    where
        RequestedResult: TopDecodeMulti,
    {
        let contract_call_full = self
            .into_contract_call_full()
            .convert_to_esdt_transfer_call();
        let raw_result = SendRawWrapper::<SA>::new().execute_on_same_context_raw(
            contract_call_full.resolve_gas_limit(),
            &contract_call_full.basic.to,
            &contract_call_full.egld_payment,
            &contract_call_full.basic.endpoint_name,
            &contract_call_full.basic.arg_buffer,
        );

        SendRawWrapper::<SA>::new().clean_return_data();

        decode_result(raw_result)
    }

    /// Immediately launches a transfer-execute call.
    ///
    /// This is similar to an async call, but there is no callback
    /// and there can be more than one such call per transaction.
    fn transfer_execute(self) {
        let contract_call_full = self.into_contract_call_full();

        match contract_call_full.payments.len() {
            0 => contract_call_full.no_payment_transfer_execute(),
            1 => contract_call_full.single_transfer_execute(),
            _ => contract_call_full.multi_transfer_execute(),
        }
    }
}

fn decode_result<SA, RequestedResult>(
    raw_result: ManagedVec<SA, ManagedBuffer<SA>>,
) -> RequestedResult
where
    SA: CallTypeApi + 'static,
    RequestedResult: TopDecodeMulti,
{
    let mut loader = ManagedResultArgLoader::new(raw_result);
    let arg_id = ArgId::from(&b"sync result"[..]);
    let h = ArgErrorHandler::<SA>::from(arg_id);
    let Ok(result) = RequestedResult::multi_decode_or_handle_err(&mut loader, h);
    result
}
