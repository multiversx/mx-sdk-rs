use multiversx_sc_codec::TopDecodeMulti;

use crate::{
    api::CallTypeApi,
    contract_base::{SendRawWrapper, SyncCallRawResult, SyncCallRawResultOrError},
    tuple_util::NestedTupleFlatten,
    types::{
        BackTransfersLegacy, ManagedBuffer, ManagedVec, RHListExec, Tx, TxDataFunctionCall,
        TxEmptyResultHandler, TxGas, TxNoPayment, TxPayment, TxResultHandler, TxScEnv,
        TxToSpecified, decode_result,
    },
};

impl<Api, To, Payment, Gas, FC, RH> Tx<TxScEnv<Api>, (), To, Payment, Gas, FC, RH>
where
    Api: CallTypeApi,
    To: TxToSpecified<TxScEnv<Api>>,
    Payment: TxPayment<TxScEnv<Api>>,
    Gas: TxGas<TxScEnv<Api>>,
    FC: TxDataFunctionCall<TxScEnv<Api>>,
    RH: TxResultHandler<TxScEnv<Api>>,
{
    fn execute_sync_call_raw(self) -> (SyncCallRawResult<Api>, RH) {
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
}

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
    /// Executes transaction synchronously.
    ///
    /// Only works with contracts from the same shard.
    ///
    /// The entire execution halts if the called contract fails.
    pub fn sync_call(self) -> <RH::ListReturns as NestedTupleFlatten>::Unpacked {
        self.result_handler.list_preprocessing();
        let (sync_raw_result, result_handler) = self.execute_sync_call_raw();
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
        self.result_handler.list_preprocessing();
        let (raw_result, result_handler) = self.execute_sync_call_same_context_raw();
        let sync_raw_result = SyncCallRawResult(raw_result);
        let tuple_result = result_handler.list_process_result(&sync_raw_result);
        tuple_result.flatten_unpack()
    }
}

impl<Api, To, Payment, Gas, FC, RH> Tx<TxScEnv<Api>, (), To, Payment, Gas, FC, RH>
where
    Api: CallTypeApi,
    To: TxToSpecified<TxScEnv<Api>>,
    Payment: TxPayment<TxScEnv<Api>>,
    Gas: TxGas<TxScEnv<Api>>,
    FC: TxDataFunctionCall<TxScEnv<Api>>,
    RH: TxResultHandler<TxScEnv<Api>>,
{
    fn execute_sync_call_fallible_raw(self) -> (SyncCallRawResultOrError<Api>, RH) {
        let gas_limit = self.gas.gas_value(&self.env);

        let raw_result = self.payment.with_normalized(
            &self.env,
            &self.from,
            self.to,
            self.data.into(),
            |norm_to, norm_egld, norm_fc| {
                SendRawWrapper::<Api>::new().execute_on_dest_context_error_return_raw(
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
}

impl<Api, To, Payment, Gas, FC, RH> Tx<TxScEnv<Api>, (), To, Payment, Gas, FC, RH>
where
    Api: CallTypeApi,
    To: TxToSpecified<TxScEnv<Api>>,
    Payment: TxPayment<TxScEnv<Api>>,
    Gas: TxGas<TxScEnv<Api>>,
    FC: TxDataFunctionCall<TxScEnv<Api>>,
    RH: RHListExec<SyncCallRawResultOrError<Api>, TxScEnv<Api>>,
    RH::ListReturns: NestedTupleFlatten,
{
    /// Executes transaction synchronously.
    ///
    /// Only works with contracts from the same shard.
    ///
    /// The called contract is allowed to return an error.
    /// The execution will not stop, the error code will be available to result handler.
    pub fn sync_call_fallible(self) -> <RH::ListReturns as NestedTupleFlatten>::Unpacked {
        self.result_handler.list_preprocessing();
        let (result_or_err, result_handler) = self.execute_sync_call_fallible_raw();
        let tuple_result = result_handler.list_process_result(&result_or_err);
        tuple_result.flatten_unpack()
    }
}

impl<Api, To, Payment, Gas, FC, RH> Tx<TxScEnv<Api>, (), To, Payment, Gas, FC, RH>
where
    Api: CallTypeApi,
    To: TxToSpecified<TxScEnv<Api>>,
    Payment: TxNoPayment<TxScEnv<Api>>,
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
        self.result_handler.list_preprocessing();
        let (raw_result, result_handler) = self.execute_sync_call_readonly_raw();
        let sync_raw_result = SyncCallRawResult(raw_result);
        let tuple_result = result_handler.list_process_result(&sync_raw_result);
        tuple_result.flatten_unpack()
    }
}

impl<Api, To, Payment, Gas, FC, RH> Tx<TxScEnv<Api>, (), To, Payment, Gas, FC, RH>
where
    Api: CallTypeApi,
    To: TxToSpecified<TxScEnv<Api>>,
    Payment: TxPayment<TxScEnv<Api>>,
    Gas: TxGas<TxScEnv<Api>>,
    FC: TxDataFunctionCall<TxScEnv<Api>>,
    RH: TxEmptyResultHandler<TxScEnv<Api>>,
{
    /// Backwards compatibility.
    ///
    /// Incompatible with result handlers.
    #[deprecated(since = "0.61.0", note = "Please use `.sync_call()` instead.")]
    pub fn execute_on_dest_context<RequestedResult>(self) -> RequestedResult
    where
        RequestedResult: TopDecodeMulti,
    {
        let (raw_result, _) = self.execute_sync_call_raw();
        decode_result(raw_result.0)
    }

    /// Backwards compatibility.
    ///
    /// Incompatible with result handlers.
    #[deprecated(
        since = "0.61.0",
        note = "Please use `.returns(ReturnsBackTransfers).sync_call()` instead."
    )]
    pub fn execute_on_dest_context_with_back_transfers<RequestedResult>(
        self,
    ) -> (RequestedResult, BackTransfersLegacy<Api>)
    where
        RequestedResult: TopDecodeMulti,
    {
        let result = self.execute_on_dest_context();
        let back_transfers =
            crate::contract_base::BlockchainWrapper::<Api>::new().get_back_transfers_legacy();

        (result, back_transfers)
    }

    /// Reimagined, based on the old syntax. Please use `.returns(ReturnsHandledOrError::new().returns(...)).sync_call_fallible()` instead.
    ///
    /// Incompatible with result handlers.
    #[deprecated(
        since = "0.61.0",
        note = "Please use `.returns(ReturnsHandledOrError::new().returns(...)).sync_call_fallible()` instead."
    )]
    pub fn execute_on_dest_context_fallible<RequestedResult>(self) -> Result<RequestedResult, u32>
    where
        RequestedResult: TopDecodeMulti,
    {
        let (raw_result, _) = self.execute_sync_call_fallible_raw();

        match raw_result {
            SyncCallRawResultOrError::Success(result) => Ok(decode_result(result.0)),
            SyncCallRawResultOrError::Error(err_code) => Err(err_code),
        }
    }
}
