use crate::{
    multiversx_sc::{
        codec::{CodecFrom, PanicErrorHandler, TopEncodeMulti},
        types::heap::Address,
    },
    scenario::model::{ScDeployStep, TypedScDeploy},
    scenario_model::TxResponse,
};

use multiversx_chain_vm::{
    tx_execution::execute_current_tx_context_input,
    tx_mock::{TxFunctionName, TxInput, TxResult},
};

use super::{check_tx_output, tx_input_util::generate_tx_hash, ScenarioVMRunner};

impl ScenarioVMRunner {
    pub fn perform_sc_deploy_lambda<F>(
        &mut self,
        sc_deploy_step: &ScDeployStep,
        f: F,
    ) -> (Address, TxResult)
    where
        F: FnOnce(),
    {
        let tx_input = tx_input_from_deploy(sc_deploy_step);
        let contract_code = &sc_deploy_step.tx.contract_code.value;
        let (new_address, tx_result) = self.blockchain_mock.vm.sc_create(
            tx_input,
            contract_code,
            &mut self.blockchain_mock.state,
            f,
        );
        assert!(
            tx_result.pending_calls.no_calls(),
            "Async calls from constructors are currently not supported"
        );
        (new_address.as_array().into(), tx_result)
    }

    pub fn perform_sc_deploy_lambda_and_check<F>(
        &mut self,
        sc_deploy_step: &ScDeployStep,
        f: F,
    ) -> (Address, TxResult)
    where
        F: FnOnce(),
    {
        let (new_address, tx_result) = self.perform_sc_deploy_lambda(sc_deploy_step, f);
        if let Some(tx_expect) = &sc_deploy_step.expect {
            check_tx_output(&sc_deploy_step.id, tx_expect, &tx_result);
        }
        (new_address, tx_result)
    }

    /// Adds a SC deploy step, as specified in the `sc_deploy_step` argument, then executes it.
    pub fn perform_sc_deploy(&mut self, sc_deploy_step: &ScDeployStep) {
        let _ = self
            .perform_sc_deploy_lambda_and_check(sc_deploy_step, execute_current_tx_context_input);
    }

    /// Adds a SC deploy step, executes it and retrieves the transaction result ("out" field).
    ///
    /// The transaction is expected to complete successfully.
    ///
    /// It takes the `contract_call` argument separately from the SC call step,
    /// so we can benefit from type inference in the result.
    pub fn perform_sc_deploy_get_result<OriginalResult, RequestedResult>(
        &mut self,
        typed_sc_deploy: TypedScDeploy<OriginalResult>,
    ) -> (Address, RequestedResult)
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>,
    {
        let sc_deploy_step: ScDeployStep = typed_sc_deploy.into();
        let (new_address, tx_result) =
            self.perform_sc_deploy_lambda(&sc_deploy_step, execute_current_tx_context_input);
        let mut raw_result = tx_result.result_values;
        let deser_result =
            RequestedResult::multi_decode_or_handle_err(&mut raw_result, PanicErrorHandler)
                .unwrap();

        (new_address, deser_result)
    }

    pub fn perform_sc_deploy_update_results(&mut self, sc_deploy_step: &mut ScDeployStep) {
        let (new_address, tx_result) = self
            .perform_sc_deploy_lambda_and_check(sc_deploy_step, execute_current_tx_context_input);
        let mut response = TxResponse::from_tx_result(tx_result);
        response.new_deployed_address = Some(new_address);
        sc_deploy_step.response = Some(response);
    }
}

fn tx_input_from_deploy(sc_deploy_step: &ScDeployStep) -> TxInput {
    let tx = &sc_deploy_step.tx;
    TxInput {
        from: tx.from.to_vm_address(),
        to: multiversx_chain_vm::types::VMAddress::zero(),
        egld_value: tx.egld_value.value.clone(),
        esdt_values: Vec::new(),
        func_name: TxFunctionName::INIT,
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
