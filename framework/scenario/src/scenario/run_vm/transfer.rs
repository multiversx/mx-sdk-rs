use super::{sc_call::tx_esdt_transfers_from_scenario, ScenarioVMRunner};
use crate::scenario::model::{TransferStep, TxTransfer, ValidatorRewardStep};
use multiversx_chain_vm::{
    tx_execution::BlockchainVMRef,
    tx_mock::{TxFunctionName, TxInput},
    world_mock::BlockchainState,
};

impl ScenarioVMRunner {
    pub fn perform_transfer(&mut self, transfer_step: &TransferStep) {
        self.blockchain_mock
            .with_borrowed(|vm, state| ((), execute(vm, state, &transfer_step.tx)));
    }

    pub fn perform_validator_reward(&mut self, validator_rewards_step: &ValidatorRewardStep) {
        self.blockchain_mock.state.increase_validator_reward(
            &validator_rewards_step.tx.to.to_vm_address(),
            &validator_rewards_step.tx.egld_value.value,
        );
    }
}

fn execute(
    vm: BlockchainVMRef,
    mut state: BlockchainState,
    tx_transfer: &TxTransfer,
) -> BlockchainState {
    let tx_input = TxInput {
        from: tx_transfer.from.to_vm_address(),
        to: tx_transfer.to.to_vm_address(),
        egld_value: tx_transfer.egld_value.value.clone(),
        esdt_values: tx_esdt_transfers_from_scenario(tx_transfer.esdt_value.as_slice()),
        func_name: TxFunctionName::EMPTY,
        args: Vec::new(),
        gas_limit: tx_transfer.gas_limit.value,
        gas_price: tx_transfer.gas_price.value,
        ..Default::default()
    };

    // nonce gets increased irrespective of whether the tx fails or not
    state.increase_account_nonce(&tx_input.from);

    let (tx_result, state) = vm.execute_sc_call(tx_input, state);
    tx_result.assert_ok();
    state
}
