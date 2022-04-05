use crate::{
    num_bigint::BigUint,
    tx_execution::execute_sc_query,
    tx_mock::{generate_tx_hash_dummy, TxInput},
    world_mock::BlockchainMock,
};
use mandos::model::{ScQueryStep, Step};

use super::check_tx_output;

impl BlockchainMock {
    pub fn mandos_sc_query(&mut self, sc_query_step: ScQueryStep) -> &mut Self {
        self.with_borrowed(|state| ((), execute(state, &sc_query_step)));
        self.mandos_trace.steps.push(Step::ScQuery(sc_query_step));
        self
    }
}

fn execute(state: BlockchainMock, sc_query_step: &ScQueryStep) -> BlockchainMock {
    let tx_input = TxInput {
        from: sc_query_step.tx.to.value.into(),
        to: sc_query_step.tx.to.value.into(),
        egld_value: BigUint::from(0u32),
        esdt_values: Vec::new(),
        func_name: sc_query_step.tx.function.as_bytes().to_vec(),
        args: sc_query_step
            .tx
            .arguments
            .iter()
            .map(|scen_arg| scen_arg.value.clone())
            .collect(),
        gas_limit: u64::MAX,
        gas_price: 0u64,
        tx_hash: generate_tx_hash_dummy(&sc_query_step.tx_id),
    };

    let (tx_result, state) = execute_sc_query(tx_input, state);
    assert!(
        tx_result.result_calls.is_empty(),
        "Can't query a view function that performs an async call"
    );
    if let Some(tx_expect) = &sc_query_step.expect {
        check_tx_output(&sc_query_step.tx_id, tx_expect, &tx_result);
    }

    state
}
