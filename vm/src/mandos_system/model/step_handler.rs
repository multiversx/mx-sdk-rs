use super::*;

/// Allows caller to construct a scenario and do something with it on the fly.
/// 
/// Abstracts away implementation, can be
/// - a simulation using any executor,
/// - calls to a blockchain,
/// - collecting/exporting the scenario,
/// - something else.
pub trait StepHandler: TypedScCallExecutor + TypedScDeployExecutor + TypedScQueryExecutor {
    fn mandos_set_state(&mut self, set_state_step: SetStateStep) -> &mut Self;

    /// Adds a SC call step, as specified in the `sc_call_step` argument, then executes it.
    fn mandos_sc_call(&mut self, sc_call_step: ScCallStep) -> &mut Self;

    /// Adds a SC query step, as specified in the `sc_query_step` argument, then executes it.
    fn mandos_sc_query(&mut self, sc_query_step: ScQueryStep) -> &mut Self;

    /// Adds a SC deploy step, as specified in the `sc_deploy_step` argument, then executes it.
    fn mandos_sc_deploy(&mut self, sc_deploy_step: ScDeployStep) -> &mut Self;

    fn mandos_transfer(&mut self, transfer_step: TransferStep) -> &mut Self;

    fn mandos_validator_reward(&mut self, validator_rewards_step: ValidatorRewardStep)
        -> &mut Self;

    fn mandos_check_state(&mut self, check_state_step: CheckStateStep) -> &mut Self;

    fn mandos_dump_state(&mut self) -> &mut Self;
}
