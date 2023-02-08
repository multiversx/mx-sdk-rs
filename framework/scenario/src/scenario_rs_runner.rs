use crate::scenario::{self, handler::StepRunner, model::*};

use crate::ScenarioWorld;
use std::path::Path;

/// Runs scenario test using the Rust infrastructure and the debug mode.
/// Uses a contract map to replace the references to the wasm bytecode
/// with the contracts running in debug mode.
pub fn run_rs<P: AsRef<Path>>(relative_path: P, mut world: ScenarioWorld) {
    let mut absolute_path = world.current_dir.clone();
    absolute_path.push(relative_path);
    parse_execute_mandos_steps(absolute_path.as_ref(), &mut world);
}

fn parse_execute_mandos_steps(steps_path: &Path, state: &mut ScenarioWorld) {
    let scenario = scenario::parse_scenario(steps_path);

    for step in &scenario.steps {
        match step {
            Step::ExternalSteps(external_steps_step) => {
                let parent_path = steps_path.parent().unwrap();
                let new_path = parent_path.join(external_steps_step.path.as_str());
                parse_execute_mandos_steps(new_path.as_path(), state);
            },
            Step::SetState(set_state_step) => {
                state.run_set_state_step(set_state_step);
            },
            Step::ScCall(sc_call_step) => {
                state.run_sc_call_step(sc_call_step);
            },
            Step::ScQuery(sc_query_step) => {
                state.run_sc_query_step(sc_query_step);
            },
            Step::ScDeploy(sc_deploy_step) => {
                state.run_sc_deploy_step(sc_deploy_step);
            },
            Step::Transfer(transfer_step) => {
                state.run_transfer_step(transfer_step);
            },
            Step::ValidatorReward(validator_reward_step) => {
                state.run_validator_reward_step(validator_reward_step);
            },
            Step::CheckState(check_state_step) => {
                state.run_check_state_step(check_state_step);
            },
            Step::DumpState(_) => {
                state.run_dump_state_step();
            },
        }
    }
}
