use multiversx_sc_codec::TopDecodeMulti;

use crate::{
    api::CallTypeApi,
    contract_base::SendRawWrapper,
    tuple_util::NestedTupleFlatten,
    types::{
        decode_result, BackTransfers, ManagedBuffer, ManagedVec, OriginalResultMarker, RHListExec,
        Tx, TxDataFunctionCall, TxGas, TxPayment, TxScEnv, TxToSpecified,
    },
};

pub struct SyncCallRawResult<Api>(pub ManagedVec<Api, ManagedBuffer<Api>>)
where
    Api: CallTypeApi;

impl<Api, To, Payment, Gas, FC, RH> Tx<TxScEnv<Api>, (), To, Payment, Gas, FC, RH>
where
    Api: CallTypeApi,
    To: TxToSpecified<TxScEnv<Api>>,
    Payment: TxPayment<TxScEnv<Api>>,
    Gas: TxGas<TxScEnv<Api>>,
    FC: TxDataFunctionCall<TxScEnv<Api>>,
    RH: RHListExec<SyncCallRawResult<Api>, TxScEnv<Api>>,
    RH::ListReturns: NestedTupleFlatten,
{
    fn execute_sync_call_raw(self) -> (ManagedVec<Api, ManagedBuffer<Api>>, RH) {
        let gas_limit = self.gas.gas_value(&self.env);

        let raw_result = self.payment.with_normalized(
            &self.env,
            &self.from,
            self.to,
            self.data.into(),
            |norm_to, norm_egld, norm_fc| {
                SendRawWrapper::<Api>::new().execute_on_dest_context_raw(
                    gas_limit,
                    norm_to,
                    norm_egld,
                    &norm_fc.function_name,
                    &norm_fc.arg_buffer,
                )
            },
        );

        SendRawWrapper::<Api>::new().clean_return_data();

        (raw_result, self.result_handler)
    }

    /// Executes transaction synchronously.
    ///
    /// Only works with contracts from the same shard.
    pub fn sync_call(self) -> <RH::ListReturns as NestedTupleFlatten>::Unpacked {
        let (raw_result, result_handler) = self.execute_sync_call_raw();
        let sync_raw_result = SyncCallRawResult(raw_result);
        let tuple_result = result_handler.list_process_result(&sync_raw_result);
        tuple_result.flatten_unpack()
    }

    fn execute_sync_call_same_context_raw(self) -> (ManagedVec<Api, ManagedBuffer<Api>>, RH) {
        let gas_limit = self.gas.gas_value(&self.env);

        let raw_result = self.payment.with_normalized(
            &self.env,
            &self.from,
            self.to,
            self.data.into(),
            |norm_to, norm_egld, norm_fc| {
                SendRawWrapper::<Api>::new().execute_on_same_context_raw(
                    gas_limit,
                    norm_to,
                    norm_egld,
                    &norm_fc.function_name,
                    &norm_fc.arg_buffer,
                )
            },
        );

        SendRawWrapper::<Api>::new().clean_return_data();

        (raw_result, self.result_handler)
    }

    /// Executes transaction synchronously, in the same context (performed in the name of the caller).
    ///
    /// Only works with contracts from the same shard.
    pub fn sync_call_same_context(self) -> <RH::ListReturns as NestedTupleFlatten>::Unpacked {
        let (raw_result, result_handler) = self.execute_sync_call_same_context_raw();
        let sync_raw_result = SyncCallRawResult(raw_result);
        let tuple_result = result_handler.list_process_result(&sync_raw_result);
        tuple_result.flatten_unpack()
    }
}

impl<Api, To, Gas, FC, RH> Tx<TxScEnv<Api>, (), To, (), Gas, FC, RH>
where
    Api: CallTypeApi,
    To: TxToSpecified<TxScEnv<Api>>,
    Gas: TxGas<TxScEnv<Api>>,
    FC: TxDataFunctionCall<TxScEnv<Api>>,
    RH: RHListExec<SyncCallRawResult<Api>, TxScEnv<Api>>,
    RH::ListReturns: NestedTupleFlatten,
{
    fn execute_sync_call_readonly_raw(self) -> (ManagedVec<Api, ManagedBuffer<Api>>, RH) {
        let gas_limit = self.gas.gas_value(&self.env);
        let function_call = self.data.into();

        let raw_result = self.to.with_value_ref(&self.env, |to| {
            SendRawWrapper::<Api>::new().execute_on_dest_context_readonly_raw(
                gas_limit,
                to,
                &function_call.function_name,
                &function_call.arg_buffer,
            )
        });

        SendRawWrapper::<Api>::new().clean_return_data();

        (raw_result, self.result_handler)
    }

    /// Executes transaction synchronously, in readonly mode (target contract cannot have its state altered).
    ///
    /// Only works with contracts from the same shard.
    pub fn sync_call_readonly(self) -> <RH::ListReturns as NestedTupleFlatten>::Unpacked {
        let (raw_result, result_handler) = self.execute_sync_call_readonly_raw();
        let sync_raw_result = SyncCallRawResult(raw_result);
        let tuple_result = result_handler.list_process_result(&sync_raw_result);
        tuple_result.flatten_unpack()
    }
}

impl<Api, To, Payment, Gas, FC, OriginalResult>
    Tx<TxScEnv<Api>, (), To, Payment, Gas, FC, OriginalResultMarker<OriginalResult>>
where
    Api: CallTypeApi,
    To: TxToSpecified<TxScEnv<Api>>,
    Payment: TxPayment<TxScEnv<Api>>,
    Gas: TxGas<TxScEnv<Api>>,
    FC: TxDataFunctionCall<TxScEnv<Api>>,
{
    /// Backwards compatibility.
    pub fn execute_on_dest_context<RequestedResult>(self) -> RequestedResult
    where
        RequestedResult: TopDecodeMulti,
    {
        let (raw_result, _) = self.execute_sync_call_raw();
        decode_result(raw_result)
    }

    /// Backwards compatibility.
    pub fn execute_on_dest_context_with_back_transfers<RequestedResult>(
        self,
    ) -> (RequestedResult, BackTransfers<Api>)
    where
        RequestedResult: TopDecodeMulti,
    {
        let result = self.execute_on_dest_context();
        let back_transfers =
            crate::contract_base::BlockchainWrapper::<Api>::new().get_back_transfers();

        (result, back_transfers)
    }
}
