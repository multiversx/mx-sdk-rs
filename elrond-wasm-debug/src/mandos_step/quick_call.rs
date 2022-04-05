use crate::{
    num_bigint::BigUint,
    tx_execution::{sc_call_with_async_and_callback, sc_query},
    tx_mock::TxInput,
    world_mock::BlockchainMock,
    DebugApi,
};
use elrond_wasm::{
    elrond_codec::{CodecFrom, PanicErrorHandler, TopEncodeMulti},
    types::{Address, ContractCall, H256},
};

impl BlockchainMock {
    pub fn quick_query<OriginalResult, RequestedResult>(
        &mut self,
        contract_call: ContractCall<DebugApi, OriginalResult>,
    ) -> RequestedResult
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>,
    {
        let tx_input = TxInput {
            from: contract_call.to.to_address(),
            to: contract_call.to.to_address(),
            egld_value: BigUint::from(0u32),
            esdt_values: Vec::new(),
            func_name: contract_call.endpoint_name.to_boxed_bytes().into_vec(),
            args: contract_call.arg_buffer.to_raw_args_vec(),
            gas_limit: u64::MAX,
            gas_price: 0u64,
            tx_hash: H256::zero(),
        };

        let tx_result = self.with_borrowed(|state| sc_query(tx_input, state));
        assert!(tx_result.result_status == 0, "quick query failed"); // TODO: print more
        assert!(
            tx_result.result_calls.is_empty(),
            "Can't query a view function that performs an async call"
        );
        let mut raw_result = tx_result.result_values;

        RequestedResult::multi_decode_or_handle_err(&mut raw_result, PanicErrorHandler).unwrap()
    }

    pub fn quick_call<OriginalResult, RequestedResult>(
        &mut self,
        from: Address,
        contract_call: ContractCall<DebugApi, OriginalResult>,
    ) -> RequestedResult
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>,
    {
        let tx_input = TxInput {
            from,
            to: contract_call.to.to_address(),
            egld_value: BigUint::from(0u32),
            esdt_values: Vec::new(),
            func_name: contract_call.endpoint_name.to_boxed_bytes().into_vec(),
            args: contract_call.arg_buffer.to_raw_args_vec(),
            gas_limit: contract_call.resolve_gas_limit(),
            gas_price: 0u64,
            tx_hash: H256::zero(),
        };

        // nonce gets increased irrespective of whether the tx fails or not
        self.increase_account_nonce(&tx_input.from);

        let tx_result =
            self.with_borrowed(|state| sc_call_with_async_and_callback(tx_input, state));
        let mut raw_result = tx_result.result_values;

        RequestedResult::multi_decode_or_handle_err(&mut raw_result, PanicErrorHandler).unwrap()
    }
}
