use elrond_wasm::types::Address;
use mandos::{TxDeploy, TxExpect};

use crate::{execute_helper_functions::*, BlockchainMock, ContractMap, TxContext, TxInput};

pub fn execute(
    state: &mut BlockchainMock,
    contract_map: &ContractMap<TxContext>,
    tx_id: &String,
    tx: &Box<TxDeploy>,
    expect: &Option<TxExpect>,
) {
    let tx_input = TxInput {
        from: tx.from.value.into(),
        to: Address::zero(),
        call_value: tx.call_value.value.clone(),
        esdt_value: tx.esdt_value.value.clone(),
        esdt_token_identifier: tx.esdt_token_name.value.clone(),
        func_name: b"init".to_vec(),
        args: tx
            .arguments
            .iter()
            .map(|scen_arg| scen_arg.value.clone())
            .collect(),
        gas_limit: tx.gas_limit.value,
        gas_price: tx.gas_price.value,
        tx_hash: generate_tx_hash_dummy(tx_id.as_str()),
    };
    state.increase_nonce(&tx_input.from);
    let (tx_result, _) =
        execute_sc_create(tx_input, &tx.contract_code.value, state, contract_map).unwrap();
    if let Some(tx_expect) = expect {
        check_tx_output(tx_id.as_str(), tx_expect, &tx_result);
    }
}
