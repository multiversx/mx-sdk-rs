use elrond_wasm::types::Address;
use mandos::model::{TxDeploy, TxExpect};

use crate::{
    tx_execution::sc_create,
    tx_mock::{generate_tx_hash_dummy, TxInput},
    world_mock::BlockchainMock,
    ContractMap, DebugApi,
};

use super::check_tx_output;

pub fn execute(
    state: &mut BlockchainMock,
    contract_map: &ContractMap<DebugApi>,
    tx_id: &str,
    tx: &TxDeploy,
    expect: &Option<TxExpect>,
) {
    let tx_input = TxInput {
        from: tx.from.value.into(),
        to: Address::zero(),
        egld_value: tx.egld_value.value.clone(),
        esdt_values: Vec::new(),
        func_name: b"init".to_vec(),
        args: tx
            .arguments
            .iter()
            .map(|scen_arg| scen_arg.value.clone())
            .collect(),
        gas_limit: tx.gas_limit.value,
        gas_price: tx.gas_price.value,
        tx_hash: generate_tx_hash_dummy(tx_id),
    };
    state.increase_nonce(&tx_input.from);
    let (tx_result, _) = sc_create(tx_input, &tx.contract_code.value, state, contract_map).unwrap();
    if let Some(tx_expect) = expect {
        check_tx_output(tx_id, tx_expect, &tx_result);
    }
}
