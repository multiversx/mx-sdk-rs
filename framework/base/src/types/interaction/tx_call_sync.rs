use multiversx_sc_codec::TopDecodeMulti;

use crate::{
    api::CallTypeApi,
    contract_base::SendRawWrapper,
    tuple_util::NestedTupleFlatten,
    types::{ManagedBuffer, ManagedVec},
};

use super::{
    contract_call_exec::decode_result, BackTransfers, ConsNoRet, ConsRet, OriginalResultMarker,
    RHList, RHListItem, Tx, TxDataFunctionCall, TxEnv, TxGas, TxPayment, TxScEnv, TxToSpecified,
};

pub trait RHListItemSync<Env, Original>: RHListItem<Env, Original>
where
    Env: TxEnv,
{
    fn item_sync_call_result(
        self,
        raw_results: &ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
    ) -> Self::Returns;
}

pub trait RHListSync<Env>: RHList<Env>
where
    Env: TxEnv,
{
    fn list_sync_call_result(
        self,
        raw_results: &ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
    ) -> Self::ListReturns;
}

impl<Env> RHListSync<Env> for ()
where
    Env: TxEnv,
{
    fn list_sync_call_result(
        self,
        _raw_results: &ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
    ) -> Self::ListReturns {
    }
}

impl<Env, O> RHListSync<Env> for OriginalResultMarker<O>
where
    Env: TxEnv,
{
    fn list_sync_call_result(
        self,
        _raw_results: &ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
    ) -> Self::ListReturns {
    }
}

impl<Env, Head, Tail> RHListSync<Env> for ConsRet<Env, Head, Tail>
where
    Env: TxEnv,
    Head: RHListItemSync<Env, Tail::OriginalResult>,
    Tail: RHListSync<Env>,
{
    fn list_sync_call_result(
        self,
        raw_results: &ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
    ) -> Self::ListReturns {
        let head_result = self.head.item_sync_call_result(raw_results);
        let tail_result = self.tail.list_sync_call_result(raw_results);
        (head_result, tail_result)
    }
}

impl<Env, Head, Tail> RHListSync<Env> for ConsNoRet<Env, Head, Tail>
where
    Env: TxEnv,
    Head: RHListItemSync<Env, Tail::OriginalResult, Returns = ()>,
    Tail: RHListSync<Env>,
{
    fn list_sync_call_result(
        self,
        raw_results: &ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
    ) -> Self::ListReturns {
        self.head.item_sync_call_result(raw_results);
        self.tail.list_sync_call_result(raw_results)
    }
}

impl<Api, To, Payment, Gas, FC, RH> Tx<TxScEnv<Api>, (), To, Payment, Gas, FC, RH>
where
    Api: CallTypeApi,
    To: TxToSpecified<TxScEnv<Api>>,
    Payment: TxPayment<TxScEnv<Api>>,
    Gas: TxGas<TxScEnv<Api>>,
    FC: TxDataFunctionCall<TxScEnv<Api>>,
    RH: RHListSync<TxScEnv<Api>>,
    RH::ListReturns: NestedTupleFlatten,
{
    fn execute_sync_call_raw(self) -> (ManagedVec<Api, ManagedBuffer<Api>>, RH) {
        let gas_limit = self.gas.resolve_gas(&self.env);

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

    pub fn sync_call(self) -> <RH::ListReturns as NestedTupleFlatten>::Unpacked {
        let (raw_result, result_handler) = self.execute_sync_call_raw();

        let tuple_result = result_handler.list_sync_call_result(&raw_result);
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
