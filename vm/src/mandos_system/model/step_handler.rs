use super::*;

/// Allows caller to construct a scenario and do something with it on the fly.
///
/// Abstracts away implementation, can be
/// - a simulation using any executor,
/// - calls to a blockchain,
/// - collecting/exporting the scenario,
/// - something else.
pub trait StepHandler: TypedScCallExecutor + TypedScDeployExecutor + TypedScQueryExecutor {
    /// Adds a SC call step, then executes it.
    fn set_state_step(&mut self, step: SetStateStep) -> &mut Self;

    /// Adds a SC call step, then executes it.
    fn sc_call_step(&mut self, step: ScCallStep) -> &mut Self;

    /// Adds a SC query step, then executes it.
    fn sc_query_step(&mut self, step: ScQueryStep) -> &mut Self;

    /// Adds a SC deploy step, then executes it.
    fn sc_deploy_step(&mut self, step: ScDeployStep) -> &mut Self;

    /// Adds a simple transfer step, then executes it.
    fn transfer_step(&mut self, step: TransferStep) -> &mut Self;

    /// Adds a validator reward step, then executes it.
    fn validator_reward_step(&mut self, step: ValidatorRewardStep) -> &mut Self;

    /// Adds a check state step, then executes it.
    fn check_state_step(&mut self, step: CheckStateStep) -> &mut Self;

    /// Adds a dump state step, then executes it.
    fn dump_state_step(&mut self) -> &mut Self;

    #[deprecated(since = "0.39.0", note = "Renamed, use `set_state_step` instead.")]
    fn mandos_set_state(&mut self, step: SetStateStep) -> &mut Self {
        self.set_state_step(step)
    }

    #[deprecated(since = "0.39.0", note = "Renamed, use `sc_call_step` instead.")]
    fn mandos_sc_call(&mut self, step: ScCallStep) -> &mut Self {
        self.sc_call_step(step)
    }

    #[deprecated(since = "0.39.0", note = "Renamed, use `sc_query_step` instead.")]
    fn mandos_sc_query(&mut self, step: ScQueryStep) -> &mut Self {
        self.sc_query_step(step)
    }

    #[deprecated(since = "0.39.0", note = "Renamed, use `sc_deploy_step` instead.")]
    fn mandos_sc_deploy(&mut self, step: ScDeployStep) -> &mut Self {
        self.sc_deploy_step(step)
    }

    #[deprecated(since = "0.39.0", note = "Renamed, use `transfer_step` instead.")]
    fn mandos_transfer(&mut self, step: TransferStep) -> &mut Self {
        self.transfer_step(step)
    }

    #[deprecated(
        since = "0.39.0",
        note = "Renamed, use `validator_reward_step` instead."
    )]
    fn mandos_validator_reward(&mut self, step: ValidatorRewardStep) -> &mut Self {
        self.validator_reward_step(step)
    }

    #[deprecated(since = "0.39.0", note = "Renamed, use `check_state_step` instead.")]
    fn mandos_check_state(&mut self, step: CheckStateStep) -> &mut Self {
        self.check_state_step(step)
    }

    #[deprecated(since = "0.39.0", note = "Renamed, use `dump_state_step` instead.")]
    fn mandos_dump_state(&mut self) -> &mut Self {
        self.dump_state_step()
    }
}
