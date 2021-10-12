use mandos::model::{TxExpect, TxQuery};
use num_bigint::BigUint;

use crate::{
    execute_helper_functions::{check_tx_output, generate_tx_hash_dummy, sc_call},
    tx_mock::{TxContext, TxInput},
    world_mock::BlockchainMock,
    ContractMap,
};

pub fn execute(
    state: &mut BlockchainMock,
    contract_map: &ContractMap<TxContext>,
    tx_id: &str,
    tx: &TxQuery,
    expect: &Option<TxExpect>,
) {
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

    let (tx_result, opt_async_data) = sc_call(tx_input, state, contract_map).unwrap();
    assert!(
        tx_result.result_status != 0 || !opt_async_data.is_some(),
        "Can't query a view function that performs an async call"
    );
    if let Some(tx_expect) = expect {
        check_tx_output(tx_id, tx_expect, &tx_result);
    }
}
