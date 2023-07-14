use crate::{address_h256_to_erdrs, Interactor};
use log::info;
use multiversx_sc_scenario::{
    api::StaticApi,
    mandos_system::ScenarioRunner,
    multiversx_sc::{codec::CodecFrom, types::ContractCall},
    scenario_model::{ScQueryStep, TxResponse},
};
use multiversx_sdk::data::vm::VmValueRequest;

impl Interactor {
    pub async fn sc_query<S>(&mut self, mut step: S) -> &mut Self
    where
        S: AsMut<ScQueryStep>,
    {
        self.perform_sc_query(step.as_mut()).await;
        self
    }

    pub async fn perform_sc_query(&mut self, step: &mut ScQueryStep) {
        let sc_address = address_h256_to_erdrs(&step.tx.to.to_address());
        let req = VmValueRequest {
            sc_address: sc_address.clone(),
            func_name: step.tx.function.clone(),
            args: step
                .tx
                .arguments
                .iter()
                .map(|arg| hex::encode(&arg.value))
                .collect(),
            caller: sc_address,
            value: "0".to_string(),
        };
        let result = self
            .proxy
            .execute_vmquery(&req)
            .await
            .expect("error executing VM query");

        info!("{:#?}", result);

        let raw_results: Vec<Vec<u8>> = result
            .data
            .return_data
            .iter()
            .map(|result| base64::decode(result).expect("query result base64 decode error"))
            .collect();
        step.save_response(TxResponse::from_raw_results(raw_results));

        self.pre_runners.run_sc_query_step(step);
        self.post_runners.run_sc_query_step(step);
    }

    #[deprecated(since = "0.42.0", note = "Was renamed to `quick_query`.")]
    pub async fn vm_query<CC, RequestedResult>(&mut self, contract_call: CC) -> RequestedResult
    where
        CC: ContractCall<StaticApi>,
        RequestedResult: CodecFrom<CC::OriginalResult>,
    {
        self.quick_query(contract_call).await
    }
}
