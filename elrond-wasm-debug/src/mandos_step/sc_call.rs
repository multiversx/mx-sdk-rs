use std::rc::Rc;

use mandos::model::{TxCall, TxESDT, TxExpect};

use crate::{
    tx_execution::sc_call_with_async_and_callback,
    tx_mock::{generate_tx_hash_dummy, TxInput, TxInputESDT},
    world_mock::BlockchainMock,
};

use super::check_tx_output;

pub fn execute(
    state: &mut Rc<BlockchainMock>,
    tx_id: &str,
    tx: &TxCall,
    expect: &Option<TxExpect>,
) {
    let tx_input = TxInput {
        from: tx.from.value.into(),
        to: tx.to.value.into(),
        egld_value: tx.egld_value.value.clone(),
        esdt_values: tx_esdt_transfers_from_mandos(tx.esdt_value.as_slice()),
        func_name: tx.function.as_bytes().to_vec(),
        args: tx
            .arguments
            .iter()
            .map(|scen_arg| scen_arg.value.clone())
            .collect(),
        gas_limit: tx.gas_limit.value,
        gas_price: tx.gas_price.value,
        tx_hash: generate_tx_hash_dummy(tx_id),
    };
    let tx_result = sc_call_with_async_and_callback(tx_input, state, true);
    if let Some(tx_expect) = expect {
        check_tx_output(tx_id, tx_expect, &tx_result);
    }
}

pub fn tx_esdt_transfers_from_mandos(mandos_transf_esdt: &[TxESDT]) -> Vec<TxInputESDT> {
    mandos_transf_esdt
        .iter()
        .map(tx_esdt_transfer_from_mandos)
        .collect()
}

pub fn tx_esdt_transfer_from_mandos(mandos_transf_esdt: &TxESDT) -> TxInputESDT {
    TxInputESDT {
        token_identifier: mandos_transf_esdt.esdt_token_identifier.value.clone(),
        nonce: mandos_transf_esdt.nonce.value,
        value: mandos_transf_esdt.esdt_value.value.clone(),
    }
}
