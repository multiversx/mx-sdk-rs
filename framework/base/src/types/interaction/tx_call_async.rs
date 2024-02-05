use crate::{
    api::{CallTypeApi, StorageWriteApi},
    contract_base::SendRawWrapper,
    types::CallbackClosure,
};

use super::{
    Tx, TxData, TxDataFunctionCall, TxEnv, TxFrom, TxGas, TxPayment, TxResultHandler, TxScEnv,
    TxTo, TxToSpecified,
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

impl<Api> TxResultHandler<TxScEnv<Api>> for CallbackClosure<Api>
where
    Api: CallTypeApi,
{
    type OriginalResult = ();
}

impl<Api> TxAsyncCallCallback<Api> for CallbackClosure<Api>
where
    Api: CallTypeApi + StorageWriteApi,
{
    fn save_callback_closure_to_storage(&self) {
        self.save_to_storage::<Api>();
    }
}

impl<Api> TxResultHandler<TxScEnv<Api>> for Option<CallbackClosure<Api>>
where
    Api: CallTypeApi,
{
    type OriginalResult = ();
}

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

impl<Api, From, To, Payment, Gas, Data> Tx<TxScEnv<Api>, From, To, Payment, Gas, Data, ()>
where
    Api: CallTypeApi,
    From: TxFrom<TxScEnv<Api>>,
    To: TxTo<TxScEnv<Api>>,
    Payment: TxPayment<TxScEnv<Api>>,
    Gas: TxGas<TxScEnv<Api>>,
    Data: TxData<TxScEnv<Api>>,
{
    #[inline]
    pub fn callback<RH>(self, callback: RH) -> Tx<TxScEnv<Api>, From, To, Payment, Gas, Data, RH>
    where
        RH: TxAsyncCallCallback<Api>,
    {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: self.data,
            result_handler: callback,
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
