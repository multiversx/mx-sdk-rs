use std::rc::Rc;

use elrond_wasm::{
    elrond_codec::{num_bigint::BigUint, CodecFrom, PanicErrorHandler, TopEncodeMulti},
    types::{Address, ContractCall, H256},
};
use mandos::model::{ScCallStep, Step, TxESDT};

use crate::{
    tx_execution::sc_call_with_async_and_callback,
    tx_mock::{generate_tx_hash_dummy, TxInput, TxInputESDT},
    world_mock::BlockchainMock,
    DebugApi,
};

use super::check_tx_output;

impl BlockchainMock {
    pub fn mandos_sc_call(&mut self, sc_call_step: ScCallStep) -> &mut Self {
        self.with_borrowed_rc(|rc| {
            execute_rc(rc, &sc_call_step);
        });
        self.mandos_trace.steps.push(Step::ScCall(sc_call_step));
        self
    }

    /// TODO: REFACTOR!
    pub fn quick_call<OriginalResult, RequestedResult>(
        &mut self,
        from: Address,
        contract_call: ContractCall<DebugApi, OriginalResult>,
    ) -> RequestedResult
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>,
    {
        let tx_input = TxInput {
            from,
            to: contract_call.to.to_address(),
            egld_value: BigUint::from(0u32),
            esdt_values: Vec::new(),
            func_name: contract_call.endpoint_name.to_boxed_bytes().into_vec(),
            args: contract_call.arg_buffer.to_raw_args_vec(),
            gas_limit: contract_call.resolve_gas_limit(),
            gas_price: 0u64,
            tx_hash: H256::zero(),
        };

        let tx_result =
            self.with_borrowed_rc(|rc| sc_call_with_async_and_callback(tx_input, rc, true));
        let mut raw_result = tx_result.result_values;

        RequestedResult::multi_decode_or_handle_err(&mut raw_result, PanicErrorHandler).unwrap()
    }
}

fn execute_rc(state: &mut Rc<BlockchainMock>, sc_call_step: &ScCallStep) {
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
    let tx_result = sc_call_with_async_and_callback(tx_input, state, true);
    if let Some(tx_expect) = &sc_call_step.expect {
        check_tx_output(&sc_call_step.tx_id, tx_expect, &tx_result);
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
