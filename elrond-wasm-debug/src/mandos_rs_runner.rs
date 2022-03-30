#![allow(unused_variables)] // for now

use crate::{mandos_step, world_mock::BlockchainMock};

use mandos::model::Step;
use std::path::Path;

/// Runs mandos test using the Rust infrastructure and the debug mode.
/// Uses a contract map to replace the references to the wasm bytecode
/// with the contracts running in debug mode.
pub fn mandos_rs<P: AsRef<Path>>(relative_path: P, world: BlockchainMock) {
    let mut absolute_path = world.current_dir.clone();
    absolute_path.push(relative_path);
    let _ = parse_execute_mandos_steps(absolute_path.as_ref(), world);
}

fn parse_execute_mandos_steps(steps_path: &Path, mut state: BlockchainMock) -> BlockchainMock {
    let scenario = mandos::parse_scenario(steps_path);

    for step in scenario.steps.iter() {
        state = match step {
            Step::ExternalSteps(external_steps_step) => {
                let parent_path = steps_path.parent().unwrap();
                let new_path = parent_path.join(&external_steps_step.path);
                parse_execute_mandos_steps(new_path.as_path(), state)
            },
            Step::SetState(set_state_step) => {
                mandos_step::set_state::execute(&mut state, set_state_step);
                state
            },
            Step::ScCall(sc_call_step) => mandos_step::sc_call::execute(state, sc_call_step),
            Step::ScQuery(sc_query_step) => mandos_step::sc_query::execute(state, sc_query_step),
            Step::ScDeploy(sc_deploy_step) => {
                mandos_step::sc_deploy::execute(state, sc_deploy_step)
            },
            Step::Transfer(transfer_step) => {
                mandos_step::transfer::execute(state, &transfer_step.tx)
            },
            Step::ValidatorReward(validator_reward) => {
                state.increase_validator_reward(
                    &validator_reward.tx.to.value.into(),
                    &validator_reward.tx.egld_value.value,
                );
                state
            },
            Step::CheckState(check_state_step) => {
                mandos_step::check_state::execute(&check_state_step.accounts, &mut state);
                state
            },
            Step::DumpState(_) => {
                state.print_accounts();
                state
            },
        };
    }

    state
}
