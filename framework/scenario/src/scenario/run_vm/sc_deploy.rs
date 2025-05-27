use crate::{
    multiversx_sc::types::heap::Address, scenario::model::ScDeployStep, scenario_model::TxResponse,
};

use multiversx_chain_vm::{
    host::{
        context::{TxFunctionName, TxInput, TxResult},
        execution,
        runtime::{RuntimeInstanceCallLambda, RuntimeInstanceCallLambdaDefault},
    },
    types::VMCodeMetadata,
};

use super::{check_tx_output, tx_input_util::generate_tx_hash, ScenarioVMRunner};

impl ScenarioVMRunner {
    /// Adds a SC deploy step, as specified in the `step` argument, then executes it.
    ///
    /// The result of the operation gets saved back in the step's response field.
    pub fn perform_sc_deploy_update_results(&mut self, step: &mut ScDeployStep) {
        let (new_address, tx_result) =
            self.perform_sc_deploy_lambda_and_check(step, RuntimeInstanceCallLambdaDefault);
        let mut response = TxResponse::from_tx_result(tx_result);
        response.new_deployed_address = Some(new_address);
        step.save_response(response);
    }

    pub fn perform_sc_deploy_lambda<F>(
        &mut self,
        sc_deploy_step: &ScDeployStep,
        f: F,
    ) -> (Address, TxResult)
    where
        F: RuntimeInstanceCallLambda,
    {
        let tx_input = tx_input_from_deploy(sc_deploy_step, f.override_function_name());
        let runtime = self.create_debugger_runtime();
        let contract_code = &sc_deploy_step.tx.contract_code.value;
        let (new_address, tx_result) = execution::commit_deploy(
            tx_input,
            contract_code,
            VMCodeMetadata::from(sc_deploy_step.tx.code_metadata.bits()),
            &mut self.blockchain_mock.state,
            &runtime,
            f,
        );
        assert!(
            tx_result.pending_calls.no_calls(),
            "Async calls from constructors are currently not supported"
        );
        (new_address, tx_result)
    }

    pub fn perform_sc_deploy_lambda_and_check<F>(
        &mut self,
        sc_deploy_step: &ScDeployStep,
        f: F,
    ) -> (Address, TxResult)
    where
        F: RuntimeInstanceCallLambda,
    {
        let (new_address, tx_result) = self.perform_sc_deploy_lambda(sc_deploy_step, f);
        if let Some(tx_expect) = &sc_deploy_step.expect {
            check_tx_output(&sc_deploy_step.id, tx_expect, &tx_result);
        }
        (new_address, tx_result)
    }
}

fn tx_input_from_deploy(
    sc_deploy_step: &ScDeployStep,
    override_func_name: Option<TxFunctionName>,
) -> TxInput {
    let tx = &sc_deploy_step.tx;
    TxInput {
        from: tx.from.to_address(),
        to: multiversx_chain_vm::types::VMAddress::zero(),
        egld_value: tx.egld_value.value.clone(),
        esdt_values: Vec::new(),
        func_name: override_func_name.unwrap_or(TxFunctionName::INIT),
        args: tx
            .arguments
            .iter()
            .map(|scen_arg| scen_arg.value.clone())
            .collect(),
        gas_limit: tx.gas_limit.value,
        gas_price: tx.gas_price.value,
        tx_hash: generate_tx_hash(&sc_deploy_step.id, &sc_deploy_step.explicit_tx_hash),
        ..Default::default()
    }
}
