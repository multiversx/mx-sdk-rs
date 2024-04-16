use crate::{
    api::{const_handles, CallTypeApi},
    contract_base::SendRawWrapper,
    types::{
        interaction::callback_closure::CallbackClosureWithGas, CallbackClosure, ExplicitGas,
        FunctionCall, ManagedBuffer, ManagedType, OriginalResultMarker, Tx, TxGas, TxGasValue,
        TxPayment, TxResultHandler, TxScEnv, TxToSpecified,
    },
};

pub trait TxPromisesCallback<Api>: TxResultHandler<TxScEnv<Api>>
where
    Api: CallTypeApi,
{
    fn callback_name(&self) -> &'static str;

    fn overwrite_with_serialized_args(&self, cb_closure_args_serialized: &mut ManagedBuffer<Api>);

    fn gas_for_callback(&self) -> u64;
}

impl<Api> TxPromisesCallback<Api> for ()
where
    Api: CallTypeApi,
{
    fn callback_name(&self) -> &'static str {
        ""
    }

    fn overwrite_with_serialized_args(&self, cb_closure_args_serialized: &mut ManagedBuffer<Api>) {
        cb_closure_args_serialized.overwrite(&[]);
    }

    fn gas_for_callback(&self) -> u64 {
        0
    }
}

impl<Api, O> TxPromisesCallback<Api> for OriginalResultMarker<O>
where
    Api: CallTypeApi,
{
    fn callback_name(&self) -> &'static str {
        ""
    }

    fn overwrite_with_serialized_args(&self, cb_closure_args_serialized: &mut ManagedBuffer<Api>) {
        cb_closure_args_serialized.overwrite(&[]);
    }

    fn gas_for_callback(&self) -> u64 {
        0
    }
}

impl<Api> TxPromisesCallback<Api> for CallbackClosure<Api>
where
    Api: CallTypeApi,
{
    fn callback_name(&self) -> &'static str {
        self.callback_name
    }

    fn overwrite_with_serialized_args(&self, cb_closure_args_serialized: &mut ManagedBuffer<Api>) {
        self.closure_args
            .serialize_overwrite(cb_closure_args_serialized);
    }

    fn gas_for_callback(&self) -> u64 {
        0u64
    }
}

impl<Api> TxResultHandler<TxScEnv<Api>> for CallbackClosureWithGas<Api>
where
    Api: CallTypeApi,
{
    type OriginalResult = ();
}

impl<Api> TxPromisesCallback<Api> for CallbackClosureWithGas<Api>
where
    Api: CallTypeApi,
{
    fn callback_name(&self) -> &'static str {
        self.closure.callback_name
    }

    fn overwrite_with_serialized_args(&self, cb_closure_args_serialized: &mut ManagedBuffer<Api>) {
        self.closure
            .closure_args
            .serialize_overwrite(cb_closure_args_serialized);
    }

    fn gas_for_callback(&self) -> u64 {
        self.gas_for_callback
    }
}

impl<Api, To, Payment, Gas>
    Tx<TxScEnv<Api>, (), To, Payment, Gas, FunctionCall<Api>, CallbackClosure<Api>>
where
    Api: CallTypeApi,
    To: TxToSpecified<TxScEnv<Api>>,
    Payment: TxPayment<TxScEnv<Api>>,
    Gas: TxGas<TxScEnv<Api>>,
{
    pub fn gas_for_callback(
        self,
        gas: u64,
    ) -> Tx<TxScEnv<Api>, (), To, Payment, Gas, FunctionCall<Api>, CallbackClosureWithGas<Api>>
    {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: self.data,
            result_handler: CallbackClosureWithGas {
                closure: self.result_handler,
                gas_for_callback: gas,
            },
        }
    }

    /// Backwards compatibility.
    pub fn with_extra_gas_for_callback(
        self,
        gas: u64,
    ) -> Tx<TxScEnv<Api>, (), To, Payment, Gas, FunctionCall<Api>, CallbackClosureWithGas<Api>>
    {
        self.gas_for_callback(gas)
    }
}

impl<Api, To, Payment, GasValue, Callback>
    Tx<TxScEnv<Api>, (), To, Payment, ExplicitGas<GasValue>, FunctionCall<Api>, Callback>
where
    Api: CallTypeApi,
    To: TxToSpecified<TxScEnv<Api>>,
    Payment: TxPayment<TxScEnv<Api>>,
    GasValue: TxGasValue<TxScEnv<Api>>,
    Callback: TxPromisesCallback<Api>,
{
    pub fn register_promise(self) {
        let callback_name = self.result_handler.callback_name();
        let mut cb_closure_args_serialized =
            ManagedBuffer::<Api>::from_raw_handle(const_handles::MBUF_TEMPORARY_1);
        self.result_handler
            .overwrite_with_serialized_args(&mut cb_closure_args_serialized);
        let extra_gas_for_callback = self.result_handler.gas_for_callback();
        let gas = self.gas.gas_value(&self.env);

        self.payment.with_normalized(
            &self.env,
            &self.from,
            self.to,
            self.data,
            |norm_to, norm_egld, norm_fc| {
                SendRawWrapper::<Api>::new().create_async_call_raw(
                    &norm_to,
                    &norm_egld,
                    &norm_fc.function_name,
                    &norm_fc.arg_buffer,
                    callback_name,
                    callback_name,
                    gas,
                    extra_gas_for_callback,
                    &cb_closure_args_serialized,
                )
            },
        )
    }
}

impl<Api, To, Payment, Gas, Callback>
    Tx<TxScEnv<Api>, (), To, Payment, Gas, FunctionCall<Api>, Callback>
where
    Api: CallTypeApi,
    To: TxToSpecified<TxScEnv<Api>>,
    Payment: TxPayment<TxScEnv<Api>>,
    Gas: TxGas<TxScEnv<Api>>,
    Payment: TxPayment<TxScEnv<Api>>,
    Callback: TxPromisesCallback<Api>,
{
    /// Backwards compatibility only.
    #[inline]
    pub fn async_call_promise(self) -> Self {
        self
    }
}
