use crate::scenario::model::{ScCallStep, Step, TxESDT, TypedScCall, TypedScCallExecutor};
use multiversx_sc::codec::{CodecFrom, PanicErrorHandler, TopEncodeMulti};

use crate::{
    tx_execution::sc_call_with_async_and_callback,
    tx_mock::{generate_tx_hash_dummy, TxInput, TxResult, TxTokenTransfer},
    world_mock::BlockchainMock,
};

use super::check_tx_output;

impl BlockchainMock {
    /// Adds a SC call step, as specified in the `sc_call_step` argument, then executes it.
    pub fn perform_sc_call(&mut self, sc_call_step: ScCallStep) -> &mut Self {
        let _ = self.with_borrowed(|state| execute_and_check(state, &sc_call_step));
        self.scenario_trace.steps.push(Step::ScCall(sc_call_step));
        self
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
        let tx_result = self.with_borrowed(|state| execute_and_check(state, &sc_call_step));
        self.scenario_trace.steps.push(Step::ScCall(sc_call_step));
        let mut raw_result = tx_result.result_values;
        RequestedResult::multi_decode_or_handle_err(&mut raw_result, PanicErrorHandler).unwrap()
    }
}

impl TypedScCallExecutor for BlockchainMock {
    fn execute_typed_sc_call<OriginalResult, RequestedResult>(
        &mut self,
        typed_sc_call: TypedScCall<OriginalResult>,
    ) -> RequestedResult
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>,
    {
        self.perform_sc_call_get_result(typed_sc_call)
    }
}

pub(crate) fn execute(
    mut state: BlockchainMock,
    sc_call_step: &ScCallStep,
) -> (TxResult, BlockchainMock) {
    let tx = &sc_call_step.tx;
    let tx_input = TxInput {
        from: tx.from.to_address(),
        to: tx.to.to_address(),
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
        tx_hash: generate_tx_hash_dummy(&sc_call_step.id),
        ..Default::default()
    };

    // nonce gets increased irrespective of whether the tx fails or not
    state.increase_account_nonce(&tx_input.from);

    sc_call_with_async_and_callback(tx_input, state)
}

fn execute_and_check(
    state: BlockchainMock,
    sc_call_step: &ScCallStep,
) -> (TxResult, BlockchainMock) {
    let (tx_result, state) = execute(state, sc_call_step);
    if let Some(tx_expect) = &sc_call_step.expect {
        check_tx_output(&sc_call_step.id, tx_expect, &tx_result);
    }
    (tx_result, state)
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
