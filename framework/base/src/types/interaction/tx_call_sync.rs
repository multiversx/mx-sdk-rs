use crate::{
    api::{CallTypeApi, StorageWriteApi},
    contract_base::SendRawWrapper,
    tuple_util::NestedTupleFlatten,
    types::{BigUint, CallbackClosure, ManagedAddress, ManagedBuffer, ManagedVec},
};

use super::{
    ConsNoRet, ConsRet, FunctionCall, OriginalResultMarker, RHList, RHListItem, Tx,
    TxDataFunctionCall, TxEnv, TxGas, TxPayment, TxResultHandler, TxReturn, TxReturnSync, TxScEnv,
    TxToSpecified,
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
        raw_results: &ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
    ) -> Self::ListReturns {
    }
}

impl<Env, O> RHListSync<Env> for OriginalResultMarker<O>
where
    Env: TxEnv,
{
    fn list_sync_call_result(
        self,
        raw_results: &ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
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
    pub fn execute_on_dest_context(self) -> <RH::ListReturns as NestedTupleFlatten>::Unpacked {
        let gas_limit = self.gas.resolve_gas(&self.env);
        let normalized = self.normalize_tx();

        let raw_result = SendRawWrapper::<Api>::new().execute_on_dest_context_raw(
            gas_limit,
            &normalized.to,
            &normalized.payment.value,
            &normalized.data.function_name,
            &normalized.data.arg_buffer,
        );

        SendRawWrapper::<Api>::new().clean_return_data();

        let tuple_result = normalized.result_handler.list_sync_call_result(&raw_result);
        tuple_result.flatten_unpack()
    }
}
