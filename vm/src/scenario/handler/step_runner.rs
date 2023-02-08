use super::super::model::*;

/// Allows caller to process a single scenario step, not matter what this means concretely.
///
/// Abstracts away implementation, can be
/// - a simulation using any executor,
/// - calls to a blockchain,
/// - collecting/exporting the scenario,
/// - something else.
pub trait StepRunner {
    fn run_external_steps(&mut self, step: &ExternalStepsStep);

    fn run_set_state_step(&mut self, step: &SetStateStep);

    fn run_sc_call_step(&mut self, step: &ScCallStep);

    fn run_sc_query_step(&mut self, step: &ScQueryStep);

    fn run_sc_deploy_step(&mut self, step: &ScDeployStep);

    fn run_transfer_step(&mut self, step: &TransferStep);

    fn run_validator_reward_step(&mut self, step: &ValidatorRewardStep);

    fn run_check_state_step(&mut self, step: &CheckStateStep);

    fn run_dump_state_step(&mut self);
}
