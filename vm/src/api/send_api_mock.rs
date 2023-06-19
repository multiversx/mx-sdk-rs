use crate::{
    num_bigint,
    tx_execution::{deploy_contract, execute_builtin_function_or_default},
    tx_mock::{
        async_call_tx_input, AsyncCallTxData, BlockchainUpdate, Promise, TxCache, TxFunctionName,
        TxInput, TxPanic, TxResult, TxTokenTransfer,
    },
    DebugApi,
};
use multiversx_sc::{
    api::{
        BlockchainApiImpl, RawHandle, SendApi, SendApiImpl, ESDT_MULTI_TRANSFER_FUNC_NAME,
        ESDT_NFT_TRANSFER_FUNC_NAME, ESDT_TRANSFER_FUNC_NAME, UPGRADE_CONTRACT_FUNC_NAME,
    },
    codec::top_encode_to_vec_u8,
    err_msg,
    types::{heap::Address, CodeMetadata},
};
use num_traits::Zero;

impl DebugApi {
    fn append_endpoint_name_and_args(
        args: &mut Vec<Vec<u8>>,
        endpoint_name: TxFunctionName,
        arg_buffer: Vec<Vec<u8>>,
    ) {
        if !endpoint_name.is_empty() {
            args.push(endpoint_name.into_bytes());
            args.extend(arg_buffer.into_iter());
        }
    }

    fn sync_call_post_processing(
        &self,
        tx_result: TxResult,
        blockchain_updates: BlockchainUpdate,
    ) -> Vec<Vec<u8>> {
        self.blockchain_cache().commit_updates(blockchain_updates);

        self.result_borrow_mut().merge_after_sync_call(&tx_result);

        tx_result.result_values
    }

    fn create_async_call_data(
        &self,
        to: Address,
        egld_value: num_bigint::BigUint,
        func_name: TxFunctionName,
        arguments: Vec<Vec<u8>>,
    ) -> AsyncCallTxData {
        let contract_address = &self.input_ref().to;
        let tx_hash = self.get_tx_hash_legacy();
        AsyncCallTxData {
            from: contract_address.clone(),
            to,
            call_value: egld_value,
            endpoint_name: func_name,
            arguments,
            tx_hash,
        }
    }

    fn prepare_execute_on_dest_context_input(
        &self,
        to: Address,
        egld_value: num_bigint::BigUint,
        func_name: TxFunctionName,
        args: Vec<Vec<u8>>,
    ) -> TxInput {
        let async_call_data = self.create_async_call_data(to, egld_value, func_name, args);
        async_call_tx_input(&async_call_data)
    }

    fn perform_execute_on_dest_context(
        &self,
        to: Address,
        egld_value: num_bigint::BigUint,
        func_name: TxFunctionName,
        args: Vec<Vec<u8>>,
    ) -> Vec<Vec<u8>> {
        let tx_input = self.prepare_execute_on_dest_context_input(to, egld_value, func_name, args);
        let tx_cache = TxCache::new(self.blockchain_cache_rc());
        let (tx_result, blockchain_updates) =
            execute_builtin_function_or_default(tx_input, tx_cache);

        if tx_result.result_status == 0 {
            self.sync_call_post_processing(tx_result, blockchain_updates)
        } else {
            // also kill current execution
            std::panic::panic_any(TxPanic {
                status: tx_result.result_status,
                message: tx_result.result_message,
            })
        }
    }

