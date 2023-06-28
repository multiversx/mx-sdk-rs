use crate::{
    debug_executor::ContractMapRef,
    multiversx_chain_vm::BlockchainMock,
    scenario::{model::*, ScenarioRunner},
};

/// Wraps calls to the blockchain mock,
/// while implementing the StepRunner interface.
#[derive(Default, Debug)]
pub struct ScenarioVMRunner {
    pub contract_map_ref: ContractMapRef,
    pub blockchain_mock: BlockchainMock,
}

impl ScenarioVMRunner {
    pub fn new() -> Self {
        let contract_map_ref = ContractMapRef::new();
        let blockchain_mock = BlockchainMock::new(Box::new(contract_map_ref.clone()));
        ScenarioVMRunner {
            contract_map_ref,
            blockchain_mock,
        }
    }
}

impl ScenarioRunner for ScenarioVMRunner {
    fn run_external_steps(&mut self, _step: &ExternalStepsStep) {
        panic!("cannot call directly as such")
    }

    fn run_set_state_step(&mut self, step: &SetStateStep) {
        self.perform_set_state(step);
    }

    fn run_sc_call_step(&mut self, step: &ScCallStep) {
        self.perform_sc_call(step);
    }

    fn run_multi_sc_call_step(&mut self, steps: &[ScCallStep]) {
        for step in steps {
            self.perform_sc_call(step);
        }
    }

    fn run_multi_sc_deploy_step(&mut self, steps: &[ScDeployStep]) {
        for step in steps {
            self.perform_sc_deploy(step);
        }
    }

    fn run_sc_query_step(&mut self, step: &ScQueryStep) {
        let _ = self.perform_sc_query(step);
    }

    fn run_sc_deploy_step(&mut self, step: &ScDeployStep) {
        self.perform_sc_deploy(step);
    }

    fn run_transfer_step(&mut self, step: &TransferStep) {
        self.perform_transfer(step);
    }

    fn run_validator_reward_step(&mut self, step: &ValidatorRewardStep) {
        self.perform_validator_reward(step);
    }

    fn run_check_state_step(&mut self, step: &CheckStateStep) {
        self.perform_check_state(step);
    }

    fn run_dump_state_step(&mut self) {
        self.perform_dump_state();
    }
}
