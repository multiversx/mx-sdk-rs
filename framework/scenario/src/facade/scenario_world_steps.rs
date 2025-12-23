use crate::{
    facade::ScenarioWorld,
    scenario::{ScenarioRunner, model::*},
};

impl ScenarioWorld {
    /// Imports and processes steps from an external scenario file.
    pub fn external_steps(&mut self, step: ExternalStepsStep) -> &mut Self {
        self.run_external_steps(&step);
        self
    }

    /// Adds a SC call step, then executes it.
    pub fn set_state_step(&mut self, step: SetStateStep) -> &mut Self {
        self.run_set_state_step(&step);
        self
    }

    /// Adds a SC call step, then executes it.
    pub fn sc_call<S>(&mut self, mut step: S) -> &mut Self
    where
        S: AsMut<ScCallStep>,
    {
        self.run_sc_call_step(step.as_mut());
        self
    }

    /// Adds a SC query step, then executes it.
    pub fn sc_query<S>(&mut self, mut step: S) -> &mut Self
    where
        S: AsMut<ScQueryStep>,
    {
        self.run_sc_query_step(step.as_mut());
        self
    }

    /// Adds a SC deploy step, then executes it.
    pub fn sc_deploy<S>(&mut self, mut step: S) -> &mut Self
    where
        S: AsMut<ScDeployStep>,
    {
        self.run_sc_deploy_step(step.as_mut());
        self
    }

    /// Adds a simple transfer step, then executes it.
    pub fn transfer_step(&mut self, step: TransferStep) -> &mut Self {
        self.run_transfer_step(&step);
        self
    }

    /// Adds a validator reward step, then executes it.
    pub fn validator_reward_step(&mut self, step: ValidatorRewardStep) -> &mut Self {
        self.run_validator_reward_step(&step);
        self
    }

    /// Adds a check state step, then executes it.
    pub fn check_state_step(&mut self, step: CheckStateStep) -> &mut Self {
        self.run_check_state_step(&step);
        self
    }

    /// Adds a dump state step, then executes it.
    pub fn dump_state_step(&mut self) -> &mut Self {
        self.run_dump_state_step();
        self
    }
}
