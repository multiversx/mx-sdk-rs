use crate::Interactor;
use multiversx_sc_scenario::{
    api::StaticApi,
    multiversx_sc::{
        codec::{CodecFrom, TopEncodeMulti},
        types::{Address, ContractCall},
    },
    scenario_model::{
        ScCallStep, ScDeployStep, ScQueryStep, TxResponse, TypedResponse, TypedScCall,
        TypedScDeploy, TypedScQuery,
    },
};

impl Interactor {
    pub async fn sc_call_use_raw_response<S, F>(
        &mut self,
        mut step: S,
        use_raw_response: F,
    ) -> &mut Self
    where
        S: AsMut<ScCallStep>,
        F: FnOnce(&TxResponse),
    {
        self.sc_call(step.as_mut()).await;
        let response = unwrap_response(&step.as_mut().response);
        use_raw_response(response);
        self
    }

    pub async fn sc_call_use_result<OriginalResult, RequestedResult, F>(
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
        .await
    }

    pub async fn sc_query_use_raw_response<S, F>(
        &mut self,
        mut step: S,
        use_raw_response: F,
    ) -> &mut Self
    where
        S: AsMut<ScQueryStep>,
        F: FnOnce(&TxResponse),
    {
        self.sc_query(step.as_mut()).await;
        let response = unwrap_response(&step.as_mut().response);
        use_raw_response(response);
        self
    }

    pub async fn sc_query_use_result<OriginalResult, RequestedResult, F>(
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
        .await
    }

    pub async fn sc_query_get_result<OriginalResult, RequestedResult>(
        &mut self,
        mut step: TypedScQuery<OriginalResult>,
    ) -> RequestedResult
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>,
    {
        self.sc_query(step.as_mut()).await;
        let response = unwrap_response(&step.sc_query_step.response);
        let typed_response = TypedResponse::from_raw(response);
        typed_response.result.unwrap()
    }

    pub async fn quick_query<CC, RequestedResult>(&mut self, contract_call: CC) -> RequestedResult
    where
        CC: ContractCall<StaticApi>,
        RequestedResult: CodecFrom<CC::OriginalResult>,
    {
        let mut typed_sc_query = ScQueryStep::new().call(contract_call);
        self.sc_query(&mut typed_sc_query).await;
        let response = unwrap_response(&typed_sc_query.sc_query_step.response);
        let typed_response = TypedResponse::from_raw(response);
        typed_response.result.unwrap()
    }

    pub async fn sc_deploy_use_raw_response<S, F>(
        &mut self,
        mut step: S,
        use_raw_response: F,
    ) -> &mut Self
    where
        S: AsMut<ScDeployStep>,
        F: FnOnce(&TxResponse),
    {
        self.sc_deploy(step.as_mut()).await;
        let response = unwrap_response(&step.as_mut().response);
        use_raw_response(response);
        self
    }

    pub async fn sc_deploy_use_result<OriginalResult, RequestedResult, F>(
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
        .await
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
