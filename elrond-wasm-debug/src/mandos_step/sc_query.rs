use crate::{
    num_bigint::BigUint,
    tx_execution::sc_query,
    tx_mock::{generate_tx_hash_dummy, TxInput},
    world_mock::BlockchainMock,
};
use mandos::model::ScQueryStep;
use std::rc::Rc;

use super::check_tx_output;

impl BlockchainMock {
    pub fn mandos_sc_query(self, sc_query_step: ScQueryStep) -> BlockchainMock {
        let state_rc = Rc::new(self);
        execute_rc(state_rc.clone(), &sc_query_step);
        Rc::try_unwrap(state_rc).unwrap()
    }
}

fn execute_rc(state: Rc<BlockchainMock>, sc_query_step: &ScQueryStep) {
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

    let tx_result = sc_query(tx_input, state);
    assert!(
        tx_result.result_status != 0 || tx_result.result_calls.is_empty(),
        "Can't query a view function that performs an async call"
    );
    if let Some(tx_expect) = &sc_query_step.expect {
        check_tx_output(&sc_query_step.tx_id, tx_expect, &tx_result);
    }
}
