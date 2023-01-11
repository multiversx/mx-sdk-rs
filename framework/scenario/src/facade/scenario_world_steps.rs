use crate::{
    multiversx_sc::{
        codec::{CodecFrom, TopEncodeMulti},
        types::Address,
    },
    ScenarioWorld,
};
use multiversx_chain_vm::mandos_system::model::*;

impl ScenarioWorld {
    pub fn mandos_set_state(&mut self, set_state_step: SetStateStep) -> &mut Self {
        self.blockchain_mock.mandos_set_state(set_state_step);
        self
    }

    /// Adds a SC call step, as specified in the `sc_call_step` argument, then executes it.
    pub fn mandos_sc_call(&mut self, sc_call_step: ScCallStep) -> &mut Self {
        self.blockchain_mock.mandos_sc_call(sc_call_step);
        self
    }

    /// Adds a SC query step, as specified in the `sc_query_step` argument, then executes it.
    pub fn mandos_sc_query(&mut self, sc_query_step: ScQueryStep) -> &mut Self {
        self.blockchain_mock.mandos_sc_query(sc_query_step);
        self
    }

    /// Adds a SC deploy step, as specified in the `sc_deploy_step` argument, then executes it.
    pub fn mandos_sc_deploy(&mut self, sc_deploy_step: ScDeployStep) -> &mut Self {
        self.blockchain_mock.mandos_sc_deploy(sc_deploy_step);
        self
    }

    pub fn mandos_transfer(&mut self, transfer_step: TransferStep) -> &mut Self {
        self.blockchain_mock.mandos_transfer(transfer_step);
        self
    }

    pub fn mandos_validator_reward(
        &mut self,
        validator_rewards_step: ValidatorRewardStep,
    ) -> &mut Self {
        self.blockchain_mock
            .mandos_validator_reward(validator_rewards_step);
        self
    }

    pub fn mandos_check_state(&mut self, check_state_step: CheckStateStep) -> &mut Self {
        self.blockchain_mock.mandos_check_state(check_state_step);
        self
    }

    pub fn mandos_dump_state(&mut self) -> &mut Self {
        self.blockchain_mock.mandos_dump_state();
        self
    }
}

impl TypedScCallExecutor for ScenarioWorld {
    fn execute_typed_sc_call<OriginalResult, RequestedResult>(
        &mut self,
        typed_sc_call: TypedScCall<OriginalResult>,
    ) -> RequestedResult
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>,
    {
        self.blockchain_mock.execute_typed_sc_call(typed_sc_call)
    }
}

impl TypedScDeployExecutor for ScenarioWorld {
    fn execute_typed_sc_deploy<OriginalResult, RequestedResult>(
        &mut self,
        typed_sc_call: TypedScDeploy<OriginalResult>,
    ) -> (Address, RequestedResult)
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>,
    {
        self.blockchain_mock.execute_typed_sc_deploy(typed_sc_call)
    }
}

impl TypedScQueryExecutor for ScenarioWorld {
    fn execute_typed_sc_query<OriginalResult, RequestedResult>(
        &mut self,
        typed_sc_call: TypedScQuery<OriginalResult>,
    ) -> RequestedResult
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>,
    {
        self.blockchain_mock.execute_typed_sc_query(typed_sc_call)
    }
}
