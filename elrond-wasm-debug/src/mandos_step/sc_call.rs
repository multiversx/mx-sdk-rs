use mandos::model::{ScCallStep, Step, TxESDT};

use crate::{
    tx_execution::sc_call_with_async_and_callback,
    tx_mock::{generate_tx_hash_dummy, TxInput, TxInputESDT, TxResult},
    world_mock::BlockchainMock,
};

use super::check_tx_output;

impl BlockchainMock {
    pub fn mandos_sc_call(&mut self, sc_call_step: ScCallStep) -> &mut Self {
        self.with_borrowed(|state| ((), execute_and_check(state, &sc_call_step)));
        self.mandos_trace.steps.push(Step::ScCall(sc_call_step));
        self
    }
}

pub(crate) fn execute(
    mut state: BlockchainMock,
    sc_call_step: &ScCallStep,
) -> (TxResult, BlockchainMock) {
    let tx = &sc_call_step.tx;
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
        tx_hash: generate_tx_hash_dummy(&sc_call_step.tx_id),
    };

    // nonce gets increased irrespective of whether the tx fails or not
    state.increase_account_nonce(&tx_input.from);

    sc_call_with_async_and_callback(tx_input, state)
}

fn execute_and_check(state: BlockchainMock, sc_call_step: &ScCallStep) -> BlockchainMock {
    let (tx_result, state) = execute(state, sc_call_step);
    if let Some(tx_expect) = &sc_call_step.expect {
        check_tx_output(&sc_call_step.tx_id, tx_expect, &tx_result);
    }
    state
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
