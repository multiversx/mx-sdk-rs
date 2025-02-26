use super::{sc_call::tx_esdt_transfers_from_scenario, ScenarioVMRunner};
use crate::scenario::model::{TransferStep, TxTransfer, ValidatorRewardStep};
use multiversx_chain_vm::{
    tx_execution::{instance_call, BlockchainVMRef, RuntimeRef},
    tx_mock::{TxFunctionName, TxInput},
    world_mock::BlockchainStateRef,
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

    let tx_result = runtime.execute_sc_call_lambda(tx_input, state, instance_call);
    tx_result.assert_ok();
}
