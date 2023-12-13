use crate::{
    api::{CallTypeApi, StorageWriteApi},
    contract_base::SendRawWrapper,
    types::{BigUint, CallbackClosure, ManagedAddress},
};

use super::{
    FunctionCall, Tx, TxDataFunctionCall, TxEnv, TxPayment, TxResultHandler, TxReturn, TxScEnv,
    TxToSpecified, TxGas,
};

impl<Api, To, Payment, Gas, FC, RH> Tx<TxScEnv<Api>, (), To, Payment, Gas, FC, RH>
where
    Api: CallTypeApi,
    To: TxToSpecified<TxScEnv<Api>>,
    Payment: TxPayment<TxScEnv<Api>>,
    Gas: TxGas<TxScEnv<Api>>,
    FC: TxDataFunctionCall<TxScEnv<Api>>,
    RH: TxReturn<TxScEnv<Api>>,
{
    pub fn execute_on_dest_context(self) -> RH::Returned {
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

        normalized.result_handler.sync_call_result(&raw_result)
    }
}
