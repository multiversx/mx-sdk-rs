use crate::{
    executor::debug::ContractDebugInstance, num_bigint::BigUint, scenario::model::ScQueryStep,
    scenario_model::TxResponse,
};
use multiversx_chain_vm::host::{
    context::{TxInput, TxResult},
    execution,
    runtime::instance_call,
};

use super::{tx_input_util::generate_tx_hash, ScenarioVMRunner};

impl ScenarioVMRunner {
    /// Adds a SC query step, as specified in the `sc_query_step` argument, then executes it.
    ///
    /// The result of the operation gets saved back in the step's response field.
    pub fn perform_sc_query_update_results(&mut self, step: &mut ScQueryStep) {
        let tx_result = self.perform_sc_query_in_debugger(step);
        let response = TxResponse::from_tx_result(tx_result);
        step.save_response(response);
    }

    pub fn perform_sc_query_in_debugger(&mut self, step: &ScQueryStep) -> TxResult {
        let tx_input = tx_input_from_query(step);
        let runtime = self.create_debugger_runtime();
        let tx_result = execution::execute_query(
            tx_input,
            &mut self.blockchain_mock.state,
            &runtime,
            instance_call,
        );

        assert!(
            tx_result.pending_calls.no_calls(),
            "Can't query a view function that performs an async call"
        );
        tx_result
    }

    pub fn perform_sc_query_lambda_in_debugger<F>(&mut self, step: &ScQueryStep, f: F) -> TxResult
    where
        F: FnOnce(),
    {
        let tx_input = tx_input_from_query(step);
        let runtime = self.create_debugger_runtime();
        // let tx_context = TxContext::new(runtime.clone(), tx_input, tx_cache);
        let tx_result = execution::execute_query(
            tx_input,
            &mut self.blockchain_mock.state,
            &runtime,
            |instance_call| {
                ContractDebugInstance::wrap_lambda_call(true, instance_call, f);
            },
        );

        assert!(
            tx_result.pending_calls.no_calls(),
            "Can't query a view function that performs an async call"
        );
        tx_result
    }
}

fn tx_input_from_query(sc_query_step: &ScQueryStep) -> TxInput {
    TxInput {
        from: sc_query_step.tx.to.to_address(),
        to: sc_query_step.tx.to.to_address(),
        egld_value: BigUint::from(0u32),
        esdt_values: Vec::new(),
        func_name: sc_query_step.tx.function.clone().into(),
        args: sc_query_step
            .tx
            .arguments
            .iter()
            .map(|scen_arg| scen_arg.value.clone())
            .collect(),
        gas_limit: u64::MAX,
        gas_price: 0u64,
        tx_hash: generate_tx_hash(&sc_query_step.id, &sc_query_step.explicit_tx_hash),
        ..Default::default()
    }
}
