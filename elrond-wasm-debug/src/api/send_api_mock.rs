use crate::{
    tx_execution::{deploy_contract, execute_builtin_function_or_default},
    tx_mock::{AsyncCallTxData, BlockchainUpdate, TxCache, TxInput, TxPanic, TxResult},
    DebugApi,
};
use elrond_wasm::{
    api::{
        BlockchainApi, SendApi, StorageReadApi, StorageWriteApi, ESDT_MULTI_TRANSFER_FUNC_NAME,
        ESDT_NFT_TRANSFER_FUNC_NAME, ESDT_TRANSFER_FUNC_NAME, UPGRADE_CONTRACT_FUNC_NAME,
    },
    elrond_codec::top_encode_to_vec_u8,
    types::{
        Address, BigUint, CodeMetadata, EsdtTokenPayment, ManagedAddress, ManagedArgBuffer,
        ManagedBuffer, ManagedVec, TokenIdentifier,
    },
};
use num_traits::Zero;

impl DebugApi {
    fn append_endpoint_name_and_args(
        args: &mut Vec<Vec<u8>>,
        endpoint_name: &ManagedBuffer<Self>,
        arg_buffer: &ManagedArgBuffer<Self>,
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

    fn perform_execute_on_dest_context(
        &self,
        to: Address,
        egld_value: num_bigint::BigUint,
        func_name: Vec<u8>,
        args: Vec<Vec<u8>>,
    ) -> Vec<Vec<u8>> {
        let contract_address = &self.input_ref().to;
        let tx_hash = self.get_tx_hash_legacy();
        let tx_input = TxInput {
            from: contract_address.clone(),
            to,
            egld_value,
            esdt_values: Vec::new(),
            func_name,
            args,
            gas_limit: 1000,
            gas_price: 0,
            tx_hash,
        };

        let tx_cache = TxCache::new(self.blockchain_cache_rc());
        let (tx_result, blockchain_updates) =
            execute_builtin_function_or_default(tx_input, tx_cache);

        if tx_result.result_status == 0 {
            self.sync_call_post_processing(tx_result, blockchain_updates)
        } else {
            // also kill current execution
            std::panic::panic_any(TxPanic {
                status: tx_result.result_status,
                message: tx_result.result_message.into_bytes(),
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
            func_name: Vec::new(),
            args,
            gas_limit: 1000,
            gas_price: 0,
            tx_hash,
        };

        let tx_cache = TxCache::new(self.blockchain_cache_rc());
        tx_cache.increase_acount_nonce(contract_address);
        let (tx_result, blockchain_updates, new_address) =
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
                message: b"error signalled by smartcontract".to_vec(),
            })
        }
    }

    fn perform_async_call(&self, call: AsyncCallTxData) -> ! {
        // the cell is no longer needed, since we end in a panic
        let mut tx_result = self.extract_result();
        tx_result.result_calls.async_call = Some(call);
        std::panic::panic_any(tx_result)
    }

    fn perform_upgrade_contract(
        &self,
        sc_address: &ManagedAddress<Self>,
        amount: &BigUint<Self>,
        contract_code: Vec<u8>,
        code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<Self>,
    ) -> ! {
        let recipient = sc_address.to_address();
        let call_value = self.big_uint_value(amount);
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
            endpoint_name: UPGRADE_CONTRACT_FUNC_NAME.to_vec(),
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
    fn direct_egld<D>(&self, to: &ManagedAddress<Self>, amount: &BigUint<Self>, _data: D)
    where
        D: Into<ManagedBuffer<Self>>,
    {
        let amount_value = self.big_uint_value(amount);
        let available_egld_balance =
            self.with_contract_account(|account| account.egld_balance.clone());
        if amount_value > available_egld_balance {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: b"failed transfer (insufficient funds)".to_vec(),
            });
        }

        let contract_address = &self.input_ref().to;
        self.blockchain_cache()
            .subtract_egld_balance(contract_address, &amount_value);

