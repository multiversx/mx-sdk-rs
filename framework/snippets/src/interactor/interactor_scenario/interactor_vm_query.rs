#![allow(deprecated)]

use std::process;

use super::error_message::query_err_message;
use crate::InteractorBase;
use multiversx_sc_scenario::{
    api::StaticApi,
    imports::ReturnCode,
    mandos_system::ScenarioRunner,
    multiversx_sc::{abi::TypeAbiFrom, codec::TopDecodeMulti, types::ContractCall},
    scenario_model::{ScQueryStep, TxResponse, TxResponseStatus},
};
use multiversx_sdk::data::vm::VMQueryInput;
use multiversx_sdk::gateway::{GatewayAsyncService, VMQueryRequest};

impl<GatewayProxy> InteractorBase<GatewayProxy>
where
    GatewayProxy: GatewayAsyncService,
{
    pub async fn sc_query<S>(&mut self, mut step: S) -> &mut Self
    where
        S: AsMut<ScQueryStep>,
    {
        self.perform_sc_query(step.as_mut()).await;
        self
    }

    pub async fn perform_sc_query(&mut self, step: &mut ScQueryStep) {
        let hrp = self.network_config.address_hrp.clone();

        let sc_address = step.tx.to.to_address().to_bech32(&hrp);
        let req = VMQueryInput {
            sc_address,
            func_name: step.tx.function.clone(),
            args: step
                .tx
                .arguments
                .iter()
                .map(|arg| hex::encode(&arg.value))
                .collect(),
        };
        let result = match self.proxy.request(VMQueryRequest(&req)).await {
            Ok(r) => r,
            Err(err) => {
                query_err_message(&err);
                process::exit(1);
            },
        };

        if result.data.is_ok() {
            let raw_results = result.data.return_data_base64_decode();
            step.save_response(TxResponse::from_raw_results(raw_results));
        } else {
            step.save_response(TxResponse {
                tx_error: TxResponseStatus {
                    status: ReturnCode::VMQueryError,
                    message: result.data.return_message.clone(),
                },
                ..Default::default()
            })
        }

        self.pre_runners.run_sc_query_step(step);
        self.post_runners.run_sc_query_step(step);
    }

    #[deprecated(since = "0.42.0", note = "Was renamed to `quick_query`.")]
    pub async fn vm_query<CC, RequestedResult>(&mut self, contract_call: CC) -> RequestedResult
    where
        CC: ContractCall<StaticApi>,
        RequestedResult: TopDecodeMulti + TypeAbiFrom<CC::OriginalResult>,
    {
        self.quick_query(contract_call).await
    }
}
