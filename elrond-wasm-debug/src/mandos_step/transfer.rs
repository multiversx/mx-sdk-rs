use std::rc::Rc;

use elrond_wasm::types::heap::H256;
use mandos::model::{TransferStep, TxTransfer, ValidatorRewardStep};

use crate::{
    sc_call::tx_esdt_transfers_from_mandos, tx_execution::sc_call, tx_mock::TxInput,
    world_mock::BlockchainMock,
};

impl BlockchainMock {
    pub fn mandos_transfer(self, transfer_step: TransferStep) -> BlockchainMock {
        let mut state_rc = Rc::new(self);
        execute_rc(&mut state_rc, &transfer_step.tx);
        Rc::try_unwrap(state_rc).unwrap()
    }

    pub fn mandos_validator_reward(
        mut self,
        validator_rewards_step: ValidatorRewardStep,
    ) -> BlockchainMock {
        self.increase_validator_reward(
            &validator_rewards_step.tx.to.value.into(),
            &validator_rewards_step.tx.egld_value.value,
        );
        self
    }
}

pub fn execute_rc(state: &mut Rc<BlockchainMock>, tx_transfer: &TxTransfer) {
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
    sc_call(tx_input, state, true).assert_ok();
}
