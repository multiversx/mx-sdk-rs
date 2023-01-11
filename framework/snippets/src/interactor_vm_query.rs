use crate::{address_h256_to_erdrs, Interactor};
use log::info;
use multiversx_sc_scenario::{
    multiversx_sc::{
        codec::{CodecFrom, PanicErrorHandler},
        types::ContractCall,
    },
    DebugApi,
};
use multiversx_sdk::data::vm::VmValueRequest;

impl Interactor {
    pub async fn vm_query<CC, RequestedResult>(&mut self, contract_call: CC) -> RequestedResult
    where
        CC: ContractCall<DebugApi>,
        RequestedResult: CodecFrom<CC::OriginalResult>,
    {
        let full_cc = contract_call.into_normalized();
        let sc_address = address_h256_to_erdrs(&full_cc.basic.to.to_address());
        let req = VmValueRequest {
            sc_address: sc_address.clone(),
            func_name: String::from_utf8(full_cc.basic.endpoint_name.to_boxed_bytes().into_vec())
                .unwrap(),
            args: full_cc
                .basic
                .arg_buffer
                .raw_arg_iter()
                .map(|arg| hex::encode(arg.to_boxed_bytes().as_slice()))
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

        let mut raw_results: Vec<Vec<u8>> = result
            .data
            .return_data
            .iter()
            .map(|result| base64::decode(result).expect("query result base64 decode error"))
            .collect();
        RequestedResult::multi_decode_or_handle_err(&mut raw_results, PanicErrorHandler).unwrap()
    }
}
