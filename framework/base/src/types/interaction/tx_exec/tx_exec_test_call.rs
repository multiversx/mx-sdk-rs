use crate::api::{CallTypeApi, TestApi};

use crate::{
    contract_base::TestRawWrapper,
    proxy_imports::TxFromSpecified,
    types::{
        FunctionCall, Tx, TxData, TxEmptyResultHandler, TxGas, TxPayment, TxScEnv, TxToSpecified,
    },
};

impl<Api, From, To, Payment, Gas, FC, RH> Tx<TxScEnv<Api>, From, To, Payment, Gas, FC, RH>
where
    Api: CallTypeApi + TestApi,
    From: TxFromSpecified<TxScEnv<Api>>,
    To: TxToSpecified<TxScEnv<Api>>,
    Payment: TxPayment<TxScEnv<Api>>,
    Gas: TxGas<TxScEnv<Api>>,
    FC: TxData<TxScEnv<Api>> + Into<FunctionCall<Api>>,
    RH: TxEmptyResultHandler<TxScEnv<Api>>,
{
    pub fn test_call(self) {
        self.from.with_value_ref(&self.env, |from| {
            TestRawWrapper::<Api>::new().start_prank(from);
        });

        let gas_limit = self.gas.gas_value(&self.env);
        self.to.with_value_ref(&self.env, |to| {
            self.payment
                .perform_transfer_execute(&self.env, to, gas_limit, self.data.into());
        });
        TestRawWrapper::<Api>::new().stop_prank();
    }
}
