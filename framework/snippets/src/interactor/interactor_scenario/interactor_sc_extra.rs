#![allow(deprecated)]

use crate::InteractorBase;
use multiversx_sc_scenario::{
    multiversx_sc::{
        abi::TypeAbiFrom,
        codec::{TopDecodeMulti, TopEncodeMulti},
        types::Address,
    },
    scenario_model::{
        ScCallStep, ScDeployStep, ScQueryStep, TxResponse, TypedResponse, TypedScCall,
        TypedScDeploy, TypedScQuery,
    },
};
use multiversx_sdk::gateway::GatewayAsyncService;

impl<GatewayProxy> InteractorBase<GatewayProxy>
where
    GatewayProxy: GatewayAsyncService,
{
    #[deprecated(
        since = "0.49.0",
        note = "Please use the unified transaction syntax instead."
    )]
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

    #[deprecated(
        since = "0.49.0",
        note = "Please use the unified transaction syntax instead."
    )]
    pub async fn sc_call_use_result<OriginalResult, RequestedResult, F>(
        &mut self,
        step: TypedScCall<OriginalResult>,
        use_result: F,
    ) -> &mut Self
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: TopDecodeMulti + TypeAbiFrom<OriginalResult>,
        F: FnOnce(TypedResponse<RequestedResult>),
    {
        use_result(self.sc_call_get_result(step).await);
        self
    }

    #[deprecated(
        since = "0.49.0",
        note = "Please use the unified transaction syntax instead."
    )]
    pub async fn sc_call_get_result<OriginalResult, RequestedResult>(
        &mut self,
        mut step: TypedScCall<OriginalResult>,
    ) -> TypedResponse<RequestedResult>
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: TopDecodeMulti + TypeAbiFrom<OriginalResult>,
    {
        self.sc_call(step.as_mut()).await;
        let response = unwrap_response(&step.as_mut().response);
        TypedResponse::from_raw(response)
    }

    #[deprecated(
        since = "0.49.0",
        note = "Please use the unified transaction syntax instead."
    )]
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

    #[deprecated(
        since = "0.49.0",
        note = "Please use the unified transaction syntax instead."
    )]
    pub async fn sc_query_use_result<OriginalResult, RequestedResult, F>(
        &mut self,
        step: TypedScQuery<OriginalResult>,
        use_result: F,
    ) -> &mut Self
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: TopDecodeMulti + TypeAbiFrom<OriginalResult>,
        F: FnOnce(TypedResponse<RequestedResult>),
    {
        use_result(self.sc_query_get_result(step).await);
        self
    }

    #[deprecated(
        since = "0.49.0",
        note = "Please use the unified transaction syntax instead."
    )]
    pub async fn sc_query_get_result<OriginalResult, RequestedResult>(
        &mut self,
        mut step: TypedScQuery<OriginalResult>,
    ) -> TypedResponse<RequestedResult>
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: TopDecodeMulti + TypeAbiFrom<OriginalResult>,
    {
        self.sc_query(step.as_mut()).await;
        let response = unwrap_response(&step.sc_query_step.response);
        TypedResponse::from_raw(response)
    }

    #[deprecated(
        since = "0.49.0",
        note = "Please use the unified transaction syntax instead."
    )]
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

    #[deprecated(
        since = "0.49.0",
        note = "Please use the unified transaction syntax instead."
    )]
    pub async fn sc_deploy_use_result<OriginalResult, RequestedResult, F>(
        &mut self,
        step: TypedScDeploy<OriginalResult>,
        use_result: F,
    ) -> &mut Self
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: TopDecodeMulti + TypeAbiFrom<OriginalResult>,
        F: FnOnce(Address, TypedResponse<RequestedResult>),
    {
        let (new_address, response) = self.sc_deploy_get_result(step).await;
        use_result(new_address, response);
        self
    }

    #[deprecated(
        since = "0.49.0",
        note = "Please use the unified transaction syntax instead."
    )]
    pub async fn sc_deploy_get_result<OriginalResult, RequestedResult>(
        &mut self,
        mut step: TypedScDeploy<OriginalResult>,
    ) -> (Address, TypedResponse<RequestedResult>)
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: TopDecodeMulti + TypeAbiFrom<OriginalResult>,
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
