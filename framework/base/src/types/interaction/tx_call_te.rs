use core::marker::PhantomData;

use crate::{
    api::{BlockchainApiImpl, CallTypeApi},
    contract_base::BlockchainWrapper,
    types::{ManagedAddress, ManagedBuffer},
};

use super::{
    contract_call_exec::TRANSFER_EXECUTE_DEFAULT_LEFTOVER, AnnotatedValue, AsyncCall, ExplicitGas,
    FunctionCall, Tx, TxBaseWithEnv, TxData, TxEnv, TxFrom, TxGas, TxPayment, TxScEnv,
    TxToSpecified,
};

impl<Api, From, To, Payment, Gas, FC> Tx<TxScEnv<Api>, From, To, Payment, Gas, FC, ()>
where
    Api: CallTypeApi,
    From: TxFrom<TxScEnv<Api>>,
    To: TxToSpecified<TxScEnv<Api>>,
    Payment: TxPayment<TxScEnv<Api>>,
    Gas: TxGas<TxScEnv<Api>>,
    FC: TxData<TxScEnv<Api>> + Into<FunctionCall<Api>>,
{
    fn transfer_execute_with_gas(self, gas_limit: u64) {
        self.to.with_value_ref(|to| {
            self.payment
                .perform_transfer_execute(&self.env, to, gas_limit, self.data.into());
        });
    }

    pub fn transfer_execute(self) {
        let gas_limit: u64;
        if self.data.is_no_call() {
            if self.payment.is_no_payment() {
                return;
            } else {
                gas_limit = 0;
            }
        } else {
            gas_limit = self.gas.resolve_gas(&self.env);
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
}
