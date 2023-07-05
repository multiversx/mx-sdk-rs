use multiversx_chain_vm::tx_mock::{TxFunctionName, TxResult};
use multiversx_sc::contract_base::{CallableContract, ContractBase};

use crate::{
    scenario_model::{ScCallStep, ScQueryStep},
    DebugApi, ScenarioWorld,
};

use super::whitebox_contract::WhiteboxContract;

impl ScenarioWorld {
    pub fn whitebox_query<ContractObj, F>(
        &mut self,
        whitebox_contract: &WhiteboxContract<ContractObj>,
        f: F,
    ) -> &mut Self
    where
        ContractObj: ContractBase<Api = DebugApi> + CallableContract + 'static,
        F: FnOnce(ContractObj),
    {
        self.whitebox_query_check(whitebox_contract, f, |tx_result| {
            tx_result.assert_ok();
        })
    }

    pub fn whitebox_query_check<ContractObj, F, C>(
        &mut self,
        whitebox_contract: &WhiteboxContract<ContractObj>,
        f: F,
        check_result: C,
    ) -> &mut Self
    where
        ContractObj: ContractBase<Api = DebugApi> + CallableContract + 'static,
        F: FnOnce(ContractObj),
        C: FnOnce(TxResult),
    {
        let sc_query_step = ScQueryStep::new().to(&whitebox_contract.address_expr);
        let contract_obj = (whitebox_contract.contract_obj_builder)();
        let debugger_backend = self.get_mut_contract_debugger_backend();
        let tx_result = debugger_backend
            .vm_runner
            .perform_sc_query_lambda_and_check(&sc_query_step, || {
                f(contract_obj);
            });
        check_result(tx_result);

        self
    }

    pub fn whitebox_call<ContractObj, F>(
        &mut self,
        whitebox_contract: &WhiteboxContract<ContractObj>,
        sc_call_step: ScCallStep,
        f: F,
    ) -> &mut Self
    where
        ContractObj: ContractBase<Api = DebugApi> + CallableContract + 'static,
        F: FnOnce(ContractObj) + 'static,
    {
        self.whitebox_call_check(whitebox_contract, sc_call_step, f, |tx_result| {
            tx_result.assert_ok();
        })
    }

    pub fn whitebox_call_check<ContractObj, F, C>(
        &mut self,
        whitebox_contract: &WhiteboxContract<ContractObj>,
        sc_call_step: ScCallStep,
        f: F,
        check_result: C,
    ) -> &mut Self
    where
        ContractObj: ContractBase<Api = DebugApi> + CallableContract + 'static,
        F: FnOnce(ContractObj) + 'static,
        C: FnOnce(TxResult),
    {
        // the recipient can be deduced from the contract object, it is redundant to provide it in the step
        let mut sc_call_step = sc_call_step.to(&whitebox_contract.address_expr);

        // no endpoint is called per se, but if it is empty, the VM thinks it is a simple transfer of value
        if sc_call_step.tx.function.is_empty() {
            sc_call_step.tx.function = TxFunctionName::WHITEBOX_CALL.to_string();
        }

        let contract_obj = (whitebox_contract.contract_obj_builder)();
        let debugger_backend = self.get_mut_contract_debugger_backend();
        let tx_result =
            debugger_backend
                .vm_runner
                .perform_sc_call_lambda_and_check(&sc_call_step, || {
                    f(contract_obj);
                });
        check_result(tx_result);
        self
    }
}
