use crate::api::CallTypeApi;

use crate::types::{
    FunctionCall, Tx, TxData, TxEmptyResultHandler, TxFrom, TxGas, TxPayment, TxScEnv,
    TxToSpecified,
};

impl<Api, From, To, Payment, Gas, FC, RH> Tx<TxScEnv<Api>, From, To, Payment, Gas, FC, RH>
where
    Api: CallTypeApi,
    From: TxFrom<TxScEnv<Api>>,
    To: TxToSpecified<TxScEnv<Api>>,
    Payment: TxPayment<TxScEnv<Api>>,
    Gas: TxGas<TxScEnv<Api>>,
    FC: TxData<TxScEnv<Api>> + Into<FunctionCall<Api>>,
    RH: TxEmptyResultHandler<TxScEnv<Api>>,
{
    fn transfer_execute_with_gas(self, gas_limit: u64) {
        self.to.with_address_ref(&self.env, |to| {
            self.payment
                .perform_transfer_execute(&self.env, to, gas_limit, self.data.into());
        });
    }

    /// Sends transaction asynchronously, and doesn't wait for callback ("fire and forget".)
    pub fn transfer_execute(self) {
        let gas_limit: u64;
        if self.data.is_no_call() {
            if self.payment.is_no_payment(&self.env) {
                return;
            } else {
                gas_limit = 0;
            }
        } else {
            gas_limit = self.gas.gas_value(&self.env);
        }

        self.transfer_execute_with_gas(gas_limit);
    }
}

impl<Api, From, To, Payment> Tx<TxScEnv<Api>, From, To, Payment, (), (), ()>
where
    Api: CallTypeApi,
    From: TxFrom<TxScEnv<Api>>,
    To: TxToSpecified<TxScEnv<Api>>,
    Payment: TxPayment<TxScEnv<Api>>,
{
    /// Only allowed for simple transfers.
    pub fn transfer(self) {
        self.transfer_execute_with_gas(0)
    }

    /// Transfers funds, if amount is greater than zero. Does nothing otherwise.
    ///
    /// Can only used for simple transfers.
    pub fn transfer_if_not_empty(self) {
        if self.payment.is_no_payment(&self.env) {
            return;
        }

        self.transfer();
    }
}
