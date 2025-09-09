use crate::api::{quick_signal_error, CallTypeApi};

use crate::contract_base::TransferExecuteFailed;
use crate::err_msg;
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
    fn transfer_execute_gas_limit(&self) -> u64 {
        if self.data.is_no_call() {
            if self.payment.is_no_payment(&self.env) {
                quick_signal_error::<Api>(err_msg::TRANSFER_EXECUTE_EMPTY);
            } else {
                0
            }
        } else {
            self.gas.gas_value(&self.env)
        }
    }

    fn transfer_execute_with_gas_fallible(
        self,
        gas_limit: u64,
    ) -> Result<(), TransferExecuteFailed> {
        self.to.with_address_ref(&self.env, |to| {
            self.payment.perform_transfer_execute_fallible(
                &self.env,
                to,
                gas_limit,
                self.data.into(),
            )
        })
    }

    /// Sends transaction asynchronously, and doesn't wait for callback ("fire and forget".)
    pub fn transfer_execute(self) {
        let gas_limit = self.transfer_execute_gas_limit();
        self.transfer_execute_with_gas(gas_limit);
    }

    fn transfer_execute_with_gas(self, gas_limit: u64) {
        self.to.with_address_ref(&self.env, |to| {
            self.payment.perform_transfer_execute_legacy(
                &self.env,
                to,
                gas_limit,
                self.data.into(),
            );
        });
    }

    /// Sends transaction asynchronously, and doesn't wait for callback ("fire and forget".)
    pub fn transfer_execute_fallible(self) -> Result<(), TransferExecuteFailed> {
        let gas_limit = self.transfer_execute_gas_limit();
        self.transfer_execute_with_gas_fallible(gas_limit)
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
    ///
    /// Will return error if transfer unsuccessful (e.g. because of frozen ESDT).
    pub fn transfer_fallible(self) -> Result<(), TransferExecuteFailed> {
        self.to.with_address_ref(&self.env, |to| {
            self.payment.perform_transfer_fallible(&self.env, to)
        })
    }

    /// Only allowed for simple transfers.
    ///
    /// Will crash if transfer unsuccessful (e.g. because of frozen ESDT).
    pub fn transfer(self) {
        self.transfer_execute_with_gas(0);
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
