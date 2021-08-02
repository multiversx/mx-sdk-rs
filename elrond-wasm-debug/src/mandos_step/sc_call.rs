use crate::execute_helper_functions::{self, *};
use mandos::{TxCall, TxExpect};
use num_bigint::BigUint;
use num_traits::Zero;

use crate::{BlockchainMock, ContractMap, TxContext, TxInput};

pub fn execute(
    state: &mut BlockchainMock,
    contract_map: &ContractMap<TxContext>,
    tx_id: &String,
    tx: &Box<TxCall>,
    expect: &Option<TxExpect>,
) {
    let tx_input = TxInput {
        from: tx.from.value.into(),
        to: tx.to.value.into(),
        call_value: tx.call_value.value.clone(),
        esdt_value: if let Some(esdt) = &tx.esdt_value {
            // TODO: clean this up
            esdt.esdt_value.value.clone()
        } else {
            BigUint::zero()
        },
        esdt_token_identifier: if let Some(esdt) = &tx.esdt_value {
            // TODO: clean this up
            esdt.esdt_token_name.value.clone()
        } else {
            Vec::new()
        },
        func_name: tx.function.as_bytes().to_vec(),
        args: tx
            .arguments
            .iter()
            .map(|scen_arg| scen_arg.value.clone())
            .collect(),
        gas_limit: tx.gas_limit.value,
        gas_price: tx.gas_price.value,
        tx_hash: execute_helper_functions::generate_tx_hash_dummy(tx_id.as_str()),
    };
    state.increase_nonce(&tx_input.from);
    let tx_result = execute_sc_call_with_async_and_callback(tx_input, state, contract_map).unwrap();
    if let Some(tx_expect) = expect {
        check_tx_output(tx_id.as_str(), tx_expect, &tx_result);
    }
}
