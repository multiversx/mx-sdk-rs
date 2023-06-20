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
    tx_execution::execute_sc_query,
    tx_mock::{generate_tx_hash_dummy, TxInput, TxResult},
    world_mock::BlockchainMock,
};

use super::{check_tx_output, ScenarioVMRunner};

impl ScenarioVMRunner {
    /// Adds a SC query step, as specified in the `sc_query_step` argument, then executes it.
    pub fn perform_sc_query(&mut self, sc_query_step: &ScQueryStep) -> TxResult {
        self.blockchain_mock
            .with_borrowed(|state| execute_and_check(state, sc_query_step))
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
        let tx_result = self
            .blockchain_mock
            .with_borrowed(|state| execute(state, &sc_query_step));
        let mut raw_result = tx_result.result_values;
        RequestedResult::multi_decode_or_handle_err(&mut raw_result, PanicErrorHandler).unwrap()
    }
}

pub(crate) fn execute(
    state: BlockchainMock,
    sc_query_step: &ScQueryStep,
) -> (TxResult, BlockchainMock) {
    let tx_input = TxInput {
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
        tx_hash: generate_tx_hash_dummy(&sc_query_step.id),
        ..Default::default()
    };

    let (tx_result, state) = execute_sc_query(tx_input, state);
    assert!(
        tx_result.pending_calls.no_calls(),
        "Can't query a view function that performs an async call"
    );
    (tx_result, state)
}

fn execute_and_check(
    state: BlockchainMock,
    sc_query_step: &ScQueryStep,
) -> (TxResult, BlockchainMock) {
    let (tx_result, state) = execute(state, sc_query_step);
    if let Some(tx_expect) = &sc_query_step.expect {
        check_tx_output(&sc_query_step.id, tx_expect, &tx_result);
    }

    (tx_result, state)
}
