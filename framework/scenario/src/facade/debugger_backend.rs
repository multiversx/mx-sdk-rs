use crate::scenario::{
    self, model::*, run_trace::ScenarioTrace, run_vm::ScenarioVMRunner, ScenarioRunner,
};
use std::path::Path;

/// Coordinates the execution of scenario tests
/// using the Rust implementation of the VM and direct contract execution.
pub(crate) struct DebuggerBackend {
    pub vm_runner: ScenarioVMRunner,
    pub trace: Option<ScenarioTrace>,
}

impl DebuggerBackend {
    pub fn for_each_runner_mut<F: FnMut(&mut dyn ScenarioRunner)>(&mut self, mut f: F) {
        f(&mut self.vm_runner);
        if let Some(trace) = &mut self.trace {
            f(trace);
        }
    }
}

impl ScenarioRunner for DebuggerBackend {
    fn run_external_steps(&mut self, step: &ExternalStepsStep) {
        self.for_each_runner_mut(|runner| runner.run_external_steps(step));
    }

    fn run_set_state_step(&mut self, step: &SetStateStep) {
        self.for_each_runner_mut(|runner| runner.run_set_state_step(step));
    }

    fn run_sc_call_step(&mut self, step: &mut ScCallStep) {
        self.vm_runner.run_sc_call_step(step);
        step.expect = step.response.as_ref().map(TxResponse::to_expect);
        if let Some(trace) = &mut self.trace {
            trace.run_sc_call_step(step);
        }
    }

    fn run_multi_sc_call_step(&mut self, steps: &mut [ScCallStep]) {
        self.vm_runner.run_multi_sc_call_step(steps);
        for step in steps.iter_mut() {
            step.expect = step.response.as_ref().map(TxResponse::to_expect);
        }
        if let Some(trace) = &mut self.trace {
            trace.run_multi_sc_call_step(steps);
        }
    }

    fn run_sc_query_step(&mut self, step: &mut ScQueryStep) {
        self.vm_runner.run_sc_query_step(step);
        step.expect = step.response.as_ref().map(TxResponse::to_expect);
        if let Some(trace) = &mut self.trace {
            trace.run_sc_query_step(step);
        }
    }

    fn run_sc_deploy_step(&mut self, step: &mut ScDeployStep) {
        self.vm_runner.run_sc_deploy_step(step);
        step.expect = step.response.as_ref().map(TxResponse::to_expect);
        if let Some(trace) = &mut self.trace {
            trace.run_sc_deploy_step(step);
        }
    }

    fn run_multi_sc_deploy_step(&mut self, steps: &mut [ScDeployStep]) {
        self.vm_runner.run_multi_sc_deploy_step(steps);
        for step in steps.iter_mut() {
            step.expect = step.response.as_ref().map(TxResponse::to_expect);
        }
        if let Some(trace) = &mut self.trace {
            trace.run_multi_sc_deploy_step(steps);
        }
    }

    fn run_transfer_step(&mut self, step: &TransferStep) {
        self.for_each_runner_mut(|runner| runner.run_transfer_step(step));
    }

    fn run_validator_reward_step(&mut self, step: &ValidatorRewardStep) {
        self.for_each_runner_mut(|runner| runner.run_validator_reward_step(step));
    }

    fn run_check_state_step(&mut self, step: &CheckStateStep) {
        self.for_each_runner_mut(|runner| runner.run_check_state_step(step));
    }

    fn run_dump_state_step(&mut self) {
        self.for_each_runner_mut(|runner| runner.run_dump_state_step());
    }
}

impl DebuggerBackend {
    pub(super) fn run_scenario_file(&mut self, steps_path: &Path) {
        let mut scenario = scenario::parse_scenario(steps_path);

        for step in &mut scenario.steps {
            match step {
                Step::ExternalSteps(external_steps_step) => {
                    let parent_path = steps_path.parent().unwrap();
                    let new_path = parent_path.join(external_steps_step.path.as_str());
                    self.run_scenario_file(new_path.as_path());
                },
                Step::SetState(set_state_step) => {
                    self.run_set_state_step(set_state_step);
                },
                Step::ScCall(sc_call_step) => {
                    self.run_sc_call_step(sc_call_step);
                },
                Step::ScQuery(sc_query_step) => {
                    self.run_sc_query_step(sc_query_step);
                },
                Step::ScDeploy(sc_deploy_step) => {
                    self.run_sc_deploy_step(sc_deploy_step);
                },
                Step::Transfer(transfer_step) => {
                    self.run_transfer_step(transfer_step);
                },
                Step::ValidatorReward(validator_reward_step) => {
                    self.run_validator_reward_step(validator_reward_step);
                },
                Step::CheckState(check_state_step) => {
                    self.run_check_state_step(check_state_step);
                },
                Step::DumpState(_) => {
                    self.run_dump_state_step();
                },
            }
        }
    }
}
