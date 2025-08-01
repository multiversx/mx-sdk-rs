use crate::{
    api::{CallTypeApi, StorageWriteApi},
    contract_base::SendRawWrapper,
    types::{
        CallbackClosure, OriginalResultMarker, Tx, TxData, TxDataFunctionCall,
        TxEmptyResultHandler, TxEnv, TxFrom, TxGas, TxPayment, TxResultHandler, TxScEnv, TxTo,
        TxToSpecified,
    },
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

impl<Api, O> TxAsyncCallCallback<Api> for OriginalResultMarker<O>
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

impl<Api, To, Payment, Gas, Data, EmptyRH> Tx<TxScEnv<Api>, (), To, Payment, Gas, Data, EmptyRH>
where
    Api: CallTypeApi,
    To: TxTo<TxScEnv<Api>>,
    Payment: TxPayment<TxScEnv<Api>>,
    Gas: TxGas<TxScEnv<Api>>,
    Data: TxData<TxScEnv<Api>>,
    EmptyRH: TxEmptyResultHandler<TxScEnv<Api>>,
{
    #[inline]
    pub fn callback<RH>(self, callback: RH) -> Tx<TxScEnv<Api>, (), To, Payment, Gas, Data, RH>
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

impl<Api, To, Payment, Gas, FC, EmptyRH> Tx<TxScEnv<Api>, (), To, Payment, Gas, FC, EmptyRH>
where
    Api: CallTypeApi,
    To: TxToSpecified<TxScEnv<Api>>,
    Payment: TxPayment<TxScEnv<Api>>,
    Gas: TxGas<TxScEnv<Api>>,
    FC: TxDataFunctionCall<TxScEnv<Api>>,
    EmptyRH: TxEmptyResultHandler<TxScEnv<Api>>,
{
    /// Backwards compatibility.
    pub fn with_callback<RH>(self, callback: RH) -> Tx<TxScEnv<Api>, (), To, Payment, Gas, FC, RH>
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
        self.result_handler.save_callback_closure_to_storage();
        self.payment.with_normalized(
            &self.env,
            &self.from,
            self.to,
            self.data.into(),
            |norm_to, norm_egld, norm_fc| {
                SendRawWrapper::<Api>::new().async_call_raw(
                    norm_to,
                    norm_egld,
                    &norm_fc.function_name,
                    &norm_fc.arg_buffer,
                )
            },
        )
    }

    /// Backwards compatibility only.
    #[deprecated(
        since = "0.59.2",
        note = "Backwards compatibility only, does nothing. Just delete. Use `async_call_and_exit` to launch asynchronous calls or `sync_call` to launch synchronous calls."
    )]
    pub fn call_and_exit(self) -> ! {
        self.async_call_and_exit()
    }
}

impl<Env, From, To, Payment, Gas, Data, RH> Tx<Env, From, To, Payment, Gas, Data, RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
    Data: TxData<Env>,
    RH: TxResultHandler<Env>,
{
    /// Backwards compatibility only.
    #[deprecated(
        since = "0.50.2",
        note = "Backwards compatibility only, does nothing. Just delete. Use `async_call_and_exit` to launch asynchronous calls."
    )]
    #[inline]
    pub fn async_call(self) -> Tx<Env, From, To, Payment, Gas, Data, RH> {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: self.data,
            result_handler: self.result_handler,
        }
    }
}
