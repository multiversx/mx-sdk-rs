use crate::{
    api::DebugApi,
    multiversx_sc::{
        codec::{CodecFrom, PanicErrorHandler},
        types::ContractCall,
    },
    num_bigint::BigUint,
    scenario::model::ScQueryStep,
};
use multiversx_chain_vm::{
    tx_execution::execute_current_tx_context_input,
    tx_mock::{generate_tx_hash_dummy, TxInput, TxResult},
};

use super::{check_tx_output, ScenarioVMRunner};

impl ScenarioVMRunner {
    /// Adds a SC query step, as specified in the `sc_query_step` argument, then executes it.
    pub fn perform_sc_query(&mut self, sc_query_step: &ScQueryStep) -> TxResult {
        self.perform_sc_query_lambda_and_check(sc_query_step, execute_current_tx_context_input)
    }

    pub fn perform_sc_query_lambda<F>(&mut self, sc_query_step: &ScQueryStep, f: F) -> TxResult
    where
        F: FnOnce(),
    {
        let tx_input = tx_input_from_query(sc_query_step);
        let tx_result = self.blockchain_mock.vm.execute_sc_query_lambda(
            tx_input,
            &mut self.blockchain_mock.state,
            f,
        );
        assert!(
            tx_result.pending_calls.no_calls(),
            "Can't query a view function that performs an async call"
        );
        tx_result
    }

    pub fn perform_sc_query_lambda_and_check<F>(
        &mut self,
        sc_query_step: &ScQueryStep,
        f: F,
    ) -> TxResult
    where
        F: FnOnce(),
    {
        let tx_result = self.perform_sc_query_lambda(sc_query_step, f);
        if let Some(tx_expect) = &sc_query_step.expect {
            check_tx_output(&sc_query_step.id, tx_expect, &tx_result);
        }
        tx_result
    }
}

impl ScenarioVMRunner {
    /// Performs a SC query to a contract, leaves no scenario trace behind.
    ///
    /// Meant to be used for the test to investigate the state of the contract.
    ///
    /// Use `mandos_sc_query` to embed the SC query in the resulting scenario.
    pub fn quick_query<CC, RequestedResult>(&mut self, contract_call: CC) -> RequestedResult
    where
        CC: ContractCall<DebugApi>,
        RequestedResult: CodecFrom<CC::OriginalResult>,
    {
        let sc_query_step = ScQueryStep::new().call(contract_call);
        let tx_result = self.perform_sc_query(&sc_query_step);
        let mut raw_result = tx_result.result_values;
        RequestedResult::multi_decode_or_handle_err(&mut raw_result, PanicErrorHandler).unwrap()
    }
}

fn tx_input_from_query(sc_query_step: &ScQueryStep) -> TxInput {
    TxInput {
        from: sc_query_step.tx.to.to_vm_address(),
        to: sc_query_step.tx.to.to_vm_address(),
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
        tx_hash: generate_tx_hash_dummy(&sc_query_step.id),
        ..Default::default()
    }
}
