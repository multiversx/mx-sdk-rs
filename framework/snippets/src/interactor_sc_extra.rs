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
        use_result(self.sc_call_get_result(step).await);
        self
    }

    pub async fn sc_call_get_result<OriginalResult, RequestedResult>(
        &mut self,
        mut step: TypedScCall<OriginalResult>,
    ) -> TypedResponse<RequestedResult>
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>,
    {
        self.sc_call(step.as_mut()).await;
        let response = unwrap_response(&step.as_mut().response);
        TypedResponse::from_raw(response)
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
        use_result(self.sc_query_get_result(step).await);
        self
    }

    pub async fn sc_query_get_result<OriginalResult, RequestedResult>(
        &mut self,
        mut step: TypedScQuery<OriginalResult>,
    ) -> TypedResponse<RequestedResult>
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>,
    {
        self.sc_query(step.as_mut()).await;
        let response = unwrap_response(&step.sc_query_step.response);
        TypedResponse::from_raw(response)
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
        let (new_address, response) = self.sc_deploy_get_result(step).await;
        use_result(new_address, response);
        self
    }

    pub async fn sc_deploy_get_result<OriginalResult, RequestedResult>(
        &mut self,
        mut step: TypedScDeploy<OriginalResult>,
    ) -> (Address, TypedResponse<RequestedResult>)
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>,
    {
        self.sc_deploy(step.as_mut()).await;
        let response = unwrap_response(&step.sc_deploy_step.response);
        let new_address = unwrap_new_address(response);
        let response = TypedResponse::from_raw(response);
        (new_address, response)
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
