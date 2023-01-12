use crate::world_mock::BlockchainMock;

use crate::scenario::model::*;
use std::path::Path;

pub fn parse_execute_mandos_steps(steps_path: &Path, state: &mut BlockchainMock) {
    let scenario = crate::scenario::parse_scenario(steps_path);

    for step in scenario.steps.into_iter() {
        match step {
            Step::ExternalSteps(external_steps_step) => {
                let parent_path = steps_path.parent().unwrap();
                let new_path = parent_path.join(external_steps_step.path);
                parse_execute_mandos_steps(new_path.as_path(), state);
            },
            Step::SetState(set_state_step) => {
                state.perform_set_state(set_state_step);
            },
            Step::ScCall(sc_call_step) => {
                state.perform_sc_call(sc_call_step);
            },
            Step::ScQuery(sc_query_step) => {
                state.perform_sc_query(sc_query_step);
            },
            Step::ScDeploy(sc_deploy_step) => {
                state.perform_sc_deploy(sc_deploy_step);
            },
            Step::Transfer(transfer_step) => {
                state.perform_transfer(transfer_step);
            },
            Step::ValidatorReward(validator_reward_step) => {
                state.perform_validator_reward(validator_reward_step);
            },
            Step::CheckState(check_state_step) => {
                state.perform_check_state(check_state_step);
            },
            Step::DumpState(_) => {
                state.perform_dump_state();
            },
        }
    }
}

impl StepHandler for BlockchainMock {
    fn set_state_step(&mut self, step: SetStateStep) -> &mut Self {
        self.perform_set_state(step);
        self
    }

    /// Adds a SC call step, as specified in the `sc_call_step` argument, then executes it.
    fn sc_call_step(&mut self, step: ScCallStep) -> &mut Self {
        self.perform_sc_call(step);
        self
    }

    /// Adds a SC query step, as specified in the `sc_query_step` argument, then executes it.
    fn sc_query_step(&mut self, step: ScQueryStep) -> &mut Self {
        self.perform_sc_query(step);
        self
    }

    /// Adds a SC deploy step, as specified in the `sc_deploy_step` argument, then executes it.
    fn sc_deploy_step(&mut self, step: ScDeployStep) -> &mut Self {
        self.perform_sc_deploy(step);
        self
    }

    fn transfer_step(&mut self, step: TransferStep) -> &mut Self {
        self.perform_transfer(step);
        self
    }

    fn validator_reward_step(&mut self, step: ValidatorRewardStep) -> &mut Self {
        self.perform_validator_reward(step);
        self
    }

    fn check_state_step(&mut self, step: CheckStateStep) -> &mut Self {
        self.perform_check_state(step);
        self
    }

    fn dump_state_step(&mut self) -> &mut Self {
        self.perform_dump_state();
        self
    }
}
