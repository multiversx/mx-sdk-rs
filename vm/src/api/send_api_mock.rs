use crate::{
    num_bigint,
    tx_execution::{deploy_contract, execute_builtin_function_or_default},
    tx_mock::{
        async_call_tx_input, AsyncCallTxData, BlockchainUpdate, Promise, TxCache, TxFunctionName,
        TxInput, TxPanic, TxResult,
    },
    DebugApi,
};
use multiversx_sc::{
    api::{
        BlockchainApiImpl, HandleConstraints, ManagedTypeApi, SendApi, SendApiImpl,
        ESDT_MULTI_TRANSFER_FUNC_NAME, ESDT_NFT_TRANSFER_FUNC_NAME, ESDT_TRANSFER_FUNC_NAME,
        UPGRADE_CONTRACT_FUNC_NAME,
    },
    codec::top_encode_to_vec_u8,
    err_msg,
    types::{
        heap::Address, ArgBuffer, BigUint, BoxedBytes, CodeMetadata, EsdtTokenPayment,
        ManagedAddress, ManagedArgBuffer, ManagedBuffer, ManagedType, ManagedVec, TokenIdentifier,
    },
};
use num_traits::Zero;

impl DebugApi {
    fn append_endpoint_name_and_args<M: ManagedTypeApi>(
        args: &mut Vec<Vec<u8>>,
        endpoint_name: &ManagedBuffer<M>,
        arg_buffer: &ManagedArgBuffer<M>,
    ) {
        if !endpoint_name.is_empty() {
            args.push(endpoint_name.to_boxed_bytes().into_vec());
            args.extend(
                arg_buffer
                    .raw_arg_iter()
                    .map(|mb| mb.to_boxed_bytes().into_vec()),
            );
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

    fn perform_deploy(
        &self,
        contract_code: Vec<u8>,
        egld_value: num_bigint::BigUint,
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

    fn perform_upgrade_contract<M: ManagedTypeApi>(
        &self,
        sc_address: &ManagedAddress<M>,
        amount: &BigUint<M>,
        contract_code: Vec<u8>,
        code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<M>,
    ) -> ! {
        let recipient = sc_address.to_address();
        let call_value =
            self.big_uint_handle_to_value(amount.get_handle().cast_or_signal_error::<M, _>());
        let contract_address = self.input_ref().to.clone();
        let tx_hash = self.get_tx_hash_legacy();

        let mut arguments = vec![contract_code, top_encode_to_vec_u8(&code_metadata).unwrap()];
        arguments.extend(
            arg_buffer
                .raw_arg_iter()
                .map(|mb| mb.to_boxed_bytes().into_vec()),
        );
        let call = AsyncCallTxData {
            from: contract_address,
            to: recipient,
            call_value,
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
    fn transfer_value_execute<M: ManagedTypeApi>(
        &self,
        to: &ManagedAddress<M>,
        amount: &BigUint<M>,
        _gas_limit: u64,
        endpoint_name: &ManagedBuffer<M>,
        arg_buffer: &ManagedArgBuffer<M>,
    ) -> Result<(), &'static [u8]> {
        let egld_value =
            self.big_uint_handle_to_value(amount.get_handle().cast_or_signal_error::<M, _>());
        let recipient = to.to_address();

        let _ = self.perform_transfer_execute(
            recipient,
            egld_value,
            endpoint_name.to_boxed_bytes().as_slice().into(),
            arg_buffer.to_raw_args_vec(),
        );

        Ok(())
    }

    fn transfer_esdt_execute<M: ManagedTypeApi>(
        &self,
        to: &ManagedAddress<M>,
        token: &TokenIdentifier<M>,
        amount: &BigUint<M>,
        _gas_limit: u64,
        endpoint_name: &ManagedBuffer<M>,
        arg_buffer: &ManagedArgBuffer<M>,
    ) -> Result<(), &'static [u8]> {
        let recipient = to.to_address();
        let token_bytes = top_encode_to_vec_u8(token).unwrap();
        let amount_bytes = top_encode_to_vec_u8(amount).unwrap();

        let mut args = vec![token_bytes, amount_bytes];
        Self::append_endpoint_name_and_args(&mut args, endpoint_name, arg_buffer);

        let _ = self.perform_transfer_execute(
            recipient,
            num_bigint::BigUint::zero(),
            ESDT_TRANSFER_FUNC_NAME.into(),
            args,
        );

        Ok(())
    }

    fn transfer_esdt_nft_execute<M: ManagedTypeApi>(
        &self,
        to: &ManagedAddress<M>,
        token: &TokenIdentifier<M>,
        nonce: u64,
        amount: &BigUint<M>,
        _gas_limit: u64,
        endpoint_name: &ManagedBuffer<M>,
        arg_buffer: &ManagedArgBuffer<M>,
    ) -> Result<(), &'static [u8]> {
        let contract_address = self.input_ref().to.clone();
        let recipient = to.to_address();

        let token_bytes = top_encode_to_vec_u8(token).unwrap();
        let nonce_bytes = top_encode_to_vec_u8(&nonce).unwrap();
        let amount_bytes = top_encode_to_vec_u8(amount).unwrap();

        let mut args = vec![
            token_bytes,
            nonce_bytes,
            amount_bytes,
            recipient.as_bytes().to_vec(),
        ];

        Self::append_endpoint_name_and_args(&mut args, endpoint_name, arg_buffer);

        let _ = self.perform_transfer_execute(
            contract_address,
            num_bigint::BigUint::zero(),
            ESDT_NFT_TRANSFER_FUNC_NAME.into(),
            args,
        );

        Ok(())
    }

    fn multi_transfer_esdt_nft_execute<M: ManagedTypeApi>(
        &self,
        to: &ManagedAddress<M>,
        payments: &ManagedVec<M, EsdtTokenPayment<M>>,
        _gas_limit: u64,
        endpoint_name: &ManagedBuffer<M>,
        arg_buffer: &ManagedArgBuffer<M>,
    ) -> Result<(), &'static [u8]> {
        let contract_address = self.input_ref().to.clone();
        let recipient = to.to_address();

        let mut args = vec![
            recipient.as_bytes().to_vec(),
            top_encode_to_vec_u8(&payments.len()).unwrap(),
        ];

        for payment in payments.into_iter() {
            let token_bytes = top_encode_to_vec_u8(&payment.token_identifier).unwrap();
            args.push(token_bytes);
            let nonce_bytes = top_encode_to_vec_u8(&payment.token_nonce).unwrap();
            args.push(nonce_bytes);
            let amount_bytes = top_encode_to_vec_u8(&payment.amount).unwrap();
            args.push(amount_bytes);
        }

        if !endpoint_name.is_empty() {
            args.push(endpoint_name.to_boxed_bytes().into_vec());
            args.extend(
                arg_buffer
                    .raw_arg_iter()
                    .map(|mb| mb.to_boxed_bytes().into_vec()),
            );
        }

        let _ = self.perform_transfer_execute(
            contract_address,
            num_bigint::BigUint::zero(),
            ESDT_MULTI_TRANSFER_FUNC_NAME.into(),
            args,
        );

        Ok(())
    }

    fn async_call_raw<M: ManagedTypeApi>(
        &self,
        to: &ManagedAddress<M>,
        amount: &BigUint<M>,
        endpoint_name: &ManagedBuffer<M>,
        arg_buffer: &ManagedArgBuffer<M>,
    ) -> ! {
        let amount_value =
            self.big_uint_handle_to_value(amount.get_handle().cast_or_signal_error::<M, _>());
        let contract_address = self.input_ref().to.clone();
        let recipient = to.to_address();
        let tx_hash = self.get_tx_hash_legacy();
        let call = AsyncCallTxData {
            from: contract_address,
            to: recipient,
            call_value: amount_value,
            endpoint_name: endpoint_name.to_boxed_bytes().as_slice().into(),
            arguments: arg_buffer.to_raw_args_vec(),
            tx_hash,
        };
        self.perform_async_call(call)
    }

    fn create_async_call_raw(
        &self,
        to: Self::ManagedBufferHandle,
        amount: Self::BigIntHandle,
        endpoint_name_handle: Self::ManagedBufferHandle,
        arg_buffer_handle: Self::ManagedBufferHandle,
        success_callback: &'static str,
        error_callback: &'static str,
        _gas: u64,
        _extra_gas_for_callback: u64,
        callback_closure_handle: Self::ManagedBufferHandle,
    ) {
        let amount_value = self.big_uint_handle_to_value(amount);
        let contract_address = self.input_ref().to.clone();
        let recipient = self.address_handle_to_value(to);
        let endpoint_name = self.mb_handle_to_value(endpoint_name_handle);
        let tx_hash = self.get_tx_hash_legacy();
        let callback_closure_data = self.mb_handle_to_value(callback_closure_handle);

        let call = AsyncCallTxData {
            from: contract_address,
            to: recipient,
            call_value: amount_value,
            endpoint_name: endpoint_name.into(),
            arguments: ManagedArgBuffer::<Self>::from_raw_handle(
                arg_buffer_handle.get_raw_handle_unchecked(),
            )
            .to_raw_args_vec(),
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

    fn deploy_contract<M: ManagedTypeApi>(
        &self,
        _gas: u64,
        amount: &BigUint<M>,
        code: &ManagedBuffer<M>,
        _code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<M>,
    ) -> (ManagedAddress<M>, ManagedVec<M, ManagedBuffer<M>>) {
        let egld_value =
            self.big_uint_handle_to_value(amount.get_handle().cast_or_signal_error::<M, _>());
        let contract_code = code.to_boxed_bytes().into_vec();
        let (new_address, result) =
            self.perform_deploy(contract_code, egld_value, arg_buffer.to_raw_args_vec());

        (ManagedAddress::from(new_address), ManagedVec::from(result))
    }

    fn deploy_from_source_contract<M: ManagedTypeApi>(
        &self,
        _gas: u64,
        amount: &BigUint<M>,
        source_contract_address: &ManagedAddress<M>,
        _code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<M>,
    ) -> (ManagedAddress<M>, ManagedVec<M, ManagedBuffer<M>>) {
        let egld_value =
            self.big_uint_handle_to_value(amount.get_handle().cast_or_signal_error::<M, _>());
        let source_contract_code = self.get_contract_code(&source_contract_address.to_address());
        let (new_address, result) = self.perform_deploy(
            source_contract_code,
            egld_value,
            arg_buffer.to_raw_args_vec(),
        );

        (ManagedAddress::from(new_address), ManagedVec::from(result))
    }

    fn upgrade_contract<M: ManagedTypeApi>(
        &self,
        sc_address: &ManagedAddress<M>,
        _gas: u64,
        amount: &BigUint<M>,
        code: &ManagedBuffer<M>,
        code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<M>,
    ) {
        let contract_code = code.to_boxed_bytes().into_vec();
        self.perform_upgrade_contract(sc_address, amount, contract_code, code_metadata, arg_buffer)
    }

    fn upgrade_from_source_contract<M: ManagedTypeApi>(
        &self,
        sc_address: &ManagedAddress<M>,
        _gas: u64,
        amount: &BigUint<M>,
        source_contract_address: &ManagedAddress<M>,
        code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<M>,
    ) {
        let contract_code = self.get_contract_code(&source_contract_address.to_address());
        self.perform_upgrade_contract(sc_address, amount, contract_code, code_metadata, arg_buffer)
    }

    fn execute_on_dest_context_raw<M: ManagedTypeApi>(
        &self,
        _gas: u64,
        to: &ManagedAddress<M>,
        value: &BigUint<M>,
        endpoint_name: &ManagedBuffer<M>,
        arg_buffer: &ManagedArgBuffer<M>,
    ) -> ManagedVec<M, ManagedBuffer<M>> {
        let egld_value =
            self.big_uint_handle_to_value(value.get_handle().cast_or_signal_error::<M, _>());
        let recipient = to.to_address();

        let result = self.perform_execute_on_dest_context(
            recipient,
            egld_value,
            endpoint_name.to_boxed_bytes().as_slice().into(),
            arg_buffer.to_raw_args_vec(),
        );

        ManagedVec::from(result)
    }

    fn execute_on_same_context_raw<M: ManagedTypeApi>(
        &self,
        _gas: u64,
        _to: &ManagedAddress<M>,
        _value: &BigUint<M>,
        _endpoint_name: &ManagedBuffer<M>,
        _arg_buffer: &ManagedArgBuffer<M>,
    ) -> ManagedVec<M, ManagedBuffer<M>> {
        panic!("execute_on_same_context_raw not implemented yet!");
    }

    fn execute_on_dest_context_readonly_raw<M: ManagedTypeApi>(
        &self,
        _gas: u64,
        _to: &ManagedAddress<M>,
        _endpoint_name: &ManagedBuffer<M>,
        _arg_buffer: &ManagedArgBuffer<M>,
    ) -> ManagedVec<M, ManagedBuffer<M>> {
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

    fn transfer_value_legacy<M>(&self, _to: &Address, _amount: &BigUint<M>, _data: &BoxedBytes)
    where
        M: ManagedTypeApi,
    {
        panic!("legacy operation not implemented");
    }

    fn transfer_value_execute_legacy<M: ManagedTypeApi>(
        &self,
        _to: &Address,
        _amount: &BigUint<M>,
        _gas_limit: u64,
        _endpoint_name: &BoxedBytes,
        _arg_buffer: &ArgBuffer,
    ) -> Result<(), &'static [u8]> {
        panic!("legacy operation not implemented");
    }

    fn transfer_esdt_execute_legacy<M: ManagedTypeApi>(
        &self,
        _to: &Address,
        _token: &TokenIdentifier<M>,
        _amount: &BigUint<M>,
        _gas: u64,
        _endpoint_name: &BoxedBytes,
        _arg_buffer: &ArgBuffer,
    ) -> Result<(), &'static [u8]> {
        panic!("legacy operation not implemented");
    }

    fn transfer_esdt_nft_execute_legacy<M: ManagedTypeApi>(
        &self,
        _to: &Address,
        _token: &TokenIdentifier<M>,
        _nonce: u64,
        _amount: &BigUint<M>,
        _gas_limit: u64,
        _endpoint_name: &BoxedBytes,
        _arg_buffer: &ArgBuffer,
    ) -> Result<(), &'static [u8]> {
        panic!("legacy operation not implemented");
    }

    fn multi_transfer_esdt_nft_execute_legacy<M: ManagedTypeApi>(
        &self,
        _to: &Address,
        _payments: &[EsdtTokenPayment<M>],
        _gas_limit: u64,
        _endpoint_name: &BoxedBytes,
        _arg_buffer: &ArgBuffer,
    ) -> Result<(), &'static [u8]> {
        panic!("legacy operation not implemented");
    }

