use crate::{
    api::{CallTypeApi, StorageWriteApi},
    contract_base::SendRawWrapper,
    types::{BigUint, CallbackClosure, ManagedAddress},
};

use super::{
    FunctionCall, Tx, TxDataFunctionCall, TxEnv, TxPayment, TxResultHandler, TxScEnv, TxToSpecified,
};

pub trait TxAsyncCallCallback<Api>: TxResultHandler<TxScEnv<Api>>
where
    Api: CallTypeApi,
{
    fn save_callback_closure_to_storage(&self);
}

impl<Api> TxAsyncCallCallback<Api> for ()
where
    Api: CallTypeApi,
{
    fn save_callback_closure_to_storage(&self) {}
}

impl<Api> TxResultHandler<TxScEnv<Api>> for CallbackClosure<Api> where Api: CallTypeApi {}

impl<Api> TxAsyncCallCallback<Api> for CallbackClosure<Api>
where
    Api: CallTypeApi + StorageWriteApi,
{
    fn save_callback_closure_to_storage(&self) {
        self.save_to_storage::<Api>();
    }
}

impl<Api> TxResultHandler<TxScEnv<Api>> for Option<CallbackClosure<Api>> where Api: CallTypeApi {}
impl<Api> TxAsyncCallCallback<Api> for Option<CallbackClosure<Api>>
where
    Api: CallTypeApi + StorageWriteApi,
{
    fn save_callback_closure_to_storage(&self) {
        if let Some(closure) = self {
            closure.save_callback_closure_to_storage();
        }
    }
}

impl<Api, To, Payment, FC, RH> Tx<TxScEnv<Api>, (), To, Payment, (), FC, RH>
where
    Api: CallTypeApi,
    To: TxToSpecified<TxScEnv<Api>>,
    Payment: TxPayment<TxScEnv<Api>>,
    FC: TxDataFunctionCall<TxScEnv<Api>>,
    RH: TxAsyncCallCallback<Api>,
{
    pub fn async_call_and_exit(self) -> ! {
        let normalized = self.normalize_tx();
        normalized.result_handler.save_callback_closure_to_storage();
        SendRawWrapper::<Api>::new().async_call_raw(
            &normalized.to,
            &normalized.payment.value,
            &normalized.data.function_name,
            &normalized.data.arg_buffer,
        )
    }
}
