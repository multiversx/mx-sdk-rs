use crate::{
    multiversx_sc::codec::{CodecFrom, PanicErrorHandler, TopEncodeMulti},
    scenario::model::{ScCallStep, TxESDT, TypedScCall},
    scenario_model::TxResponse,
};

use multiversx_chain_vm::{
    tx_execution::execute_current_tx_context_input,
    tx_mock::{TxInput, TxResult, TxTokenTransfer},
};

use super::{check_tx_output, tx_input_util::generate_tx_hash, ScenarioVMRunner};

impl ScenarioVMRunner {
    /// Adds a SC call step, as specified in the `step` argument, then executes it.
    ///
    /// The result of the operation gets saved back in the step's response field.
    pub fn perform_sc_call_update_results(&mut self, step: &mut ScCallStep) {
        let tx_result =
            self.perform_sc_call_lambda_and_check(step, execute_current_tx_context_input);
        let response = TxResponse::from_tx_result(tx_result);
        step.save_response(response);
    }

    /// Adds a SC call step, executes it and retrieves the transaction result ("out" field).
    ///
    /// The transaction is expected to complete successfully.
    ///
    /// It takes the `contract_call` argument separately from the SC call step,
    /// so we can benefit from type inference in the result.
    pub fn perform_sc_call_get_result<OriginalResult, RequestedResult>(
        &mut self,
        typed_sc_call: TypedScCall<OriginalResult>,
    ) -> RequestedResult
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>,
    {
        let sc_call_step: ScCallStep = typed_sc_call.into();
        let tx_result =
            self.perform_sc_call_lambda(&sc_call_step, execute_current_tx_context_input);
        let mut raw_result = tx_result.result_values;
        RequestedResult::multi_decode_or_handle_err(&mut raw_result, PanicErrorHandler).unwrap()
    }

    pub fn perform_sc_call_lambda<F>(&mut self, sc_call_step: &ScCallStep, f: F) -> TxResult
    where
        F: FnOnce(),
    {
        let tx_input = tx_input_from_call(sc_call_step);

        // nonce gets increased irrespective of whether the tx fails or not
        self.blockchain_mock
            .state
            .increase_account_nonce(&tx_input.from);

        self.blockchain_mock.vm.sc_call_with_async_and_callback(
            tx_input,
            &mut self.blockchain_mock.state,
            f,
        )
    }

    pub fn perform_sc_call_lambda_and_check<F>(
        &mut self,
        sc_call_step: &ScCallStep,
        f: F,
    ) -> TxResult
    where
        F: FnOnce(),
    {
        let tx_result = self.perform_sc_call_lambda(sc_call_step, f);
        if let Some(tx_expect) = &sc_call_step.expect {
            check_tx_output(&sc_call_step.id, tx_expect, &tx_result);
        }
        tx_result
    }
}

fn tx_input_from_call(sc_call_step: &ScCallStep) -> TxInput {
    let tx = &sc_call_step.tx;
    TxInput {
        from: tx.from.to_vm_address(),
        to: tx.to.to_vm_address(),
        egld_value: tx.egld_value.value.clone(),
        esdt_values: tx_esdt_transfers_from_scenario(tx.esdt_value.as_slice()),
        func_name: tx.function.clone().into(),
        args: tx
            .arguments
            .iter()
            .map(|scen_arg| scen_arg.value.clone())
            .collect(),
        gas_limit: tx.gas_limit.value,
        gas_price: tx.gas_price.value,
        tx_hash: generate_tx_hash(&sc_call_step.id, &sc_call_step.explicit_tx_hash),
        ..Default::default()
    }
}

pub fn tx_esdt_transfers_from_scenario(scenario_transf_esdt: &[TxESDT]) -> Vec<TxTokenTransfer> {
    scenario_transf_esdt
        .iter()
        .map(tx_esdt_transfer_from_scenario)
        .collect()
}

pub fn tx_esdt_transfer_from_scenario(scenario_transf_esdt: &TxESDT) -> TxTokenTransfer {
    TxTokenTransfer {
        token_identifier: scenario_transf_esdt.esdt_token_identifier.value.clone(),
        nonce: scenario_transf_esdt.nonce.value,
        value: scenario_transf_esdt.esdt_value.value.clone(),
    }
}
