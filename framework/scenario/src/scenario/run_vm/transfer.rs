use super::{ScenarioVMRunner, sc_call::tx_esdt_transfers_from_scenario};
use crate::scenario::model::{TransferStep, TxTransfer, ValidatorRewardStep};
use multiversx_chain_vm::{
    blockchain::state::BlockchainStateRef,
    host::{
        context::{TxFunctionName, TxInput},
        execution,
        runtime::{RuntimeInstanceCallLambdaDefault, RuntimeRef},
    },
};

impl ScenarioVMRunner {
    pub fn perform_transfer(&mut self, transfer_step: &TransferStep) {
        let runtime = self.create_debugger_runtime();
        execute(&runtime, &mut self.blockchain_mock.state, &transfer_step.tx);
    }

    pub fn perform_validator_reward(&mut self, validator_rewards_step: &ValidatorRewardStep) {
        self.blockchain_mock.state.increase_validator_reward(
            &validator_rewards_step.tx.to.to_address(),
            &validator_rewards_step.tx.egld_value.value,
        );
    }
}

fn tx_input_from_transfer(tx_transfer: &TxTransfer) -> TxInput {
    TxInput {
        from: tx_transfer.from.to_address(),
        to: tx_transfer.to.to_address(),
        egld_value: tx_transfer.egld_value.value.clone(),
        esdt_values: tx_esdt_transfers_from_scenario(tx_transfer.esdt_value.as_slice()),
        func_name: TxFunctionName::EMPTY,
        args: Vec::new(),
        gas_limit: tx_transfer.gas_limit.value,
        gas_price: tx_transfer.gas_price.value,
        ..Default::default()
    }
}

fn execute(runtime: &RuntimeRef, state: &mut BlockchainStateRef, tx_transfer: &TxTransfer) {
    let tx_input = tx_input_from_transfer(tx_transfer);

    // nonce gets increased irrespective of whether the tx fails or not
    state.increase_account_nonce(&tx_input.from);

    let tx_result =
        execution::commit_call(tx_input, state, runtime, RuntimeInstanceCallLambdaDefault);
    tx_result.assert_ok();
}
