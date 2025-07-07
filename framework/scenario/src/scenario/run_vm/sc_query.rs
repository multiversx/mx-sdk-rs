use crate::{
    num_bigint::BigUint,
    scenario::{model::ScQueryStep, run_vm::tx_output_check::check_tx_output},
    scenario_model::TxResponse,
};
use multiversx_chain_vm::host::{
    context::{TxFunctionName, TxInput, TxResult},
    execution,
    runtime::{RuntimeInstanceCallLambda, RuntimeInstanceCallLambdaDefault},
};

use super::{tx_input_util::generate_tx_hash, ScenarioVMRunner};

impl ScenarioVMRunner {
    /// Adds a SC query step, as specified in the `sc_query_step` argument, then executes it.
    ///
    /// The result of the operation gets saved back in the step's response field.
    pub fn perform_sc_query_update_results(&mut self, step: &mut ScQueryStep) {
        let tx_result = self.perform_sc_query_in_debugger(step, RuntimeInstanceCallLambdaDefault);
        if let Some(tx_expect) = &step.expect {
            check_tx_output(&step.id, tx_expect, &tx_result);
        }
        let response: TxResponse = TxResponse::from_tx_result(tx_result);
        step.save_response(response);
    }

    pub fn perform_sc_query_in_debugger<F>(&mut self, step: &ScQueryStep, f: F) -> TxResult
    where
        F: RuntimeInstanceCallLambda,
    {
        let tx_input = tx_input_from_query(step, f.override_function_name());
        let runtime = self.create_debugger_runtime();
        let tx_result =
            execution::execute_query(tx_input, &mut self.blockchain_mock.state, &runtime, f);

        assert!(
            tx_result.pending_calls.no_calls(),
            "Can't query a view function that performs an async call"
        );
        tx_result
    }
}

fn tx_input_from_query(
    sc_query_step: &ScQueryStep,
    override_func_name: Option<TxFunctionName>,
) -> TxInput {
    TxInput {
        from: sc_query_step.tx.to.to_address(),
        to: sc_query_step.tx.to.to_address(),
        egld_value: BigUint::from(0u32),
        esdt_values: Vec::new(),
        func_name: override_func_name.unwrap_or_else(|| sc_query_step.tx.function.clone().into()),
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
