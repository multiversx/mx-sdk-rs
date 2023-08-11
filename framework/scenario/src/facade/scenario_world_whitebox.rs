use multiversx_chain_vm::tx_mock::{TxFunctionName, TxResult};
use multiversx_sc::contract_base::{CallableContract, ContractBase};

use crate::{
    debug_executor::contract_instance_wrapped_execution,
    scenario_model::{ScCallStep, ScDeployStep, ScQueryStep},
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
        let debugger_backend = self.get_mut_debugger_backend();
        let tx_result = debugger_backend
            .vm_runner
            .perform_sc_query_lambda_and_check(&sc_query_step, || {
                catch_whitebox_panic(|| {
                    f(contract_obj);
                });
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
        F: FnOnce(ContractObj),
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
        F: FnOnce(ContractObj),
        C: FnOnce(TxResult),
    {
        // the recipient can be deduced from the contract object, it is redundant to provide it in the step
        let mut sc_call_step = sc_call_step.to(&whitebox_contract.address_expr);

        // no endpoint is called per se, but if it is empty, the VM thinks it is a simple transfer of value
        if sc_call_step.tx.function.is_empty() {
            sc_call_step.tx.function = TxFunctionName::WHITEBOX_CALL.to_string();
        }

        let contract_obj = (whitebox_contract.contract_obj_builder)();
        let debugger_backend = self.get_mut_debugger_backend();
        let tx_result =
            debugger_backend
                .vm_runner
                .perform_sc_call_lambda_and_check(&sc_call_step, || {
                    catch_whitebox_panic(|| {
                        f(contract_obj);
                    });
                });
        check_result(tx_result);
        self
    }

    pub fn whitebox_deploy<ContractObj, F>(
        &mut self,
        whitebox_contract: &WhiteboxContract<ContractObj>,
        sc_deploy_step: ScDeployStep,
        f: F,
    ) -> &mut Self
    where
        ContractObj: ContractBase<Api = DebugApi> + CallableContract + 'static,
        F: FnOnce(ContractObj),
    {
        self.whitebox_deploy_check(whitebox_contract, sc_deploy_step, f, |tx_result| {
            tx_result.assert_ok();
        })
    }

    pub fn whitebox_deploy_check<ContractObj, F, C>(
        &mut self,
        whitebox_contract: &WhiteboxContract<ContractObj>,
        sc_deploy_step: ScDeployStep,
        f: F,
        check_result: C,
    ) -> &mut Self
    where
        ContractObj: ContractBase<Api = DebugApi> + CallableContract + 'static,
        F: FnOnce(ContractObj),
        C: FnOnce(TxResult),
    {
        let contract_obj = (whitebox_contract.contract_obj_builder)();
        let debugger_backend = self.get_mut_debugger_backend();
        let (_, tx_result) = debugger_backend
            .vm_runner
            .perform_sc_deploy_lambda_and_check(&sc_deploy_step, || {
                catch_whitebox_panic(|| {
                    f(contract_obj);
                });
            });
        check_result(tx_result);
        self
    }
}

fn catch_whitebox_panic<F>(f: F)
where
    F: FnOnce(),
{
    contract_instance_wrapped_execution(true, || {
        f();
        Ok(())
    });
}
