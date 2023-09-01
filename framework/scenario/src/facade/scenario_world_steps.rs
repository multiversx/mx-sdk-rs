use multiversx_sc::types::{heap::Address, ContractCall};

use crate::{
    api::StaticApi,
    facade::ScenarioWorld,
    multiversx_sc::codec::{CodecFrom, TopEncodeMulti},
    scenario::{model::*, ScenarioRunner},
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

    pub fn sc_call_use_raw_response<S, F>(&mut self, mut step: S, use_raw_response: F) -> &mut Self
    where
        S: AsMut<ScCallStep>,
        F: FnOnce(&TxResponse),
    {
        self.run_sc_call_step(step.as_mut());
        let response = unwrap_response(&step.as_mut().response);
        use_raw_response(response);
        self
    }

    pub fn sc_call_use_result<OriginalResult, RequestedResult, F>(
        &mut self,
        step: TypedScCall<OriginalResult>,
        use_result: F,
    ) -> &mut Self
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>,
        F: FnOnce(TypedResponse<RequestedResult>),
    {
        self.sc_call_use_raw_response(step, |response| {
            let typed_response = TypedResponse::from_raw(response);
            use_result(typed_response);
        })
    }

    pub fn sc_call_get_result<OriginalResult, RequestedResult>(
        &mut self,
        mut step: TypedScCall<OriginalResult>,
    ) -> RequestedResult
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>,
    {
        self.run_sc_call_step(&mut step.sc_call_step);
        let response = unwrap_response(&step.sc_call_step.response);
        let typed_response = TypedResponse::from_raw(response);
        typed_response.result.expect("SC call failed")
    }

    /// Adds a SC query step, then executes it.
    pub fn sc_query<S>(&mut self, mut step: S) -> &mut Self
    where
        S: AsMut<ScQueryStep>,
    {
        self.run_sc_query_step(step.as_mut());
        self
    }

    pub fn sc_query_use_raw_response<S, F>(&mut self, mut step: S, use_raw_response: F) -> &mut Self
    where
        S: AsMut<ScQueryStep>,
        F: FnOnce(&TxResponse),
    {
        let base_step = step.as_mut();
        self.run_sc_query_step(base_step);
        let response = unwrap_response(&base_step.response);
        use_raw_response(response);
        self
    }

    pub fn sc_query_use_result<OriginalResult, RequestedResult, F>(
        &mut self,
        step: TypedScQuery<OriginalResult>,
        use_result: F,
    ) -> &mut Self
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>,
        F: FnOnce(TypedResponse<RequestedResult>),
    {
        self.sc_query_use_raw_response(step, |response| {
            let typed_response = TypedResponse::from_raw(response);
            use_result(typed_response);
        })
    }

    pub fn sc_query_get_result<OriginalResult, RequestedResult>(
        &mut self,
        mut step: TypedScQuery<OriginalResult>,
    ) -> RequestedResult
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>,
    {
        self.run_sc_query_step(&mut step.sc_query_step);
        let response = unwrap_response(&step.sc_query_step.response);
        let typed_response = TypedResponse::from_raw(response);
        typed_response.result.expect("SC query failed")
    }

    /// Performs a SC query to a contract, leaves no scenario trace behind.
    ///
    /// Meant to be used for the test to investigate the state of the contract.
    ///
    /// Use `mandos_sc_query` to embed the SC query in the resulting scenario.
    pub fn quick_query<CC, RequestedResult>(&mut self, contract_call: CC) -> RequestedResult
    where
        CC: ContractCall<StaticApi>,
        RequestedResult: CodecFrom<CC::OriginalResult>,
    {
        self.sc_query_get_result(ScQueryStep::new().call(contract_call))
    }

    /// Adds a SC deploy step, then executes it.
    pub fn sc_deploy<S>(&mut self, mut step: S) -> &mut Self
    where
        S: AsMut<ScDeployStep>,
    {
        self.run_sc_deploy_step(step.as_mut());
        self
    }

    pub fn sc_deploy_use_raw_response<S, F>(
        &mut self,
        mut step: S,
        use_raw_response: F,
    ) -> &mut Self
    where
        S: AsMut<ScDeployStep>,
        F: FnOnce(&TxResponse),
    {
        let base_step = step.as_mut();
        self.run_sc_deploy_step(base_step);
        let response = unwrap_response(&base_step.response);
        use_raw_response(response);
        self
    }

    pub fn sc_deploy_use_result<OriginalResult, RequestedResult, F>(
        &mut self,
        step: TypedScDeploy<OriginalResult>,
        use_result: F,
    ) -> &mut Self
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>,
        F: FnOnce(Address, TypedResponse<RequestedResult>),
    {
        self.sc_deploy_use_raw_response(step, |response| {
            let new_address = unwrap_new_address(response);
            let typed_response = TypedResponse::from_raw(response);
            use_result(new_address, typed_response);
        })
    }

    pub fn sc_deploy_get_result<OriginalResult, RequestedResult>(
        &mut self,
        mut step: TypedScDeploy<OriginalResult>,
    ) -> (Address, RequestedResult)
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>,
    {
        self.run_sc_deploy_step(&mut step.sc_deploy_step);
        let response = unwrap_response(&step.sc_deploy_step.response);
        let new_address = unwrap_new_address(response);
        let typed_response = TypedResponse::from_raw(response);
        (new_address, typed_response.result.unwrap())
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

impl TypedScCallExecutor for ScenarioWorld {
    fn execute_typed_sc_call<OriginalResult, RequestedResult>(
        &mut self,
        typed_sc_call: TypedScCall<OriginalResult>,
    ) -> RequestedResult
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>,
    {
        self.sc_call_get_result(typed_sc_call)
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
        self.sc_deploy_get_result(typed_sc_call)
    }
}

impl TypedScQueryExecutor for ScenarioWorld {
    /// Adds a SC query step, but sets the contract call data and returns the result.
    ///
    /// It also sets in the trace the expected result to be the actual returned result.
    ///
    /// It is the duty of the test developer to check that the result is actually correct after the call.
    fn execute_typed_sc_query<OriginalResult, RequestedResult>(
        &mut self,
        typed_sc_query: TypedScQuery<OriginalResult>,
    ) -> RequestedResult
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>,
    {
        self.sc_query_get_result(typed_sc_query)
    }
}

fn unwrap_response(opt_response: &Option<TxResponse>) -> &TxResponse {
    opt_response.as_ref().expect("response not processed")
}

fn unwrap_new_address(response: &TxResponse) -> Address {
    response
        .new_deployed_address
        .clone()
        .expect("missing new address after deploy")
}
