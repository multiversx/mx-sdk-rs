use elrond_wasm::{
    elrond_codec::{CodecFrom, PanicErrorHandler, TopEncodeMulti},
    types::{heap::Address, ContractDeploy},
};
use mandos::model::{ScDeployStep, Step};

use crate::{
    tx_execution::sc_create,
    tx_mock::{generate_tx_hash_dummy, TxInput, TxResult},
    world_mock::BlockchainMock,
    CallBuilder, DebugApi,
};

use super::check_tx_output;

impl BlockchainMock {
    /// Adds a mandos SC deploy step, as specified in the `sc_deploy_step` argument, then executes it.
    pub fn mandos_sc_deploy(&mut self, sc_deploy_step: ScDeployStep) -> &mut Self {
        self.with_borrowed(|state| {
            let (_, _, state) = execute_and_check(state, &sc_deploy_step);
            ((), state)
        });
        self.mandos_trace.steps.push(Step::ScDeploy(sc_deploy_step));
        self
    }

    /// Adds a mandos SC deploy step, executes it and retrieves the transaction result ("out" field).
    ///
    /// The transaction is expected to complete successfully.
    ///
    /// It takes the `contract_call` argument separately from the SC call step,
    /// so we can benefit from type inference in the result.
    pub fn mandos_sc_deploy_get_result<OriginalResult, RequestedResult>(
        &mut self,
        contract_deploy: ContractDeploy<DebugApi, OriginalResult>,
        mut sc_deploy_step: ScDeployStep,
    ) -> (Address, RequestedResult)
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>,
    {
        sc_deploy_step = sc_deploy_step.call(contract_deploy);
        let (tx_result, new_address) = self.with_borrowed(|state| {
            let (tx_result, new_address, state) = execute(state, &sc_deploy_step);
            ((tx_result, new_address), state)
        });
        self.mandos_trace.steps.push(Step::ScDeploy(sc_deploy_step));

        let mut raw_result = tx_result.result_values;
        let deser_result =
            RequestedResult::multi_decode_or_handle_err(&mut raw_result, PanicErrorHandler)
                .unwrap();

        (new_address, deser_result)
    }
}

pub(crate) fn execute(
    state: BlockchainMock,
    sc_deploy_step: &ScDeployStep,
) -> (TxResult, Address, BlockchainMock) {
    let tx = &sc_deploy_step.tx;
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
        tx_hash: generate_tx_hash_dummy(&sc_deploy_step.tx_id),
    };
    sc_create(tx_input, &tx.contract_code.value, state)
}

fn execute_and_check(
    state: BlockchainMock,
    sc_deploy_step: &ScDeployStep,
) -> (TxResult, Address, BlockchainMock) {
    let (tx_result, address, state) = execute(state, sc_deploy_step);
    if let Some(tx_expect) = &sc_deploy_step.expect {
        check_tx_output(&sc_deploy_step.tx_id, tx_expect, &tx_result);
    }
    (tx_result, address, state)
}