    fn perform_transfer_execute(
        &self,
        to: Address,
        egld_value: num_bigint::BigUint,
        func_name: TxFunctionName,
        arguments: Vec<Vec<u8>>,
    ) -> Vec<Vec<u8>> {
        let async_call_data = self.create_async_call_data(to, egld_value, func_name, arguments);
        let tx_input = async_call_tx_input(&async_call_data);
        let tx_cache = TxCache::new(self.blockchain_cache_rc());
        let (tx_result, blockchain_updates) =
            execute_builtin_function_or_default(tx_input, tx_cache);

        if tx_result.result_status == 0 {
            self.result_borrow_mut().all_calls.push(async_call_data);

            self.sync_call_post_processing(tx_result, blockchain_updates)
        } else {
            // also kill current execution
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::ERROR_SIGNALLED_BY_SMARTCONTRACT.to_string(),
            })
        }
    }

    fn perform_transfer_execute_esdt(
        &self,
        to: Address,
        token: Vec<u8>,
        amount: num_bigint::BigUint,
        _gas_limit: u64,
        func_name: TxFunctionName,
        arguments: Vec<Vec<u8>>,
    ) -> Result<(), &'static [u8]> {
        let mut args = vec![token, amount.to_bytes_be()];
        Self::append_endpoint_name_and_args(&mut args, func_name, arguments);

        let _ = self.perform_transfer_execute(
            to,
            num_bigint::BigUint::zero(),
            ESDT_TRANSFER_FUNC_NAME.into(),
            args,
        );

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn perform_transfer_execute_nft(
        &self,
        to: Address,
        token: Vec<u8>,
        nonce: u64,
        amount: num_bigint::BigUint,
        _gas_limit: u64,
        func_name: TxFunctionName,
        arguments: Vec<Vec<u8>>,
    ) -> Result<(), &'static [u8]> {
        let contract_address = self.input_ref().to.clone();

        let mut args = vec![
            token,
            top_encode_to_vec_u8(&nonce).unwrap(),
            amount.to_bytes_be(),
            to.to_vec(),
        ];

        Self::append_endpoint_name_and_args(&mut args, func_name, arguments);

        let _ = self.perform_transfer_execute(
            contract_address,
            num_bigint::BigUint::zero(),
            ESDT_NFT_TRANSFER_FUNC_NAME.into(),
            args,
        );

        Ok(())
    }

    fn perform_transfer_execute_multi(
        &self,
        to: Address,
        payments: Vec<TxTokenTransfer>,
        _gas_limit: u64,
        endpoint_name: TxFunctionName,
        arguments: Vec<Vec<u8>>,
    ) -> Result<(), &'static [u8]> {
        let contract_address = self.input_ref().to.clone();

        let mut args = vec![to.to_vec(), top_encode_to_vec_u8(&payments.len()).unwrap()];

        for payment in payments.into_iter() {
            let token_bytes = top_encode_to_vec_u8(&payment.token_identifier).unwrap();
            args.push(token_bytes);
            let nonce_bytes = top_encode_to_vec_u8(&payment.nonce).unwrap();
            args.push(nonce_bytes);
            let amount_bytes = top_encode_to_vec_u8(&payment.value).unwrap();
            args.push(amount_bytes);
        }

        Self::append_endpoint_name_and_args(&mut args, endpoint_name, arguments);

        let _ = self.perform_transfer_execute(
            contract_address,
            num_bigint::BigUint::zero(),
            ESDT_MULTI_TRANSFER_FUNC_NAME.into(),
            args,
        );

        Ok(())
    }

    fn perform_deploy(
        &self,
        egld_value: num_bigint::BigUint,
        contract_code: Vec<u8>,
        _code_metadata: CodeMetadata,
        args: Vec<Vec<u8>>,
    ) -> (Address, Vec<Vec<u8>>) {
        let contract_address = &self.input_ref().to;
        let tx_hash = self.get_tx_hash_legacy();
        let tx_input = TxInput {
            from: contract_address.clone(),
            to: Address::zero(),
            egld_value,
            esdt_values: Vec::new(),
            func_name: TxFunctionName::EMPTY,
            args,
            gas_limit: 1000,
            gas_price: 0,
            tx_hash,
            ..Default::default()
        };

        let tx_cache = TxCache::new(self.blockchain_cache_rc());
        tx_cache.increase_acount_nonce(contract_address);
        let (tx_result, new_address, blockchain_updates) =
            deploy_contract(tx_input, contract_code, tx_cache);

        if tx_result.result_status == 0 {
            (
                new_address,
                self.sync_call_post_processing(tx_result, blockchain_updates),
            )
        } else {
            // also kill current execution
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::ERROR_SIGNALLED_BY_SMARTCONTRACT.to_string(),
            })
        }
    }

    fn perform_async_call(&self, call: AsyncCallTxData) -> ! {
        // the cell is no longer needed, since we end in a panic
        let mut tx_result = self.extract_result();
        tx_result.all_calls.push(call.clone());
        tx_result.pending_calls.async_call = Some(call);
        std::panic::panic_any(tx_result)
    }

    fn perform_upgrade_contract(
        &self,
        to: Address,
        egld_value: num_bigint::BigUint,
        contract_code: Vec<u8>,
        code_metadata: CodeMetadata,
        args: Vec<Vec<u8>>,
    ) -> ! {
        let contract_address = self.input_ref().to.clone();
        let tx_hash = self.get_tx_hash_legacy();

        let mut arguments = vec![contract_code, top_encode_to_vec_u8(&code_metadata).unwrap()];
        arguments.extend(args.into_iter());
        let call = AsyncCallTxData {
            from: contract_address,
            to,
            call_value: egld_value,
            endpoint_name: UPGRADE_CONTRACT_FUNC_NAME.into(),
            arguments,
            tx_hash,
        };
        self.perform_async_call(call)
    }

    fn get_contract_code(&self, address: &Address) -> Vec<u8> {
        self.blockchain_cache()
            .with_account(address, |account| account.contract_path.clone())
            .unwrap_or_else(|| panic!("Account is not a smart contract, it has no code"))
    }
}