    fn async_call_raw_legacy<M: ManagedTypeApi>(
        &self,
        _to: &Address,
        _amount: &BigUint<M>,
        _endpoint_name: &BoxedBytes,
        _arg_buffer: &ArgBuffer,
    ) -> ! {
        panic!("legacy operation not implemented");
    }

    fn deploy_contract_legacy<M: ManagedTypeApi>(
        &self,
        _gas: u64,
        _amount: &BigUint<M>,
        _code: &BoxedBytes,
        _code_metadata: CodeMetadata,
        _arg_buffer: &ArgBuffer,
    ) -> (ManagedAddress<M>, ManagedVec<M, ManagedBuffer<M>>) {
        panic!("legacy operation not implemented");
    }

    fn deploy_from_source_contract_legacy<M: ManagedTypeApi>(
        &self,
        _gas: u64,
        _amount: &BigUint<M>,
        _source_contract_address: &Address,
        _code_metadata: CodeMetadata,
        _arg_buffer: &ArgBuffer,
    ) -> (ManagedAddress<M>, ManagedVec<M, ManagedBuffer<M>>) {
        panic!("legacy operation not implemented");
    }

    fn upgrade_from_source_contract_legacy<M: ManagedTypeApi>(
        &self,
        _sc_address: &Address,
        _gas: u64,
        _amount: &BigUint<M>,
        _source_contract_address: &Address,
        _code_metadata: CodeMetadata,
        _arg_buffer: &ArgBuffer,
    ) {
        panic!("legacy operation not implemented");
    }

