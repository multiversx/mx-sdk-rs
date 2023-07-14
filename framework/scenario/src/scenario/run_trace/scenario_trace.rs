use multiversx_chain_scenario_format::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    serde_raw::ScenarioRaw,
};

use crate::{
    multiversx_sc::types::Address,
    scenario::{model::*, ScenarioRunner},
    scenario_format::interpret_trait::IntoRaw,
};
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

    pub fn load_scenario_trace<P: AsRef<Path>>(&mut self, file_path: P) {
        let mandos_trace_raw = ScenarioRaw::load_from_file(file_path);
        let mandos_trace =
            Scenario::interpret_from(mandos_trace_raw, &InterpreterContext::default());
        self.scenario_trace = mandos_trace;
    }

    fn process_address_key(&mut self, address_key: &AddressKey) {
        if !self
            .addr_to_pretty_string_map
            .contains_key(&address_key.value)
        {
            self.addr_to_pretty_string_map
                .insert(address_key.value.clone(), address_key.original.clone());
        }
    }

    fn process_address_value(&mut self, address_value: &AddressValue) {
        if !self
            .addr_to_pretty_string_map
            .contains_key(&address_value.value)
        {
            self.addr_to_pretty_string_map.insert(
                address_value.value.clone(),
                address_value.original.to_concatenated_string(),
            );
        }
    }
}

impl ScenarioRunner for ScenarioTrace {
    fn run_external_steps(&mut self, step: &ExternalStepsStep) {
        self.scenario_trace
            .steps
            .push(Step::ExternalSteps(step.clone()));
    }

    fn run_set_state_step(&mut self, step: &SetStateStep) {
        for address_key in step.accounts.keys() {
            self.process_address_key(address_key);
        }
        for new_address in &step.new_addresses {
            self.process_address_value(&new_address.new_address);
        }
        self.scenario_trace.steps.push(Step::SetState(step.clone()));
    }

    fn run_sc_call_step(&mut self, step: &mut ScCallStep) {
        self.process_address_value(&step.tx.from);
        self.process_address_value(&step.tx.to);
        self.scenario_trace.steps.push(Step::ScCall(step.clone()));
    }

    fn run_multi_sc_call_step(&mut self, steps: &mut [ScCallStep]) {
        for step in steps {
            self.process_address_value(&step.tx.from);
            self.process_address_value(&step.tx.to);
            self.scenario_trace.steps.push(Step::ScCall(step.clone()));
        }
    }

    fn run_multi_sc_deploy_step(&mut self, steps: &mut [ScDeployStep]) {
        for step in steps {
            self.process_address_value(&step.tx.from);
            self.scenario_trace.steps.push(Step::ScDeploy(step.clone()));
        }
    }

    fn run_sc_query_step(&mut self, step: &mut ScQueryStep) {
        self.process_address_value(&step.tx.to);
        self.scenario_trace.steps.push(Step::ScQuery(step.clone()));
    }

    fn run_sc_deploy_step(&mut self, step: &mut ScDeployStep) {
        self.process_address_value(&step.tx.from);
        self.scenario_trace.steps.push(Step::ScDeploy(step.clone()));
    }

    fn run_transfer_step(&mut self, step: &TransferStep) {
        self.process_address_value(&step.tx.from);
        self.process_address_value(&step.tx.to);
        self.scenario_trace.steps.push(Step::Transfer(step.clone()));
    }

    fn run_validator_reward_step(&mut self, step: &ValidatorRewardStep) {
        self.scenario_trace
            .steps
            .push(Step::ValidatorReward(step.clone()));
    }

    fn run_check_state_step(&mut self, step: &CheckStateStep) {
        for address_key in step.accounts.accounts.keys() {
            self.process_address_key(address_key);
        }
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
