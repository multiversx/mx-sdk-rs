use crate::{facade::ScenarioWorld, scenario::model::*};

impl ScenarioWorld {
    #[deprecated(since = "0.42.0", note = "Renamed to  `sc_call`.")]
    pub fn sc_call_step<S>(&mut self, step: S) -> &mut Self
    where
        S: AsMut<ScCallStep>,
    {
        self.sc_call(step)
    }

    #[deprecated(since = "0.42.0", note = "Renamed to  `sc_query`.")]
    pub fn sc_query_step<S>(&mut self, step: S) -> &mut Self
    where
        S: AsMut<ScQueryStep>,
    {
        self.sc_query(step)
    }

    #[deprecated(since = "0.42.0", note = "Renamed to  `sc_deploy`.")]
    pub fn sc_deploy_step<S>(&mut self, step: S) -> &mut Self
    where
        S: AsMut<ScDeployStep>,
    {
        self.sc_deploy(step)
    }

    #[deprecated(since = "0.39.0", note = "Renamed, use `set_state_step` instead.")]
    pub fn mandos_set_state(&mut self, step: SetStateStep) -> &mut Self {
        self.set_state_step(step)
    }

    #[deprecated(since = "0.39.0", note = "Renamed, use `sc_call_step` instead.")]
    pub fn mandos_sc_call(&mut self, step: ScCallStep) -> &mut Self {
        self.sc_call(step)
    }

    #[deprecated(since = "0.39.0", note = "Renamed, use `sc_query_step` instead.")]
    pub fn mandos_sc_query(&mut self, step: ScQueryStep) -> &mut Self {
        self.sc_query(step)
    }

    #[deprecated(since = "0.39.0", note = "Renamed, use `sc_deploy_step` instead.")]
    pub fn mandos_sc_deploy(&mut self, step: ScDeployStep) -> &mut Self {
        self.sc_deploy(step)
    }

    #[deprecated(since = "0.39.0", note = "Renamed, use `transfer_step` instead.")]
    pub fn mandos_transfer(&mut self, step: TransferStep) -> &mut Self {
        self.transfer_step(step)
    }

    #[deprecated(
        since = "0.39.0",
        note = "Renamed, use `validator_reward_step` instead."
    )]
    pub fn mandos_validator_reward(&mut self, step: ValidatorRewardStep) -> &mut Self {
        self.validator_reward_step(step)
    }

    #[deprecated(since = "0.39.0", note = "Renamed, use `check_state_step` instead.")]
    pub fn mandos_check_state(&mut self, step: CheckStateStep) -> &mut Self {
        self.check_state_step(step)
    }

    #[deprecated(since = "0.39.0", note = "Renamed, use `dump_state_step` instead.")]
    pub fn mandos_dump_state(&mut self) -> &mut Self {
        self.dump_state_step()
    }
}
