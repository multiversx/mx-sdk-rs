use multiversx_sc::types::{heap::Address, ContractCall};

use crate::{
    api::StaticApi,
    facade::ScenarioWorld,
    multiversx_sc::codec::{CodecFrom, PanicErrorHandler, TopEncodeMulti},
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
    pub fn sc_call_step<S>(&mut self, mut sc_call_step: S) -> &mut Self
    where
        S: AsMut<ScCallStep>,
    {
        let base_step = sc_call_step.as_mut();
        self.run_sc_call_step(base_step);
        self
    }

    pub fn sc_call_use_raw_response<S, F>(
        &mut self,
        mut sc_call_step: S,
        use_raw_response: F,
    ) -> &mut Self
    where
        S: AsMut<ScCallStep>,
        F: FnOnce(&TxResponse),
    {
        let base_step = sc_call_step.as_mut();
        self.get_mut_debugger_backend()
            .vm_runner
            .perform_sc_call_update_results(base_step);
        let response = unwrap_response(&base_step.response);
        use_raw_response(response);
        self
    }

    pub fn sc_call_use_result<OriginalResult, RequestedResult, F>(
        &mut self,
        mut step: TypedScCall<OriginalResult>,
        use_result: F,
    ) -> &mut Self
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>,
        F: FnOnce(TypedResponse<RequestedResult>),
    {
        self.get_mut_debugger_backend()
            .vm_runner
            .perform_sc_call_update_results(&mut step.sc_call_step);
        let response = unwrap_response(&step.sc_call_step.response);
        let typed_response = TypedResponse::from_raw(response);
        use_result(typed_response);
        self
    }

    pub fn sc_call_get_result<OriginalResult, RequestedResult>(
        &mut self,
        mut step: TypedScCall<OriginalResult>,
    ) -> RequestedResult
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>,
    {
        self.get_mut_debugger_backend()
            .vm_runner
            .perform_sc_call_update_results(&mut step.sc_call_step);
        let response = unwrap_response(&step.sc_call_step.response);
        let typed_response = TypedResponse::from_raw(response);
        typed_response.result.unwrap()
    }

    /// Adds a SC query step, then executes it.
    pub fn sc_query_step<S>(&mut self, mut sc_call_step: S) -> &mut Self
    where
        S: AsMut<ScQueryStep>,
    {
        let base_step: &mut ScQueryStep = sc_call_step.as_mut();
        self.run_sc_query_step(base_step);
        self
    }

    pub fn sc_query_use_raw_response<S, F>(
        &mut self,
        mut sc_call_step: S,
        use_raw_response: F,
    ) -> &mut Self
    where
        S: AsMut<ScQueryStep>,
        F: FnOnce(&TxResponse),
    {
        let base_step = sc_call_step.as_mut();
        self.get_mut_debugger_backend()
            .vm_runner
            .perform_sc_query_update_results(base_step);
        let response = unwrap_response(&base_step.response);
        use_raw_response(response);
        self
    }

    pub fn sc_query_use_result<OriginalResult, RequestedResult, F>(
        &mut self,
        mut step: TypedScQuery<OriginalResult>,
        use_result: F,
    ) -> &mut Self
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>,
        F: FnOnce(TypedResponse<RequestedResult>),
    {
        self.get_mut_debugger_backend()
            .vm_runner
            .perform_sc_query_update_results(&mut step.sc_query_step);
        let response = unwrap_response(&step.sc_query_step.response);
        let typed_response = TypedResponse::from_raw(response);
        use_result(typed_response);
        self
    }

    pub fn sc_query_get_result<OriginalResult, RequestedResult>(
        &mut self,
        mut step: TypedScQuery<OriginalResult>,
    ) -> RequestedResult
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>,
    {
        self.get_mut_debugger_backend()
            .vm_runner
            .perform_sc_query_update_results(&mut step.sc_query_step);
        let response = unwrap_response(&step.sc_query_step.response);
        let typed_response = TypedResponse::from_raw(response);
        typed_response.result.unwrap()
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
        let vm_runner = &mut self.get_mut_debugger_backend().vm_runner;
        let typed_sc_query = ScQueryStep::new().call(contract_call);
        let tx_result = vm_runner.perform_sc_query(&typed_sc_query.sc_query_step);
        let mut raw_result = tx_result.result_values;
        RequestedResult::multi_decode_or_handle_err(&mut raw_result, PanicErrorHandler).unwrap()
    }

    /// Adds a SC deploy step, then executes it.
    pub fn sc_deploy_step<S>(&mut self, mut step: S) -> &mut Self
    where
        S: AsMut<ScDeployStep>,
    {
        let base_step = step.as_mut();
        self.run_sc_deploy_step(base_step);
        self
    }

    pub fn sc_deploy_use_raw_response<S, F>(
        &mut self,
        mut sc_deploy_step: S,
        use_raw_response: F,
    ) -> &mut Self
    where
        S: AsMut<ScDeployStep>,
        F: FnOnce(&TxResponse),
    {
        let base_step = sc_deploy_step.as_mut();
        self.get_mut_debugger_backend()
            .vm_runner
            .perform_sc_deploy_update_results(base_step);
        let response = unwrap_response(&base_step.response);
        use_raw_response(response);
        self
    }

    pub fn sc_deploy_use_result<OriginalResult, RequestedResult, F>(
        &mut self,
        mut step: TypedScDeploy<OriginalResult>,
        use_result: F,
    ) -> &mut Self
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>,
        F: FnOnce(Address, TypedResponse<RequestedResult>),
    {
        self.get_mut_debugger_backend()
            .vm_runner
            .perform_sc_deploy_update_results(&mut step.sc_deploy_step);
        let response = unwrap_response(&step.sc_deploy_step.response);
        let new_address = unwrap_new_address(response);
        let typed_response = TypedResponse::from_raw(response);
        use_result(new_address, typed_response);
        self
    }

    pub fn sc_deploy_use_new_address<S, F>(
        &mut self,
        mut sc_deploy_step: S,
        use_new_address: F,
    ) -> &mut Self
    where
        S: AsMut<ScDeployStep>,
        F: FnOnce(Address),
    {
        let base_step = sc_deploy_step.as_mut();
        self.get_mut_debugger_backend()
            .vm_runner
            .perform_sc_deploy_update_results(base_step);
        let response = unwrap_response(&base_step.response);
        let new_address = unwrap_new_address(response);
        use_new_address(new_address);
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

impl TypedScCallExecutor for ScenarioWorld {
    fn execute_typed_sc_call<OriginalResult, RequestedResult>(
        &mut self,
        typed_sc_call: TypedScCall<OriginalResult>,
    ) -> RequestedResult
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>,
    {
        self.get_mut_debugger_backend()
            .vm_runner
            .perform_sc_call_get_result(typed_sc_call)
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
        self.get_mut_debugger_backend()
            .vm_runner
            .perform_sc_deploy_get_result(typed_sc_call)
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
        let debugger = self.get_mut_debugger_backend();
        let mut sc_query_step: ScQueryStep = typed_sc_query.into();
        let tx_result = debugger.vm_runner.perform_sc_query(&sc_query_step);

        let mut tx_expect = TxExpect::ok();
        for raw_result in &tx_result.result_values {
            let result_hex_string = format!("0x{}", hex::encode(raw_result));
            tx_expect = tx_expect.result(result_hex_string.as_str());
        }
        sc_query_step = sc_query_step.expect(tx_expect);
        if let Some(trace) = &mut debugger.trace {
            trace.run_sc_query_step(&sc_query_step);
        }

        let mut raw_results = tx_result.result_values;
        RequestedResult::multi_decode_or_handle_err(&mut raw_results, PanicErrorHandler).unwrap()
    }
}

impl ScenarioWorld {
    #[deprecated(since = "0.39.0", note = "Renamed, use `set_state_step` instead.")]
    pub fn mandos_set_state(&mut self, step: SetStateStep) -> &mut Self {
        self.set_state_step(step)
    }

    #[deprecated(since = "0.39.0", note = "Renamed, use `sc_call_step` instead.")]
    pub fn mandos_sc_call(&mut self, step: ScCallStep) -> &mut Self {
        self.sc_call_step(step)
    }

    #[deprecated(since = "0.39.0", note = "Renamed, use `sc_query_step` instead.")]
    pub fn mandos_sc_query(&mut self, step: ScQueryStep) -> &mut Self {
        self.sc_query_step(step)
    }

    #[deprecated(since = "0.39.0", note = "Renamed, use `sc_deploy_step` instead.")]
    pub fn mandos_sc_deploy(&mut self, step: ScDeployStep) -> &mut Self {
        self.sc_deploy_step(step)
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

fn unwrap_response(opt_response: &Option<TxResponse>) -> &TxResponse {
    opt_response.as_ref().expect("response not processed")
}

fn unwrap_new_address(response: &TxResponse) -> Address {
    response
        .new_deployed_address
        .clone()
        .expect("missing new address after deploy")
}