impl SendApi for DebugApi {
    type SendApiImpl = DebugApi;

    fn send_api_impl() -> Self::SendApiImpl {
        DebugApi::new_from_static()
    }
}

impl SendApiImpl for DebugApi {
    fn transfer_value_execute(
        &self,
        to_handle: RawHandle,
        amount_handle: RawHandle,
        _gas_limit: u64,
        endpoint_name_handle: RawHandle,
        arg_buffer_handle: RawHandle,
    ) -> Result<(), &'static [u8]> {
        let recipient = self.m_types_borrow().mb_to_address(to_handle);
        let egld_value = self.m_types_borrow().bu_get(amount_handle);
        let endpoint_name = self
            .m_types_borrow()
            .mb_to_function_name(endpoint_name_handle);
        let arg_buffer = self.m_types_borrow().mb_get_vec_of_bytes(arg_buffer_handle);

        let _ = self.perform_transfer_execute(recipient, egld_value, endpoint_name, arg_buffer);

        Ok(())
    }

    fn multi_transfer_esdt_nft_execute(
        &self,
        to_handle: RawHandle,
        payments_handle: RawHandle,
        gas_limit: u64,
        endpoint_name_handle: RawHandle,
        arg_buffer_handle: RawHandle,
    ) -> Result<(), &'static [u8]> {
        let to = self.m_types_borrow().mb_to_address(to_handle);
        let payments = self
            .m_types_borrow()
            .mb_get_vec_of_esdt_payments(payments_handle);
        let endpoint_name = self
            .m_types_borrow()
            .mb_to_function_name(endpoint_name_handle);
        let arg_buffer = self.m_types_borrow().mb_get_vec_of_bytes(arg_buffer_handle);

        if payments.len() == 1 {
            let payment = payments[0].clone();
            if payment.nonce == 0 {
                self.perform_transfer_execute_esdt(
                    to,
                    payment.token_identifier,
                    payment.value,
                    gas_limit,
                    endpoint_name,
                    arg_buffer,
                )
            } else {
                self.perform_transfer_execute_nft(
                    to,
                    payment.token_identifier,
                    payment.nonce,
                    payment.value,
                    gas_limit,
                    endpoint_name,
                    arg_buffer,
                )
            }
        } else {
            self.perform_transfer_execute_multi(to, payments, gas_limit, endpoint_name, arg_buffer)
        }
    }

    fn async_call_raw(
        &self,
        to_handle: RawHandle,
        egld_value_handle: RawHandle,
        endpoint_name_handle: RawHandle,
        arg_buffer_handle: RawHandle,
    ) -> ! {
        let contract_address = self.input_ref().to.clone();
        let to = self.m_types_borrow().mb_to_address(to_handle);
        let egld_value = self.m_types_borrow().bu_get(egld_value_handle);
        let endpoint_name = self
            .m_types_borrow()
            .mb_to_function_name(endpoint_name_handle);
        let arg_buffer = self.m_types_borrow().mb_get_vec_of_bytes(arg_buffer_handle);
        let tx_hash = self.get_tx_hash_legacy();

        let call = AsyncCallTxData {
            from: contract_address,
            to,
            call_value: egld_value,
            endpoint_name,
            arguments: arg_buffer,
            tx_hash,
        };
        self.perform_async_call(call)
    }

    fn create_async_call_raw(
        &self,
        to_handle: RawHandle,
        egld_value_handle: RawHandle,
        endpoint_name_handle: RawHandle,
        arg_buffer_handle: RawHandle,
        success_callback: &'static str,
        error_callback: &'static str,
        _gas: u64,
        _extra_gas_for_callback: u64,
        callback_closure_handle: RawHandle,
    ) {
        let contract_address = self.input_ref().to.clone();
        let to = self.m_types_borrow().mb_to_address(to_handle);
        let egld_value = self.m_types_borrow().bu_get(egld_value_handle);
        let endpoint_name = self
            .m_types_borrow()
            .mb_to_function_name(endpoint_name_handle);
        let arg_buffer = self.m_types_borrow().mb_get_vec_of_bytes(arg_buffer_handle);
        let tx_hash = self.get_tx_hash_legacy();
        let callback_closure_data = self
            .m_types_borrow()
            .mb_get(callback_closure_handle)
            .to_vec();

        let call = AsyncCallTxData {
            from: contract_address,
            to,
            call_value: egld_value,
            endpoint_name,
            arguments: arg_buffer,
            tx_hash,
        };

        let promise = Promise {
            call,
            success_callback: success_callback.into(),
            error_callback: error_callback.into(),
            callback_closure_data,
        };

        let mut tx_result = self.result_borrow_mut();
        tx_result.all_calls.push(promise.call.clone());
        tx_result.pending_calls.promises.push(promise);
    }

    fn deploy_contract(
        &self,
        _gas: u64,
        egld_value_handle: RawHandle,
        code_handle: RawHandle,
        code_metadata_handle: RawHandle,
        arg_buffer_handle: RawHandle,
        new_address_handle: RawHandle,
        result_handle: RawHandle,
    ) {
        let egld_value = self.m_types_borrow().bu_get(egld_value_handle);
        let code = self.m_types_borrow().mb_get(code_handle).to_vec();
        let code_metadata = self
            .m_types_borrow()
            .mb_to_code_metadata(code_metadata_handle);
        let arg_buffer = self.m_types_borrow().mb_get_vec_of_bytes(arg_buffer_handle);

        let (new_address, result) =
            self.perform_deploy(egld_value, code, code_metadata, arg_buffer);

        self.m_types_borrow_mut()
            .mb_set(new_address_handle, new_address.to_vec());
        self.m_types_borrow_mut()
            .mb_set_vec_of_bytes(result_handle, result);
    }

    fn deploy_from_source_contract(
        &self,
        _gas: u64,
        egld_value_handle: RawHandle,
        source_contract_address_handle: RawHandle,
        code_metadata_handle: RawHandle,
        arg_buffer_handle: RawHandle,
        new_address_handle: RawHandle,
        result_handle: RawHandle,
    ) {
        let egld_value = self.m_types_borrow().bu_get(egld_value_handle);
        let source_contract_address = self
            .m_types_borrow()
            .mb_to_address(source_contract_address_handle);
        let source_contract_code = self.get_contract_code(&source_contract_address);
        let code_metadata = self
            .m_types_borrow()
            .mb_to_code_metadata(code_metadata_handle);
        let arg_buffer = self.m_types_borrow().mb_get_vec_of_bytes(arg_buffer_handle);

        let (new_address, result) =
            self.perform_deploy(egld_value, source_contract_code, code_metadata, arg_buffer);

        self.m_types_borrow_mut()
            .mb_set(new_address_handle, new_address.to_vec());
        self.m_types_borrow_mut()
            .mb_set_vec_of_bytes(result_handle, result);
    }

    fn upgrade_from_source_contract(
        &self,
        sc_address_handle: RawHandle,
        _gas: u64,
        egld_value_handle: RawHandle,
        source_contract_address_handle: RawHandle,
        code_metadata_handle: RawHandle,
        arg_buffer_handle: RawHandle,
    ) {
        let to = self.m_types_borrow().mb_to_address(sc_address_handle);
        let egld_value = self.m_types_borrow().bu_get(egld_value_handle);
        let source_contract_address = self
            .m_types_borrow()
            .mb_to_address(source_contract_address_handle);
        let source_contract_code = self.get_contract_code(&source_contract_address);
        let code_metadata = self
            .m_types_borrow()
            .mb_to_code_metadata(code_metadata_handle);
        let arg_buffer = self.m_types_borrow().mb_get_vec_of_bytes(arg_buffer_handle);

        self.perform_upgrade_contract(
            to,
            egld_value,
            source_contract_code,
            code_metadata,
            arg_buffer,
        )
    }

    fn upgrade_contract(
        &self,
        sc_address_handle: RawHandle,
        _gas: u64,
        egld_value_handle: RawHandle,
        code_handle: RawHandle,
        code_metadata_handle: RawHandle,
        arg_buffer_handle: RawHandle,
    ) {
        let to = self.m_types_borrow().mb_to_address(sc_address_handle);
        let egld_value = self.m_types_borrow().bu_get(egld_value_handle);
        let code = self.m_types_borrow().mb_get(code_handle).to_vec();
        let code_metadata = self
            .m_types_borrow()
            .mb_to_code_metadata(code_metadata_handle);
        let arg_buffer = self.m_types_borrow().mb_get_vec_of_bytes(arg_buffer_handle);

        self.perform_upgrade_contract(to, egld_value, code, code_metadata, arg_buffer)
    }

    fn execute_on_dest_context_raw(
        &self,
        _gas: u64,
        to_handle: RawHandle,
        egld_value_handle: RawHandle,
        endpoint_name_handle: RawHandle,
        arg_buffer_handle: RawHandle,
        result_handle: RawHandle,
    ) {
        let to = self.m_types_borrow().mb_to_address(to_handle);
        let egld_value = self.m_types_borrow().bu_get(egld_value_handle);
        let endpoint_name = self
            .m_types_borrow()
            .mb_to_function_name(endpoint_name_handle);
        let arg_buffer = self.m_types_borrow().mb_get_vec_of_bytes(arg_buffer_handle);

        let result =
            self.perform_execute_on_dest_context(to, egld_value, endpoint_name, arg_buffer);

        self.m_types_borrow_mut()
            .mb_set_vec_of_bytes(result_handle, result);
    }

    fn execute_on_same_context_raw(
        &self,
        _gas: u64,
        _to_handle: RawHandle,
        _egld_value_handle: RawHandle,
        _endpoint_name_handle: RawHandle,
        _arg_buffer_handle: RawHandle,
        _result_handle: RawHandle,
    ) {
        panic!("execute_on_same_context_raw not implemented yet!");
    }

    fn execute_on_dest_context_readonly_raw(
        &self,
        _gas: u64,
        _to_handle: RawHandle,
        _endpoint_name_handle: RawHandle,
        _arg_buffer_handle: RawHandle,
        _result_handle: RawHandle,
    ) {
        panic!("execute_on_dest_context_readonly_raw not implemented yet!");
    }

    fn clean_return_data(&self) {
        let mut tx_result = self.result_borrow_mut();
        tx_result.result_values.clear();
    }

    fn delete_from_return_data(&self, index: usize) {
        let mut tx_result = self.result_borrow_mut();
        if index > tx_result.result_values.len() {
            return;
        }

        let _ = tx_result.result_values.remove(index);
    }
}
