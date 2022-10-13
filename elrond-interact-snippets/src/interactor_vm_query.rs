use crate::{address_h256_to_erdrs, Interactor};
use elrond_sdk_erdrs::data::vm::VmValueRequest;
use elrond_wasm_debug::{
    elrond_wasm::{
        elrond_codec::{CodecFrom, PanicErrorHandler, TopEncodeMulti},
        types::ContractCall,
    },
    DebugApi,
};
use log::info;

impl Interactor {
    pub async fn vm_query<OriginalResult, RequestedResult>(
        &mut self,
        contract_call: ContractCall<DebugApi, OriginalResult>,
    ) -> RequestedResult
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>,
    {
        let sc_address = address_h256_to_erdrs(&contract_call.to.to_address());
        let req = VmValueRequest {
            sc_address: sc_address.clone(),
            func_name: String::from_utf8(contract_call.endpoint_name.to_boxed_bytes().into_vec())
                .unwrap(),
            args: contract_call
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
