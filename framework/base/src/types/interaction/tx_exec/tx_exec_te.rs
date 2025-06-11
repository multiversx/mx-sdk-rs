use crate::api::{CallTypeApi, ErrorApiImpl};

use crate::contract_base::TransferExecuteFailed;
use crate::err_msg;
use crate::types::{
    FunctionCall, Tx, TxData, TxEmptyResultHandler, TxFrom, TxGas, TxPayment, TxScEnv,
    TxToSpecified,
};

struct TransferExecuteNothingToDo;

fn transfer_execute_signal_error<Api: CallTypeApi>(result: Result<(), TransferExecuteFailed>) {
    if result.is_err() {
        Api::error_api_impl().signal_error(err_msg::TRANSFER_EXECUTE_FAILED.as_bytes());
    }
}

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
    fn transfer_execute_gas_limit(&self) -> Result<u64, TransferExecuteNothingToDo> {
        if self.data.is_no_call() {
            if self.payment.is_no_payment(&self.env) {
                Err(TransferExecuteNothingToDo)
            } else {
                Ok(0)
            }
        } else {
            Ok(self.gas.gas_value(&self.env))
        }
    }

    fn transfer_execute_with_gas_fallible(
        self,
        gas_limit: u64,
    ) -> Result<(), TransferExecuteFailed> {
        self.to.with_address_ref(&self.env, |to| {
            self.payment
                .perform_transfer_execute(&self.env, to, gas_limit, self.data.into())
        })
    }

    /// Sends transaction asynchronously, and doesn't wait for callback ("fire and forget".)
    pub fn transfer_execute(self) {
        let result = self.transfer_execute_fallible();
        transfer_execute_signal_error::<Api>(result);
    }

    /// Sends transaction asynchronously, and doesn't wait for callback ("fire and forget".)
    pub fn transfer_execute_fallible(self) -> Result<(), TransferExecuteFailed> {
        if let Ok(gas_limit) = self.transfer_execute_gas_limit() {
            self.transfer_execute_with_gas_fallible(gas_limit)
        } else {
            // nothing to do
            Ok(())
        }
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
        let result = self.transfer_fallible();
        transfer_execute_signal_error::<Api>(result);
    }

    /// Only allowed for simple transfers.
    pub fn transfer_fallible(self) -> Result<(), TransferExecuteFailed> {
        self.transfer_execute_with_gas_fallible(0)
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
