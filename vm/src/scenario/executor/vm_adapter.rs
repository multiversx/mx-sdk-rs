use crate::{
    scenario::{handler::StepRunner, model::*},
    BlockchainMock,
};

/// Wraps calls to the blockchain mock,
/// while implementing the StepRunner interface.
#[derive(Default, Debug)]
pub struct VmAdapter {
    pub blockchain_mock: BlockchainMock,
}

impl VmAdapter {
    pub fn new() -> Self {
        VmAdapter {
            blockchain_mock: BlockchainMock::new(),
        }
    }
}

impl StepRunner for VmAdapter {
    fn run_external_steps(&mut self, _step: &ExternalStepsStep) {
        panic!("cannot call directly as such")
    }

    fn run_set_state_step(&mut self, step: &SetStateStep) {
        self.blockchain_mock.perform_set_state(step);
    }

    fn run_sc_call_step(&mut self, step: &ScCallStep) {
        self.blockchain_mock.perform_sc_call(step);
    }

    fn run_sc_query_step(&mut self, step: &ScQueryStep) {
        let _ = self.blockchain_mock.perform_sc_query(step);
    }

    fn run_sc_deploy_step(&mut self, step: &ScDeployStep) {
        self.blockchain_mock.perform_sc_deploy(step);
    }

    fn run_transfer_step(&mut self, step: &TransferStep) {
        self.blockchain_mock.perform_transfer(step);
    }

    fn run_validator_reward_step(&mut self, step: &ValidatorRewardStep) {
        self.blockchain_mock.perform_validator_reward(step);
    }

    fn run_check_state_step(&mut self, step: &CheckStateStep) {
        self.blockchain_mock.perform_check_state(step);
    }

    fn run_dump_state_step(&mut self) {
        self.blockchain_mock.perform_dump_state();
    }
}
