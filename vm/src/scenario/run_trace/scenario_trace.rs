use crate::{
    scenario::{handler::StepRunner, model::*},
    scenario_format::interpret_trait::IntoRaw,
};
use multiversx_sc::types::Address;
use std::{collections::HashMap, path::Path};

#[derive(Default, Debug)]
pub struct ScenarioTrace {
    pub scenario_trace: Scenario,
    pub addr_to_pretty_string_map: HashMap<Address, String>,
}

impl ScenarioTrace {
    pub fn write_scenario_trace<P: AsRef<Path>>(&mut self, file_path: P) {
        self.scenario_trace_prettify();

        let mandos_trace = core::mem::take(&mut self.scenario_trace);
        let mandos_trace_raw = mandos_trace.into_raw();
        mandos_trace_raw.save_to_file(file_path);
    }
}

impl StepRunner for ScenarioTrace {
    fn run_external_steps(&mut self, step: &ExternalStepsStep) {
        self.scenario_trace
            .steps
            .push(Step::ExternalSteps(step.clone()));
    }

    fn run_set_state_step(&mut self, step: &SetStateStep) {
        self.scenario_trace.steps.push(Step::SetState(step.clone()));
    }

    fn run_sc_call_step(&mut self, step: &ScCallStep) {
        self.scenario_trace.steps.push(Step::ScCall(step.clone()));
    }

    fn run_sc_query_step(&mut self, step: &ScQueryStep) {
        self.scenario_trace.steps.push(Step::ScQuery(step.clone()));
    }

    fn run_sc_deploy_step(&mut self, step: &ScDeployStep) {
        self.scenario_trace.steps.push(Step::ScDeploy(step.clone()));
    }

    fn run_transfer_step(&mut self, step: &TransferStep) {
        self.scenario_trace.steps.push(Step::Transfer(step.clone()));
    }

    fn run_validator_reward_step(&mut self, step: &ValidatorRewardStep) {
        self.scenario_trace
            .steps
            .push(Step::ValidatorReward(step.clone()));
    }

    fn run_check_state_step(&mut self, step: &CheckStateStep) {
        self.scenario_trace
            .steps
            .push(Step::CheckState(step.clone()));
    }

    fn run_dump_state_step(&mut self) {
        self.scenario_trace
            .steps
            .push(Step::DumpState(DumpStateStep::default()));
    }
}
