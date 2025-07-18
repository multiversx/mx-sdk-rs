use crate::api::{CallTypeApi, TestApi};
use crate::tuple_util::NestedTupleFlatten;
use crate::{
    contract_base::TestRawWrapper,
    types::{
        CodePath, DeployCall, FunctionCall, ManagedAddress, ManagedVec, RHListExec, Tx,
        TxCodePathValue, TxData, TxEmptyResultHandler, TxFromSpecified, TxGas, TxPayment,
        TxPaymentEgldOnly, TxResultHandler, TxScEnv, TxToSpecified,
    },
};

use super::DeployRawResult;

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

impl<Api, From, Payment, Gas, CodePathValue, RH>
    Tx<TxScEnv<Api>, From, (), Payment, Gas, DeployCall<TxScEnv<Api>, CodePath<CodePathValue>>, RH>
where
    Api: CallTypeApi + TestApi,
    From: TxFromSpecified<TxScEnv<Api>>,
    Payment: TxPaymentEgldOnly<TxScEnv<Api>>,
    Gas: TxGas<TxScEnv<Api>>,
    CodePathValue: TxCodePathValue<TxScEnv<Api>>,
    RH: TxResultHandler<TxScEnv<Api>>,
{
    fn execute_test_deploy_raw(self) -> (ManagedAddress<Api>, RH) {
        let gas_limit = self.gas.gas_value(&self.env);
        let new_address = self.from.with_value_ref(&self.env, |from| {
            self.payment.with_egld_value(&self.env, |egld_value| {
                TestRawWrapper::<Api>::new().deploy_contract(
                    from,
                    gas_limit,
                    egld_value,
                    &self.data.code_source.0.into_value(&self.env),
                    &self.data.arg_buffer,
                )
            })
        });
        (new_address, self.result_handler)
    }
}

impl<Api, From, Payment, Gas, CodePathValue, RH>
    Tx<TxScEnv<Api>, From, (), Payment, Gas, DeployCall<TxScEnv<Api>, CodePath<CodePathValue>>, RH>
where
    Api: CallTypeApi + TestApi,
    From: TxFromSpecified<TxScEnv<Api>>,
    Payment: TxPaymentEgldOnly<TxScEnv<Api>>,
    Gas: TxGas<TxScEnv<Api>>,
    CodePathValue: TxCodePathValue<TxScEnv<Api>>,
    RH: RHListExec<DeployRawResult<Api>, TxScEnv<Api>>,
    RH::ListReturns: NestedTupleFlatten,
{
    /// Synchronously deploys a contract.
    pub fn test_deploy(self) -> <RH::ListReturns as NestedTupleFlatten>::Unpacked {
        let (new_address, result_handler) = self.execute_test_deploy_raw();

        // TODO: results currently not retrieved
        let raw_results = ManagedVec::new();

        let deploy_raw_result = DeployRawResult {
            new_address,
            raw_results,
        };
        let tuple_result = result_handler.list_process_result(&deploy_raw_result);
        tuple_result.flatten_unpack()
    }
}
