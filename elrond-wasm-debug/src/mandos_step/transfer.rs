use elrond_wasm::types::heap::H256;
use mandos::model::{Step, TransferStep, TxTransfer, ValidatorRewardStep};

use crate::{
    sc_call::tx_esdt_transfers_from_mandos, tx_execution::execute_sc_call, tx_mock::TxInput,
    world_mock::BlockchainMock,
};

impl BlockchainMock {
    pub fn mandos_transfer(&mut self, transfer_step: TransferStep) -> &mut Self {
        self.with_borrowed(|state| ((), execute(state, &transfer_step.tx)));
        self.mandos_trace.steps.push(Step::Transfer(transfer_step));
        self
    }

    pub fn mandos_validator_reward(
        &mut self,
        validator_rewards_step: ValidatorRewardStep,
    ) -> &mut Self {
        self.increase_validator_reward(
            &validator_rewards_step.tx.to.value.into(),
            &validator_rewards_step.tx.egld_value.value,
        );
        self.mandos_trace
            .steps
            .push(Step::ValidatorReward(validator_rewards_step));
        self
    }
}

fn execute(mut state: BlockchainMock, tx_transfer: &TxTransfer) -> BlockchainMock {
    let tx_input = TxInput {
        from: tx_transfer.from.value.into(),
        to: tx_transfer.to.value.into(),
        egld_value: tx_transfer.egld_value.value.clone(),
        esdt_values: tx_esdt_transfers_from_mandos(tx_transfer.esdt_value.as_slice()),
        func_name: Vec::new(),
        args: Vec::new(),
        gas_limit: tx_transfer.gas_limit.value,
        gas_price: tx_transfer.gas_price.value,
        tx_hash: H256::zero(),
    };

    // nonce gets increased irrespective of whether the tx fails or not
    state.increase_account_nonce(&tx_input.from);

    let (tx_result, state) = execute_sc_call(tx_input, state);
    tx_result.assert_ok();
    state
}