    fn upgrade_contract_legacy<M: ManagedTypeApi>(
        &self,
        _sc_address: &Address,
        _gas: u64,
        _amount: &BigUint<M>,
        _code: &BoxedBytes,
        _code_metadata: CodeMetadata,
        _arg_buffer: &ArgBuffer,
    ) {
        panic!("legacy operation not implemented");
    }

    fn execute_on_dest_context_raw_legacy<M: ManagedTypeApi>(
        &self,
        _gas: u64,
        _to: &Address,
        _value: &BigUint<M>,
        _endpoint_name: &BoxedBytes,
        _arg_buffer: &ArgBuffer,
    ) -> ManagedVec<M, ManagedBuffer<M>> {
        panic!("legacy operation not implemented");
    }

    fn execute_on_same_context_raw_legacy<M: ManagedTypeApi>(
        &self,
        _gas: u64,
        _to: &Address,
        _value: &BigUint<M>,
        _endpoint_name: &BoxedBytes,
        _arg_buffer: &ArgBuffer,
    ) -> ManagedVec<M, ManagedBuffer<M>> {
        panic!("legacy operation not implemented");
    }

    fn execute_on_dest_context_readonly_raw_legacy<M: ManagedTypeApi>(
        &self,
        _gas: u64,
        _address: &Address,
        _endpoint_name: &BoxedBytes,
        _arg_buffer: &ArgBuffer,
    ) -> ManagedVec<M, ManagedBuffer<M>> {
        panic!("legacy operation not implemented");
    }
}
