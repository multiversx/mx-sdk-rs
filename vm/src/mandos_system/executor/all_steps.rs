use crate::world_mock::BlockchainMock;

use crate::mandos_system::model::*;
use std::path::Path;

pub fn parse_execute_mandos_steps(steps_path: &Path, state: &mut BlockchainMock) {
    let scenario = crate::mandos_system::parse_scenario(steps_path);

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
    fn mandos_set_state(&mut self, set_state_step: SetStateStep) -> &mut Self {
        self.perform_set_state(set_state_step);
        self
    }

    /// Adds a SC call step, as specified in the `sc_call_step` argument, then executes it.
    fn mandos_sc_call(&mut self, sc_call_step: ScCallStep) -> &mut Self {
        self.perform_sc_call(sc_call_step);
        self
    }

    /// Adds a SC query step, as specified in the `sc_query_step` argument, then executes it.
    fn mandos_sc_query(&mut self, sc_query_step: ScQueryStep) -> &mut Self {
        self.perform_sc_query(sc_query_step);
        self
    }

    /// Adds a SC deploy step, as specified in the `sc_deploy_step` argument, then executes it.
    fn mandos_sc_deploy(&mut self, sc_deploy_step: ScDeployStep) -> &mut Self {
        self.perform_sc_deploy(sc_deploy_step);
        self
    }

    fn mandos_transfer(&mut self, transfer_step: TransferStep) -> &mut Self {
        self.perform_transfer(transfer_step);
        self
    }

    fn mandos_validator_reward(
        &mut self,
        validator_rewards_step: ValidatorRewardStep,
    ) -> &mut Self {
        self
            .perform_validator_reward(validator_rewards_step);
        self
    }

    fn mandos_check_state(&mut self, check_state_step: CheckStateStep) -> &mut Self {
        self.perform_check_state(check_state_step);
        self
    }

    fn mandos_dump_state(&mut self) -> &mut Self {
        self.perform_dump_state();
        self
    }
}
