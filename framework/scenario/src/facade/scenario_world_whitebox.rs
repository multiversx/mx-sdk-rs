use multiversx_chain_vm::tx_mock::TxResult;
use multiversx_sc::contract_base::{CallableContract, ContractBase};

use crate::{scenario_model::ScQueryStep, DebugApi, ScenarioWorld};

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
}
