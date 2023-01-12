use crate::{
    multiversx_sc::{
        codec::{CodecFrom, TopEncodeMulti},
        types::Address,
    },
    ScenarioWorld,
};
use multiversx_chain_vm::scenario::model::*;

impl StepHandler for ScenarioWorld {
    fn set_state_step(&mut self, step: SetStateStep) -> &mut Self {
        self.blockchain_mock.set_state_step(step);
        self
    }

    /// Adds a SC call step, as specified in the `sc_call_step` argument, then executes it.
    fn sc_call_step(&mut self, step: ScCallStep) -> &mut Self {
        self.blockchain_mock.sc_call_step(step);
        self
    }

    /// Adds a SC query step, as specified in the `sc_query_step` argument, then executes it.
    fn sc_query_step(&mut self, step: ScQueryStep) -> &mut Self {
        self.blockchain_mock.sc_query_step(step);
        self
    }

    /// Adds a SC deploy step, as specified in the `sc_deploy_step` argument, then executes it.
    fn sc_deploy_step(&mut self, step: ScDeployStep) -> &mut Self {
        self.blockchain_mock.sc_deploy_step(step);
        self
    }

    fn transfer_step(&mut self, step: TransferStep) -> &mut Self {
        self.blockchain_mock.transfer_step(step);
        self
    }

    fn validator_reward_step(&mut self, step: ValidatorRewardStep) -> &mut Self {
        self.blockchain_mock.validator_reward_step(step);
        self
    }

    fn check_state_step(&mut self, step: CheckStateStep) -> &mut Self {
        self.blockchain_mock.check_state_step(step);
        self
    }

    fn dump_state_step(&mut self) -> &mut Self {
        self.blockchain_mock.dump_state_step();
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
