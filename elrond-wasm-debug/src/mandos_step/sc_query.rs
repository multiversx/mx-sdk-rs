use std::rc::Rc;

use mandos::model::{TxExpect, TxQuery};
use num_bigint::BigUint;

use crate::{
    tx_execution::sc_query,
    tx_mock::{generate_tx_hash_dummy, TxInput},
    world_mock::BlockchainMock,
};

use super::check_tx_output;

pub fn execute(state: Rc<BlockchainMock>, tx_id: &str, tx: &TxQuery, expect: &Option<TxExpect>) {
    let tx_input = TxInput {
        from: tx.to.value.into(),
        to: tx.to.value.into(),
        egld_value: BigUint::from(0u32),
        esdt_values: Vec::new(),
        func_name: tx.function.as_bytes().to_vec(),
        args: tx
            .arguments
            .iter()
            .map(|scen_arg| scen_arg.value.clone())
            .collect(),
        gas_limit: u64::MAX,
        gas_price: 0u64,
        tx_hash: generate_tx_hash_dummy(tx_id),
    };

    let tx_result = sc_query(tx_input, state);
    assert!(
        tx_result.result_status != 0 || tx_result.result_calls.is_empty(),
        "Can't query a view function that performs an async call"
    );
    if let Some(tx_expect) = expect {
        check_tx_output(tx_id, tx_expect, &tx_result);
    }
}
