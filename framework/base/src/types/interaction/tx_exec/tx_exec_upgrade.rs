use crate::{
    api::CallTypeApi,
    contract_base::SendRawWrapper,
    proxy_imports::TxToSpecified,
    types::{
        Code, CodeMetadata, FromSource, ManagedAddress, ManagedBuffer, Tx, TxCodeValue,
        TxEmptyResultHandler, TxFromSourceValue, TxGas, TxPaymentEgldOnly, TxScEnv, UpgradeCall,
    },
};

impl<Api, Payment, Gas, CodeValue, RH>
    Tx<
        TxScEnv<Api>,
        (),
        ManagedAddress<Api>,
        Payment,
        Gas,
        UpgradeCall<TxScEnv<Api>, Code<CodeValue>>,
        RH,
    >
where
    Api: CallTypeApi,
    Payment: TxPaymentEgldOnly<TxScEnv<Api>>,
    Gas: TxGas<TxScEnv<Api>>,
    CodeValue: TxCodeValue<TxScEnv<Api>>,
    RH: TxEmptyResultHandler<TxScEnv<Api>>,
{
    /// Launches the upgrade async call.
    ///
    /// TODO: change return type to `!`.
    pub fn upgrade_async_call_and_exit(self) {
        let gas = self.gas.explicit_or_gas_left(&self.env);
        self.payment.with_egld_value(&self.env, |egld_value| {
            SendRawWrapper::<Api>::new().upgrade_contract(
                &self.to,
                gas,
                egld_value,
                &self.data.code_source.0.into_value(&self.env),
                self.data.code_metadata,
                &self.data.arg_buffer,
            );
        });
    }
}

impl<Api, Payment, Gas, FromSourceValue, RH>
    Tx<
        TxScEnv<Api>,
        (),
        ManagedAddress<Api>,
        Payment,
        Gas,
        UpgradeCall<TxScEnv<Api>, FromSource<FromSourceValue>>,
        RH,
    >
where
    Api: CallTypeApi,
    Payment: TxPaymentEgldOnly<TxScEnv<Api>>,
    Gas: TxGas<TxScEnv<Api>>,
    FromSourceValue: TxFromSourceValue<TxScEnv<Api>>,
    RH: TxEmptyResultHandler<TxScEnv<Api>>,
{
    /// Launches the upgrade from source async call.
    ///
    /// TODO: change return type to `!`.
    pub fn upgrade_async_call_and_exit(self) {
        let gas = self.gas.explicit_or_gas_left(&self.env);
        self.payment.with_egld_value(&self.env, |egld_value| {
            SendRawWrapper::<Api>::new().upgrade_from_source_contract(
                &self.to,
                gas,
                egld_value,
                &self.data.code_source.0.into_value(&self.env),
                self.data.code_metadata,
                &self.data.arg_buffer,
            );
        });
    }
}

impl<Api, To, Payment, Gas, RH>
    Tx<TxScEnv<Api>, (), To, Payment, Gas, UpgradeCall<TxScEnv<Api>, ()>, RH>
where
    Api: CallTypeApi,
    To: TxToSpecified<TxScEnv<Api>>,
    Payment: TxPaymentEgldOnly<TxScEnv<Api>>,
    Gas: TxGas<TxScEnv<Api>>,
    RH: TxEmptyResultHandler<TxScEnv<Api>>,
{
    /// Transition syntax, immitates the old API.
    ///
    /// Note that the data type (the `UpgradeCall`) doesn't have the code set.
    /// This is because the old API was passing it as parameter, so we use it from the `code` argument.
    ///
    /// Also note that the code metadata is taken from the `code_metadata` argument.
    /// If another one was previously set in the `Tx` object, that one will be ignored.
    #[deprecated(
        since = "0.49.0",
        note = "This is a transition syntax, it would not have been reachable before 0.49.0. Use [upgrade_async_call_and_exit] instead."
    )]
    pub fn upgrade_contract(self, code: &ManagedBuffer<Api>, code_metadata: CodeMetadata) {
        let gas = self.gas.explicit_or_gas_left(&self.env);
        self.payment.with_egld_value(&self.env, |egld_value| {
            self.to.with_value_ref(&self.env, |to| {
                SendRawWrapper::<Api>::new().upgrade_contract(
                    to,
                    gas,
                    egld_value,
                    code,
                    code_metadata,
                    &self.data.arg_buffer,
                );
            });
        });
    }

    /// Transition syntax, immitates the old API.
    ///
    /// Note that the data type (the `UpgradeCall`) doesn't have the code set.
    /// This is because the old API was passing it as parameter, so we use it from the `code` argument.
    ///
    /// Also note that the code metadata is taken from the `code_metadata` argument.
    /// If another one was previously set in the `Tx` object, that one will be ignored.
    #[deprecated(
        since = "0.49.0",
        note = "This is a transition syntax, it would not have been reachable before 0.49.0. Use [upgrade_async_call_and_exit] instead."
    )]
    pub fn upgrade_from_source(
        self,
        source_address: &ManagedAddress<Api>,
        code_metadata: CodeMetadata,
    ) {
        let gas = self.gas.explicit_or_gas_left(&self.env);
        self.payment.with_egld_value(&self.env, |egld_value| {
            self.to.with_value_ref(&self.env, |to| {
                SendRawWrapper::<Api>::new().upgrade_from_source_contract(
                    to,
                    gas,
                    egld_value,
                    source_address,
                    code_metadata,
                    &self.data.arg_buffer,
                );
            });
        });
    }
}