        let recipient = &to.to_address();
        self.blockchain_cache()
            .increase_egld_balance(recipient, &amount_value);
    }

    fn direct_egld_execute(
        &self,
        to: &ManagedAddress<Self>,
        amount: &BigUint<Self>,
        _gas_limit: u64,
        endpoint_name: &ManagedBuffer<Self>,
        arg_buffer: &ManagedArgBuffer<Self>,
    ) -> Result<(), &'static [u8]> {
        let egld_value = self.big_uint_value(amount);
        let recipient = to.to_address();

        let _ = self.perform_execute_on_dest_context(
            recipient,
            egld_value,
            endpoint_name.to_boxed_bytes().into_vec(),
            arg_buffer.to_raw_args_vec(),
        );

        Ok(())
    }

    fn direct_esdt_execute(
        &self,
        to: &ManagedAddress<Self>,
        token: &TokenIdentifier<Self>,
        amount: &BigUint<Self>,
        _gas_limit: u64,
        endpoint_name: &ManagedBuffer<Self>,
        arg_buffer: &ManagedArgBuffer<Self>,
    ) -> Result<(), &'static [u8]> {
        let recipient = to.to_address();
        let token_bytes = top_encode_to_vec_u8(token).unwrap();
        let amount_bytes = top_encode_to_vec_u8(amount).unwrap();

        let mut args = vec![token_bytes, amount_bytes];
        Self::append_endpoint_name_and_args(&mut args, endpoint_name, arg_buffer);

        let _ = self.perform_execute_on_dest_context(
            recipient,
            num_bigint::BigUint::zero(),
            ESDT_TRANSFER_FUNC_NAME.to_vec(),
            args,
        );

        Ok(())
    }

    fn direct_esdt_nft_execute(
        &self,
        to: &ManagedAddress<Self>,
        token: &TokenIdentifier<Self>,
        nonce: u64,
        amount: &BigUint<Self>,
        _gas_limit: u64,
        endpoint_name: &ManagedBuffer<Self>,
        arg_buffer: &ManagedArgBuffer<Self>,
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

        let _ = self.perform_execute_on_dest_context(
            contract_address,
            num_bigint::BigUint::zero(),
            ESDT_NFT_TRANSFER_FUNC_NAME.to_vec(),
            args,
        );

        Ok(())
    }

    fn direct_multi_esdt_transfer_execute(
        &self,
        to: &ManagedAddress<Self>,
        payments: &ManagedVec<Self, EsdtTokenPayment<Self>>,
        _gas_limit: u64,
        endpoint_name: &ManagedBuffer<Self>,
        arg_buffer: &ManagedArgBuffer<Self>,
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

        let _ = self.perform_execute_on_dest_context(
            contract_address,
            num_bigint::BigUint::zero(),
            ESDT_MULTI_TRANSFER_FUNC_NAME.to_vec(),
            args,
        );

        Ok(())
    }

    fn async_call_raw(
        &self,
        to: &ManagedAddress<Self>,
        amount: &BigUint<Self>,
        endpoint_name: &ManagedBuffer<Self>,
        arg_buffer: &ManagedArgBuffer<Self>,
    ) -> ! {
        let amount_value = self.big_uint_value(amount);
        let contract_address = self.input_ref().to.clone();
        let recipient = to.to_address();
        let tx_hash = self.get_tx_hash_legacy();
        let call = AsyncCallTxData {
            from: contract_address,
            to: recipient,
            call_value: amount_value,
            endpoint_name: endpoint_name.to_boxed_bytes().into_vec(),
            arguments: arg_buffer.to_raw_args_vec(),
            tx_hash,
        };
        self.perform_async_call(call)
    }

    fn deploy_contract(
        &self,
        _gas: u64,
        amount: &BigUint<Self>,
        code: &ManagedBuffer<Self>,
        _code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<Self>,
    ) -> (ManagedAddress<Self>, ManagedVec<Self, ManagedBuffer<Self>>) {
        let egld_value = self.big_uint_value(amount);
        let contract_code = code.to_boxed_bytes().into_vec();
        let (new_address, result) =
            self.perform_deploy(contract_code, egld_value, arg_buffer.to_raw_args_vec());

        (ManagedAddress::from(new_address), ManagedVec::from(result))
    }

    fn deploy_from_source_contract(
        &self,
        _gas: u64,
        amount: &BigUint<Self>,
        source_contract_address: &ManagedAddress<Self>,
        _code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<Self>,
    ) -> (ManagedAddress<Self>, ManagedVec<Self, ManagedBuffer<Self>>) {
        let egld_value = self.big_uint_value(amount);
        let source_contract_code = self.get_contract_code(&source_contract_address.to_address());
        let (new_address, result) = self.perform_deploy(
            source_contract_code,
            egld_value,
            arg_buffer.to_raw_args_vec(),
        );

        (ManagedAddress::from(new_address), ManagedVec::from(result))
    }

    fn upgrade_contract(
        &self,
        sc_address: &ManagedAddress<Self>,
        _gas: u64,
        amount: &BigUint<Self>,
        code: &ManagedBuffer<Self>,
        code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<Self>,
    ) {
        let contract_code = code.to_boxed_bytes().into_vec();
        self.perform_upgrade_contract(sc_address, amount, contract_code, code_metadata, arg_buffer)
    }

    fn upgrade_from_source_contract(
        &self,
        sc_address: &ManagedAddress<Self>,
        _gas: u64,
        amount: &BigUint<Self>,
        source_contract_address: &ManagedAddress<Self>,
        code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<Self>,
    ) {
        let contract_code = self.get_contract_code(&source_contract_address.to_address());
        self.perform_upgrade_contract(sc_address, amount, contract_code, code_metadata, arg_buffer)
    }

    fn execute_on_dest_context_raw(
        &self,
        _gas: u64,
        to: &ManagedAddress<Self>,
        value: &BigUint<Self>,
        endpoint_name: &ManagedBuffer<Self>,
        arg_buffer: &ManagedArgBuffer<Self>,
    ) -> ManagedVec<Self, ManagedBuffer<Self>> {
        let egld_value = self.big_uint_value(value);
        let recipient = to.to_address();

        let result = self.perform_execute_on_dest_context(
            recipient,
            egld_value,
            endpoint_name.to_boxed_bytes().into_vec(),
            arg_buffer.to_raw_args_vec(),
        );

        ManagedVec::from(result)
    }

    fn execute_on_dest_context_raw_custom_result_range<F>(
        &self,
        _gas: u64,
        to: &ManagedAddress<Self>,
        value: &BigUint<Self>,
        endpoint_name: &ManagedBuffer<Self>,
        arg_buffer: &ManagedArgBuffer<Self>,
        range_closure: F,
    ) -> ManagedVec<Self, ManagedBuffer<Self>>
    where
        F: FnOnce(usize, usize) -> (usize, usize),
    {
        let egld_value = self.big_uint_value(value);
        let recipient = to.to_address();

        let num_return_data_before = self.result_borrow_mut().result_values.len();

        let result = self.perform_execute_on_dest_context(
            recipient,
            egld_value,
            endpoint_name.to_boxed_bytes().into_vec(),
            arg_buffer.to_raw_args_vec(),
        );

        let num_return_data_after = result.len();
        let (result_start_index, result_end_index) = range_closure(
            num_return_data_before as usize,
            num_return_data_after as usize,
        );

        ManagedVec::from(result[result_start_index..result_end_index].to_vec())
    }

    fn execute_on_dest_context_by_caller_raw(
        &self,
        _gas: u64,
        _to: &ManagedAddress<Self>,
        _value: &BigUint<Self>,
        _endpoint_name: &ManagedBuffer<Self>,
        _arg_buffer: &ManagedArgBuffer<Self>,
    ) -> ManagedVec<Self, ManagedBuffer<Self>> {
        panic!("execute_on_dest_context_by_caller_raw not implemented yet!");
    }

    fn execute_on_same_context_raw(
        &self,
        _gas: u64,
        _to: &ManagedAddress<Self>,
        _value: &BigUint<Self>,
        _endpoint_name: &ManagedBuffer<Self>,
        _arg_buffer: &ManagedArgBuffer<Self>,
    ) -> ManagedVec<Self, ManagedBuffer<Self>> {
        panic!("execute_on_same_context_raw not implemented yet!");
    }

    fn execute_on_dest_context_readonly_raw(
        &self,
        _gas: u64,
        _to: &ManagedAddress<Self>,
        _endpoint_name: &ManagedBuffer<Self>,
        _arg_buffer: &ManagedArgBuffer<Self>,
    ) -> ManagedVec<Self, ManagedBuffer<Self>> {
        panic!("execute_on_dest_context_readonly_raw not implemented yet!");
    }

    fn storage_store_tx_hash_key(&self, data: &ManagedBuffer<Self>) {
        let tx_hash = self.get_tx_hash_legacy();
        self.storage_store_slice_u8(tx_hash.as_bytes(), data.to_boxed_bytes().as_slice());
    }

    fn storage_load_tx_hash_key(&self) -> ManagedBuffer<Self> {
        let tx_hash = self.get_tx_hash_legacy();
        let bytes = self.storage_load_boxed_bytes(tx_hash.as_bytes());
        ManagedBuffer::new_from_bytes(bytes.as_slice())
    }

    fn call_local_esdt_built_in_function(
        &self,
        _gas: u64,
        function_name: &ManagedBuffer<Self>,
        arg_buffer: &ManagedArgBuffer<Self>,
    ) -> ManagedVec<Self, ManagedBuffer<Self>> {
        let contract_address = &self.input_ref().to;

        let result = self.perform_execute_on_dest_context(
            contract_address.clone(),
            num_bigint::BigUint::zero(),
            function_name.to_boxed_bytes().into_vec(),
            arg_buffer.to_raw_args_vec(),
        );

        ManagedVec::from(result)
    }
}
