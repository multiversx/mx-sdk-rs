use crate::{
    scenario::model::{ScDeployStep, Step, TypedScDeploy, TypedScDeployExecutor},
    tx_mock::TxFunctionName,
};
use multiversx_sc::{
    codec::{CodecFrom, PanicErrorHandler, TopEncodeMulti},
    types::heap::Address,
};

use crate::{
    tx_execution::sc_create,
    tx_mock::{generate_tx_hash_dummy, TxInput, TxResult},
    world_mock::BlockchainMock,
};

use super::check_tx_output;

impl BlockchainMock {
    /// Adds a SC deploy step, as specified in the `sc_deploy_step` argument, then executes it.
    pub fn perform_sc_deploy(&mut self, sc_deploy_step: ScDeployStep) -> &mut Self {
        self.with_borrowed(|state| {
            let (_, _, state) = execute_and_check(state, &sc_deploy_step);
            ((), state)
        });
        self.scenario_trace
            .steps
            .push(Step::ScDeploy(sc_deploy_step));
        self
    }

    /// Adds a SC deploy step, executes it and retrieves the transaction result ("out" field).
    ///
    /// The transaction is expected to complete successfully.
    ///
    /// It takes the `contract_call` argument separately from the SC call step,
    /// so we can benefit from type inference in the result.
    pub fn perform_sc_deploy_get_result<OriginalResult, RequestedResult>(
        &mut self,
        typed_sc_deploy: TypedScDeploy<OriginalResult>,
    ) -> (Address, RequestedResult)
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>,
    {
        let sc_deploy_step: ScDeployStep = typed_sc_deploy.into();
        let (tx_result, new_address) = self.with_borrowed(|state| {
            let (tx_result, new_address, state) = execute(state, &sc_deploy_step);
            ((tx_result, new_address), state)
        });
        self.scenario_trace
            .steps
            .push(Step::ScDeploy(sc_deploy_step));

        let mut raw_result = tx_result.result_values;
        let deser_result =
            RequestedResult::multi_decode_or_handle_err(&mut raw_result, PanicErrorHandler)
                .unwrap();

        (new_address, deser_result)
    }
}

impl TypedScDeployExecutor for BlockchainMock {
    fn execute_typed_sc_deploy<OriginalResult, RequestedResult>(
        &mut self,
        typed_sc_call: TypedScDeploy<OriginalResult>,
    ) -> (Address, RequestedResult)
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>,
    {
        self.perform_sc_deploy_get_result(typed_sc_call)
    }
}

pub(crate) fn execute(
    state: BlockchainMock,
    sc_deploy_step: &ScDeployStep,
) -> (TxResult, Address, BlockchainMock) {
    let tx = &sc_deploy_step.tx;
    let tx_input = TxInput {
        from: tx.from.to_address(),
        to: Address::zero(),
        egld_value: tx.egld_value.value.clone(),
        esdt_values: Vec::new(),
        func_name: TxFunctionName::INIT,
        args: tx
            .arguments
            .iter()
            .map(|scen_arg| scen_arg.value.clone())
            .collect(),
        gas_limit: tx.gas_limit.value,
        gas_price: tx.gas_price.value,
        tx_hash: generate_tx_hash_dummy(&sc_deploy_step.id),
        ..Default::default()
    };
    sc_create(tx_input, &tx.contract_code.value, state)
}

fn execute_and_check(
    state: BlockchainMock,
    sc_deploy_step: &ScDeployStep,
) -> (TxResult, Address, BlockchainMock) {
    let (tx_result, address, state) = execute(state, sc_deploy_step);
    if let Some(tx_expect) = &sc_deploy_step.expect {
        check_tx_output(&sc_deploy_step.id, tx_expect, &tx_result);
    }
    (tx_result, address, state)
}
