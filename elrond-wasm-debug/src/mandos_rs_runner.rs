#![allow(unused_variables)] // for now

use crate::world_mock::BlockchainMock;

use mandos::model::Step;
use std::path::Path;

/// Runs mandos test using the Rust infrastructure and the debug mode.
/// Uses a contract map to replace the references to the wasm bytecode
/// with the contracts running in debug mode.
pub fn mandos_rs<P: AsRef<Path>>(relative_path: P, mut world: BlockchainMock) {
    let mut absolute_path = world.current_dir.clone();
    absolute_path.push(relative_path);
    parse_execute_mandos_steps(absolute_path.as_ref(), &mut world);
}

fn parse_execute_mandos_steps(steps_path: &Path, state: &mut BlockchainMock) {
    let scenario = mandos::parse_scenario(steps_path);

    for step in scenario.steps.into_iter() {
        match step {
            Step::ExternalSteps(external_steps_step) => {
                let parent_path = steps_path.parent().unwrap();
                let new_path = parent_path.join(&external_steps_step.path);
                parse_execute_mandos_steps(new_path.as_path(), state);
            },
            Step::SetState(set_state_step) => {
                state.mandos_set_state(set_state_step);
            },
            Step::ScCall(sc_call_step) => {
                state.mandos_sc_call(sc_call_step);
            },
            Step::ScQuery(sc_query_step) => {
                state.mandos_sc_query(sc_query_step);
            },
            Step::ScDeploy(sc_deploy_step) => {
                state.mandos_sc_deploy(sc_deploy_step);
            },
            Step::Transfer(transfer_step) => {
                state.mandos_transfer(transfer_step);
            },
            Step::ValidatorReward(validator_reward_step) => {
                state.mandos_validator_reward(validator_reward_step);
            },
            Step::CheckState(check_state_step) => {
                state.mandos_check_state(check_state_step);
            },
            Step::DumpState(_) => {
                state.mandos_dump_state();
            },
        }
    }
}
