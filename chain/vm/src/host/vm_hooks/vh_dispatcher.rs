use multiversx_chain_vm_executor::{MemLength, MemPtr, VMHooks, VMHooksError};

use super::VMHooksHandler;

const RESULT_TRUE: i32 = 1;
const RESULT_FALSE: i32 = 0;
const RESULT_OK: i32 = 0;
const RESULT_ERROR: i32 = 1;

/// Dispatches messages coming via VMHooks to the underlying implementation (the VMHooksHandler).
#[derive(Debug)]
pub struct VMHooksDispatcher<H: VMHooksHandler> {
    pub(crate) handler: H,
}

impl<H: VMHooksHandler> VMHooksDispatcher<H> {
    pub fn new(handler: H) -> Self {
        VMHooksDispatcher { handler }
    }
}

fn bool_to_i32(b: bool) -> Result<i32, VMHooksError> {
    Ok(if b { RESULT_TRUE } else { RESULT_FALSE })
}

#[allow(unused_variables)]
impl<H: VMHooksHandler> VMHooks for VMHooksDispatcher<H> {
    fn get_gas_left(&mut self) -> Result<i64, VMHooksError> {
        Ok(self.handler.get_gas_left() as i64)
    }

    fn get_sc_address(&mut self, result_offset: MemPtr) -> Result<(), VMHooksError> {
        panic!("Unavailable: get_sc_address");
    }

    fn get_owner_address(&mut self, result_offset: MemPtr) -> Result<(), VMHooksError> {
        panic!("Unavailable: get_owner_address");
    }

    fn get_shard_of_address(&mut self, address_offset: MemPtr) -> Result<i32, VMHooksError> {
        unsafe {
            let address_bytes = self.handler.memory_load(address_offset, 32);
            Ok(self.handler.get_shard_of_address(&address_bytes))
        }
    }

    fn is_smart_contract(&mut self, address_offset: MemPtr) -> Result<i32, VMHooksError> {
        unsafe {
            let address_bytes = self.handler.memory_load(address_offset, 32);
            bool_to_i32(self.handler.is_smart_contract(&address_bytes))
        }
    }

    fn signal_error(
        &mut self,
        message_offset: MemPtr,
        message_length: MemLength,
    ) -> Result<(), VMHooksError> {
        unsafe {
            let message = self.handler.memory_load(message_offset, message_length);
            self.handler.signal_error(&message)
        }
    }

    fn get_external_balance(
        &mut self,
        address_offset: MemPtr,
        result_offset: MemPtr,
    ) -> Result<(), VMHooksError> {
        panic!("Unavailable: get_external_balance");
    }

    fn get_block_hash(&mut self, nonce: i64, result_offset: MemPtr) -> Result<i32, VMHooksError> {
        panic!("Unavailable: get_block_hash")
    }

    fn get_esdt_balance(
        &mut self,
        address_offset: MemPtr,
        token_id_offset: MemPtr,
        token_id_len: MemLength,
        nonce: i64,
        result_offset: MemPtr,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: get_esdt_balance")
    }

    fn get_esdt_nft_name_length(
        &mut self,
        address_offset: MemPtr,
        token_id_offset: MemPtr,
        token_id_len: MemLength,
        nonce: i64,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: get_esdt_nft_name_length")
    }

    fn get_esdt_nft_attribute_length(
        &mut self,
        address_offset: MemPtr,
        token_id_offset: MemPtr,
        token_id_len: MemLength,
        nonce: i64,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: get_esdt_nft_attribute_length")
    }

    fn get_esdt_nft_uri_length(
        &mut self,
        address_offset: MemPtr,
        token_id_offset: MemPtr,
        token_id_len: MemLength,
        nonce: i64,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: get_esdt_nft_uri_length")
    }

    fn get_esdt_token_data(
        &mut self,
        address_offset: MemPtr,
        token_id_offset: MemPtr,
        token_id_len: MemLength,
        nonce: i64,
        value_handle: i32,
        properties_offset: MemPtr,
        hash_offset: MemPtr,
        name_offset: MemPtr,
        attributes_offset: MemPtr,
        creator_offset: MemPtr,
        royalties_handle: i32,
        uris_offset: MemPtr,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: get_esdt_token_data")
    }

    fn get_esdt_local_roles(&mut self, token_id_handle: i32) -> Result<i64, VMHooksError> {
        Ok(self.handler.get_esdt_local_roles_bits(token_id_handle) as i64)
    }

    fn validate_token_identifier(&mut self, token_id_handle: i32) -> Result<i32, VMHooksError> {
        panic!("Unavailable: validate_token_identifier")
    }

    fn transfer_value(
        &mut self,
        dest_offset: MemPtr,
        value_offset: MemPtr,
        data_offset: MemPtr,
        length: MemLength,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: transfer_value")
    }

    fn transfer_value_execute(
        &mut self,
        dest_offset: MemPtr,
        value_offset: MemPtr,
        gas_limit: i64,
        function_offset: MemPtr,
        function_length: MemLength,
        num_arguments: i32,
        arguments_length_offset: MemPtr,
        data_offset: MemPtr,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: transfer_value_execute")
    }

    fn transfer_esdt_execute(
        &mut self,
        dest_offset: MemPtr,
        token_id_offset: MemPtr,
        token_id_len: MemLength,
        value_offset: MemPtr,
        gas_limit: i64,
        function_offset: MemPtr,
        function_length: MemLength,
        num_arguments: i32,
        arguments_length_offset: MemPtr,
        data_offset: MemPtr,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: transfer_esdt_execute")
    }

    fn transfer_esdt_nft_execute(
        &mut self,
        dest_offset: MemPtr,
        token_id_offset: MemPtr,
        token_id_len: MemLength,
        value_offset: MemPtr,
        nonce: i64,
        gas_limit: i64,
        function_offset: MemPtr,
        function_length: MemLength,
        num_arguments: i32,
        arguments_length_offset: MemPtr,
        data_offset: MemPtr,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: transfer_esdt_nft_execute")
    }

    fn multi_transfer_esdt_nft_execute(
        &mut self,
        dest_offset: MemPtr,
        num_token_transfers: i32,
        token_transfers_args_length_offset: MemPtr,
        token_transfer_data_offset: MemPtr,
        gas_limit: i64,
        function_offset: MemPtr,
        function_length: MemLength,
        num_arguments: i32,
        arguments_length_offset: MemPtr,
        data_offset: MemPtr,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: multi_transfer_esdt_nft_execute")
    }

    fn create_async_call(
        &mut self,
        dest_offset: MemPtr,
        value_offset: MemPtr,
        data_offset: MemPtr,
        data_length: MemLength,
        success_offset: MemPtr,
        success_length: MemLength,
        error_offset: MemPtr,
        error_length: MemLength,
        gas: i64,
        extra_gas_for_callback: i64,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: create_async_call")
    }

    fn set_async_context_callback(
        &mut self,
        callback: MemPtr,
        callback_length: MemLength,
        data: MemPtr,
        data_length: MemLength,
        gas: i64,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: set_async_context_callback")
    }

    fn upgrade_contract(
        &mut self,
        dest_offset: MemPtr,
        gas_limit: i64,
        value_offset: MemPtr,
        code_offset: MemPtr,
        code_metadata_offset: MemPtr,
        length: MemLength,
        num_arguments: i32,
        arguments_length_offset: MemPtr,
        data_offset: MemPtr,
    ) -> Result<(), VMHooksError> {
        panic!("Unavailable: upgrade_contract");
    }

    fn upgrade_from_source_contract(
        &mut self,
        dest_offset: MemPtr,
        gas_limit: i64,
        value_offset: MemPtr,
        source_contract_address_offset: MemPtr,
        code_metadata_offset: MemPtr,
        num_arguments: i32,
        arguments_length_offset: MemPtr,
        data_offset: MemPtr,
    ) -> Result<(), VMHooksError> {
        panic!("Unavailable: upgrade_from_source_contract");
    }

    fn delete_contract(
        &mut self,
        dest_offset: MemPtr,
        gas_limit: i64,
        num_arguments: i32,
        arguments_length_offset: MemPtr,
        data_offset: MemPtr,
    ) -> Result<(), VMHooksError> {
        panic!("Unavailable: delete_contract");
    }

    fn async_call(
        &mut self,
        dest_offset: MemPtr,
        value_offset: MemPtr,
        data_offset: MemPtr,
        length: MemLength,
    ) -> Result<(), VMHooksError> {
        panic!("Unavailable: async_call");
    }

    fn get_argument_length(&mut self, id: i32) -> Result<i32, VMHooksError> {
        panic!("Unavailable: get_argument_length")
    }

    fn get_argument(&mut self, id: i32, arg_offset: MemPtr) -> Result<i32, VMHooksError> {
        panic!("Unavailable: get_argument")
    }

    fn get_function(&mut self, function_offset: MemPtr) -> Result<i32, VMHooksError> {
        panic!("Unavailable: get_function")
    }

    fn get_num_arguments(&mut self) -> Result<i32, VMHooksError> {
        Ok(self.handler.get_num_arguments())
    }

    fn storage_store(
        &mut self,
        key_offset: MemPtr,
        key_length: MemLength,
        data_offset: MemPtr,
        data_length: MemLength,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: storage_store")
    }

    fn storage_load_length(
        &mut self,
        key_offset: MemPtr,
        key_length: MemLength,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: storage_load_length")
    }

    fn storage_load_from_address(
        &mut self,
        address_offset: MemPtr,
        key_offset: MemPtr,
        key_length: MemLength,
        data_offset: MemPtr,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: storage_load_from_address")
    }

    fn storage_load(
        &mut self,
        key_offset: MemPtr,
        key_length: MemLength,
        data_offset: MemPtr,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: storage_load")
    }

    fn set_storage_lock(
        &mut self,
        key_offset: MemPtr,
        key_length: MemLength,
        lock_timestamp: i64,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: set_storage_lock")
    }

    fn get_storage_lock(
        &mut self,
        key_offset: MemPtr,
        key_length: MemLength,
    ) -> Result<i64, VMHooksError> {
        panic!("Unavailable: get_storage_lock")
    }

    fn is_storage_locked(
        &mut self,
        key_offset: MemPtr,
        key_length: MemLength,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: is_storage_locked")
    }

    fn clear_storage_lock(
        &mut self,
        key_offset: MemPtr,
        key_length: MemLength,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: clear_storage_lock")
    }

    fn get_caller(&mut self, result_offset: MemPtr) -> Result<(), VMHooksError> {
        panic!("Unavailable: get_caller");
    }

    fn check_no_payment(&mut self) -> Result<(), VMHooksError> {
        self.handler.check_not_payable()
    }

    fn get_call_value(&mut self, result_offset: MemPtr) -> Result<i32, VMHooksError> {
        panic!("Unavailable: get_call_value")
    }

    fn get_esdt_value(&mut self, result_offset: MemPtr) -> Result<i32, VMHooksError> {
        panic!("Unavailable: get_esdt_value")
    }

    fn get_esdt_value_by_index(
        &mut self,
        result_offset: MemPtr,
        index: i32,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: get_esdt_value_by_index")
    }

    fn get_esdt_token_name(&mut self, result_offset: MemPtr) -> Result<i32, VMHooksError> {
        panic!("Unavailable: get_esdt_token_name")
    }

    fn get_esdt_token_name_by_index(
        &mut self,
        result_offset: MemPtr,
        index: i32,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: get_esdt_token_name_by_index")
    }

    fn get_esdt_token_nonce(&mut self) -> Result<i64, VMHooksError> {
        panic!("Unavailable: get_esdt_token_nonce")
    }

    fn get_esdt_token_nonce_by_index(&mut self, index: i32) -> Result<i64, VMHooksError> {
        panic!("Unavailable: get_esdt_token_nonce_by_index")
    }

    fn get_current_esdt_nft_nonce(
        &mut self,
        address_offset: MemPtr,
        token_id_offset: MemPtr,
        token_id_len: MemLength,
    ) -> Result<i64, VMHooksError> {
        unsafe {
            let address_bytes = self.handler.memory_load(address_offset, 32);
            let token_id_bytes = self.handler.memory_load(token_id_offset, token_id_len);
            Ok(self
                .handler
                .get_current_esdt_nft_nonce(&address_bytes, &token_id_bytes) as i64)
        }
    }

    fn get_esdt_token_type(&mut self) -> Result<i32, VMHooksError> {
        panic!("Unavailable: get_esdt_token_type")
    }

    fn get_esdt_token_type_by_index(&mut self, index: i32) -> Result<i32, VMHooksError> {
        panic!("Unavailable: get_esdt_token_type_by_index")
    }

    fn get_num_esdt_transfers(&mut self) -> Result<i32, VMHooksError> {
        Ok(self.handler.esdt_num_transfers() as i32)
    }

    fn get_call_value_token_name(
        &mut self,
        call_value_offset: MemPtr,
        token_name_offset: MemPtr,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: get_call_value_token_name")
    }

    fn get_call_value_token_name_by_index(
        &mut self,
        call_value_offset: MemPtr,
        token_name_offset: MemPtr,
        index: i32,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: get_call_value_token_name_by_index")
    }

    fn write_log(
        &mut self,
        data_pointer: MemPtr,
        data_length: MemLength,
        topic_ptr: MemPtr,
        num_topics: i32,
    ) -> Result<(), VMHooksError> {
        panic!("Unavailable: write_log");
    }

    fn write_event_log(
        &mut self,
        num_topics: i32,
        topic_lengths_offset: MemPtr,
        topic_offset: MemPtr,
        data_offset: MemPtr,
        data_length: MemLength,
    ) -> Result<(), VMHooksError> {
        panic!("Unavailable: write_event_log");
    }

    fn get_block_timestamp(&mut self) -> Result<i64, VMHooksError> {
        Ok(self.handler.get_block_timestamp() as i64)
    }

    fn get_block_nonce(&mut self) -> Result<i64, VMHooksError> {
        Ok(self.handler.get_block_nonce() as i64)
    }

    fn get_block_round(&mut self) -> Result<i64, VMHooksError> {
        Ok(self.handler.get_block_round() as i64)
    }

    fn get_block_epoch(&mut self) -> Result<i64, VMHooksError> {
        Ok(self.handler.get_block_epoch() as i64)
    }

    fn get_block_random_seed(&mut self, pointer: MemPtr) -> Result<(), VMHooksError> {
        panic!("Unavailable: get_block_random_seed");
    }

    fn get_state_root_hash(&mut self, pointer: MemPtr) -> Result<(), VMHooksError> {
        panic!("Unavailable: get_state_root_hash");
    }

    fn get_prev_block_timestamp(&mut self) -> Result<i64, VMHooksError> {
        Ok(self.handler.get_prev_block_timestamp() as i64)
    }

    fn get_prev_block_nonce(&mut self) -> Result<i64, VMHooksError> {
        Ok(self.handler.get_prev_block_nonce() as i64)
    }

    fn get_prev_block_round(&mut self) -> Result<i64, VMHooksError> {
        Ok(self.handler.get_prev_block_round() as i64)
    }

    fn get_prev_block_epoch(&mut self) -> Result<i64, VMHooksError> {
        Ok(self.handler.get_prev_block_epoch() as i64)
    }

    fn get_prev_block_random_seed(&mut self, pointer: MemPtr) -> Result<(), VMHooksError> {
        panic!("Unavailable: get_prev_block_random_seed");
    }

    fn finish(&mut self, pointer: MemPtr, length: MemLength) -> Result<(), VMHooksError> {
        unsafe {
            let bytes = self.handler.memory_load(pointer, length);
            self.handler.finish_slice_u8(&bytes)
        }
    }

    fn execute_on_same_context(
        &mut self,
        gas_limit: i64,
        address_offset: MemPtr,
        value_offset: MemPtr,
        function_offset: MemPtr,
        function_length: MemLength,
        num_arguments: i32,
        arguments_length_offset: MemPtr,
        data_offset: MemPtr,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: execute_on_same_context")
    }

    fn execute_on_dest_context(
        &mut self,
        gas_limit: i64,
        address_offset: MemPtr,
        value_offset: MemPtr,
        function_offset: MemPtr,
        function_length: MemLength,
        num_arguments: i32,
        arguments_length_offset: MemPtr,
        data_offset: MemPtr,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: execute_on_dest_context")
    }

    fn execute_read_only(
        &mut self,
        gas_limit: i64,
        address_offset: MemPtr,
        function_offset: MemPtr,
        function_length: MemLength,
        num_arguments: i32,
        arguments_length_offset: MemPtr,
        data_offset: MemPtr,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: execute_read_only")
    }

    fn create_contract(
        &mut self,
        gas_limit: i64,
        value_offset: MemPtr,
        code_offset: MemPtr,
        code_metadata_offset: MemPtr,
        length: MemLength,
        result_offset: MemPtr,
        num_arguments: i32,
        arguments_length_offset: MemPtr,
        data_offset: MemPtr,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: create_contract")
    }

    fn deploy_from_source_contract(
        &mut self,
        gas_limit: i64,
        value_offset: MemPtr,
        source_contract_address_offset: MemPtr,
        code_metadata_offset: MemPtr,
        result_address_offset: MemPtr,
        num_arguments: i32,
        arguments_length_offset: MemPtr,
        data_offset: MemPtr,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: deploy_from_source_contract")
    }

    fn get_num_return_data(&mut self) -> Result<i32, VMHooksError> {
        panic!("Unavailable: get_num_return_data")
    }

    fn get_return_data_size(&mut self, result_id: i32) -> Result<i32, VMHooksError> {
        panic!("Unavailable: get_return_data_size")
    }

    fn get_return_data(
        &mut self,
        result_id: i32,
        data_offset: MemPtr,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: get_return_data")
    }

    fn clean_return_data(&mut self) -> Result<(), VMHooksError> {
        self.handler.clean_return_data()
    }

    fn delete_from_return_data(&mut self, result_id: i32) -> Result<(), VMHooksError> {
        self.handler.delete_from_return_data(result_id as usize)
    }

    fn get_original_tx_hash(&mut self, data_offset: MemPtr) -> Result<(), VMHooksError> {
        panic!("Unavailable: get_original_tx_hash");
    }

    fn get_current_tx_hash(&mut self, data_offset: MemPtr) -> Result<(), VMHooksError> {
        panic!("Unavailable: get_current_tx_hash");
    }

    fn get_prev_tx_hash(&mut self, data_offset: MemPtr) -> Result<(), VMHooksError> {
        panic!("Unavailable: get_prev_tx_hash");
    }

    fn managed_sc_address(&mut self, destination_handle: i32) -> Result<(), VMHooksError> {
        self.handler.managed_sc_address(destination_handle)
    }

    fn managed_owner_address(&mut self, destination_handle: i32) -> Result<(), VMHooksError> {
        self.handler.managed_owner_address(destination_handle)
    }

    fn managed_caller(&mut self, destination_handle: i32) -> Result<(), VMHooksError> {
        self.handler.managed_caller(destination_handle)
    }

    fn managed_signal_error(&mut self, err_handle: i32) -> Result<(), VMHooksError> {
        self.handler.signal_error_from_buffer(err_handle)
    }

    fn managed_write_log(
        &mut self,
        topics_handle: i32,
        data_handle: i32,
    ) -> Result<(), VMHooksError> {
        self.handler.managed_write_log(topics_handle, data_handle)
    }

    fn managed_get_original_tx_hash(&mut self, result_handle: i32) -> Result<(), VMHooksError> {
        self.handler.get_tx_hash(result_handle)
    }

    fn managed_get_state_root_hash(&mut self, result_handle: i32) -> Result<(), VMHooksError> {
        panic!("Unavailable: managed_get_state_root_hash");
    }

    fn managed_get_block_random_seed(&mut self, result_handle: i32) -> Result<(), VMHooksError> {
        self.handler.get_block_random_seed(result_handle)
    }

    fn managed_get_prev_block_random_seed(
        &mut self,
        result_handle: i32,
    ) -> Result<(), VMHooksError> {
        self.handler.get_prev_block_random_seed(result_handle)
    }

    fn managed_get_return_data(
        &mut self,
        result_id: i32,
        result_handle: i32,
    ) -> Result<(), VMHooksError> {
        panic!("Unavailable: managed_get_return_data");
    }

    fn managed_get_multi_esdt_call_value(
        &mut self,
        multi_call_value_handle: i32,
    ) -> Result<(), VMHooksError> {
        self.handler
            .load_all_esdt_transfers(multi_call_value_handle)
    }

    fn managed_get_esdt_balance(
        &mut self,
        address_handle: i32,
        token_id_handle: i32,
        nonce: i64,
        value_handle: i32,
    ) -> Result<(), VMHooksError> {
        panic!("Unavailable: managed_get_esdt_balance");
    }

    fn managed_get_esdt_token_data(
        &mut self,
        address_handle: i32,
        token_id_handle: i32,
        nonce: i64,
        value_handle: i32,
        properties_handle: i32,
        hash_handle: i32,
        name_handle: i32,
        attributes_handle: i32,
        creator_handle: i32,
        royalties_handle: i32,
        uris_handle: i32,
    ) -> Result<(), VMHooksError> {
        self.handler.managed_get_esdt_token_data(
            address_handle,
            token_id_handle,
            nonce as u64,
            value_handle,
            properties_handle,
            hash_handle,
            name_handle,
            attributes_handle,
            creator_handle,
            royalties_handle,
            uris_handle,
        )
    }

    fn managed_get_back_transfers(
        &mut self,
        esdt_transfer_value_handle: i32,
        call_value_handle: i32,
    ) -> Result<(), VMHooksError> {
        self.handler
            .managed_get_back_transfers(esdt_transfer_value_handle, call_value_handle)
    }

    fn managed_async_call(
        &mut self,
        dest_handle: i32,
        value_handle: i32,
        function_handle: i32,
        arguments_handle: i32,
    ) -> Result<(), VMHooksError> {
        self.handler
            .async_call_raw(dest_handle, value_handle, function_handle, arguments_handle);
    }

    fn managed_create_async_call(
        &mut self,
        dest_handle: i32,
        value_handle: i32,
        function_handle: i32,
        arguments_handle: i32,
        success_offset: MemPtr,
        success_length: MemLength,
        error_offset: MemPtr,
        error_length: MemLength,
        gas: i64,
        extra_gas_for_callback: i64,
        callback_closure_handle: i32,
    ) -> Result<i32, VMHooksError> {
        unsafe {
            let success_callback = self.handler.memory_load(success_offset, success_length);
            let error_callback = self.handler.memory_load(error_offset, error_length);
            self.handler.create_async_call_raw(
                dest_handle,
                value_handle,
                function_handle,
                arguments_handle,
                &success_callback,
                &error_callback,
                gas as u64,
                extra_gas_for_callback as u64,
                callback_closure_handle,
            );
        }
        Ok(RESULT_OK)
    }

    fn managed_get_callback_closure(
        &mut self,
        callback_closure_handle: i32,
    ) -> Result<(), VMHooksError> {
        self.handler
            .load_callback_closure_buffer(callback_closure_handle)
    }

    fn managed_upgrade_from_source_contract(
        &mut self,
        dest_handle: i32,
        gas: i64,
        value_handle: i32,
        address_handle: i32,
        code_metadata_handle: i32,
        arguments_handle: i32,
        _result_handle: i32,
    ) -> Result<(), VMHooksError> {
        self.handler.upgrade_from_source_contract(
            dest_handle,
            gas as u64,
            value_handle,
            address_handle,
            code_metadata_handle,
            arguments_handle,
        )
    }

    fn managed_upgrade_contract(
        &mut self,
        dest_handle: i32,
        gas: i64,
        value_handle: i32,
        code_handle: i32,
        code_metadata_handle: i32,
        arguments_handle: i32,
        _result_handle: i32,
    ) -> Result<(), VMHooksError> {
        self.handler.upgrade_contract(
            dest_handle,
            gas as u64,
            value_handle,
            code_handle,
            code_metadata_handle,
            arguments_handle,
        )
    }

    fn managed_delete_contract(
        &mut self,
        dest_handle: i32,
        gas_limit: i64,
        arguments_handle: i32,
    ) -> Result<(), VMHooksError> {
        panic!("Unavailable: managed_delete_contract");
    }

    fn managed_deploy_from_source_contract(
        &mut self,
        gas: i64,
        value_handle: i32,
        address_handle: i32,
        code_metadata_handle: i32,
        arguments_handle: i32,
        result_address_handle: i32,
        result_handle: i32,
    ) -> Result<i32, VMHooksError> {
        self.handler.deploy_from_source_contract(
            gas as u64,
            value_handle,
            address_handle,
            code_metadata_handle,
            arguments_handle,
            result_address_handle,
            result_handle,
        );
        Ok(RESULT_OK)
    }

    fn managed_create_contract(
        &mut self,
        gas: i64,
        value_handle: i32,
        code_handle: i32,
        code_metadata_handle: i32,
        arguments_handle: i32,
        result_address_handle: i32,
        result_handle: i32,
    ) -> Result<i32, VMHooksError> {
        self.handler.deploy_contract(
            gas as u64,
            value_handle,
            code_handle,
            code_metadata_handle,
            arguments_handle,
            result_address_handle,
            result_handle,
        );
        Ok(RESULT_OK)
    }

    fn managed_execute_read_only(
        &mut self,
        gas: i64,
        address_handle: i32,
        function_handle: i32,
        arguments_handle: i32,
        result_handle: i32,
    ) -> Result<i32, VMHooksError> {
        self.handler.execute_on_dest_context_readonly_raw(
            gas as u64,
            address_handle,
            function_handle,
            arguments_handle,
            result_handle,
        );
        Ok(RESULT_OK)
    }

    fn managed_execute_on_same_context(
        &mut self,
        gas: i64,
        address_handle: i32,
        value_handle: i32,
        function_handle: i32,
        arguments_handle: i32,
        result_handle: i32,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: managed_execute_on_same_context")
    }

    fn managed_execute_on_dest_context(
        &mut self,
        gas: i64,
        address_handle: i32,
        value_handle: i32,
        function_handle: i32,
        arguments_handle: i32,
        result_handle: i32,
    ) -> Result<i32, VMHooksError> {
        self.handler.execute_on_dest_context_raw(
            gas as u64,
            address_handle,
            value_handle,
            function_handle,
            arguments_handle,
            result_handle,
        );
        Ok(RESULT_OK)
    }

    fn managed_multi_transfer_esdt_nft_execute(
        &mut self,
        dst_handle: i32,
        token_transfers_handle: i32,
        gas_limit: i64,
        function_handle: i32,
        arguments_handle: i32,
    ) -> Result<i32, VMHooksError> {
        self.handler.multi_transfer_esdt_nft_execute(
            dst_handle,
            token_transfers_handle,
            gas_limit as u64,
            function_handle,
            arguments_handle,
        );
        Ok(RESULT_OK)
    }

    fn managed_transfer_value_execute(
        &mut self,
        dst_handle: i32,
        value_handle: i32,
        gas_limit: i64,
        function_handle: i32,
        arguments_handle: i32,
    ) -> Result<i32, VMHooksError> {
        let _ = self.handler.transfer_value_execute(
            dst_handle,
            value_handle,
            gas_limit as u64,
            function_handle,
            arguments_handle,
        );
        Ok(RESULT_OK)
    }

    fn managed_is_esdt_frozen(
        &mut self,
        address_handle: i32,
        token_id_handle: i32,
        nonce: i64,
    ) -> Result<i32, VMHooksError> {
        bool_to_i32(
            self.handler
                .check_esdt_frozen(address_handle, token_id_handle, nonce as u64),
        )
    }

    fn managed_is_esdt_limited_transfer(
        &mut self,
        _token_id_handle: i32,
    ) -> Result<i32, VMHooksError> {
        bool_to_i32(false)
    }

    fn managed_is_esdt_paused(&mut self, _token_id_handle: i32) -> Result<i32, VMHooksError> {
        bool_to_i32(false)
    }

    fn managed_buffer_to_hex(
        &mut self,
        source_handle: i32,
        dest_handle: i32,
    ) -> Result<(), VMHooksError> {
        self.handler.mb_to_hex(source_handle, dest_handle)
    }

    fn managed_get_code_metadata(
        &mut self,
        address_handle: i32,
        response_handle: i32,
    ) -> Result<(), VMHooksError> {
        self.handler
            .managed_get_code_metadata(address_handle, response_handle)
    }

    fn managed_is_builtin_function(
        &mut self,
        function_name_handle: i32,
    ) -> Result<i32, VMHooksError> {
        bool_to_i32(
            self.handler
                .managed_is_builtin_function(function_name_handle),
        )
    }

    fn big_float_new_from_parts(
        &mut self,
        integral_part: i32,
        fractional_part: i32,
        exponent: i32,
    ) -> Result<i32, VMHooksError> {
        Ok(self
            .handler
            .bf_from_parts(integral_part, fractional_part, exponent))
    }

    fn big_float_new_from_frac(
        &mut self,
        numerator: i64,
        denominator: i64,
    ) -> Result<i32, VMHooksError> {
        Ok(self.handler.bf_from_frac(numerator, denominator))
    }

    fn big_float_new_from_sci(
        &mut self,
        significand: i64,
        exponent: i64,
    ) -> Result<i32, VMHooksError> {
        Ok(self.handler.bf_from_sci(significand, exponent))
    }

    fn big_float_add(
        &mut self,
        destination_handle: i32,
        op1_handle: i32,
        op2_handle: i32,
    ) -> Result<(), VMHooksError> {
        self.handler
            .bf_add(destination_handle, op1_handle, op2_handle)
    }

    fn big_float_sub(
        &mut self,
        destination_handle: i32,
        op1_handle: i32,
        op2_handle: i32,
    ) -> Result<(), VMHooksError> {
        self.handler
            .bf_sub(destination_handle, op1_handle, op2_handle)
    }

    fn big_float_mul(
        &mut self,
        destination_handle: i32,
        op1_handle: i32,
        op2_handle: i32,
    ) -> Result<(), VMHooksError> {
        self.handler
            .bf_mul(destination_handle, op1_handle, op2_handle)
    }

    fn big_float_div(
        &mut self,
        destination_handle: i32,
        op1_handle: i32,
        op2_handle: i32,
    ) -> Result<(), VMHooksError> {
        self.handler
            .bf_div(destination_handle, op1_handle, op2_handle)
    }

    fn big_float_neg(
        &mut self,
        destination_handle: i32,
        op_handle: i32,
    ) -> Result<(), VMHooksError> {
        self.handler.bf_neg(destination_handle, op_handle)
    }

    fn big_float_clone(
        &mut self,
        destination_handle: i32,
        op_handle: i32,
    ) -> Result<(), VMHooksError> {
        self.handler.bf_clone(destination_handle, op_handle)
    }

    fn big_float_cmp(&mut self, op1_handle: i32, op2_handle: i32) -> Result<i32, VMHooksError> {
        Ok(self.handler.bf_cmp(op1_handle, op2_handle))
    }

    fn big_float_abs(
        &mut self,
        destination_handle: i32,
        op_handle: i32,
    ) -> Result<(), VMHooksError> {
        self.handler.bf_abs(destination_handle, op_handle)
    }

    fn big_float_sign(&mut self, op_handle: i32) -> Result<i32, VMHooksError> {
        Ok(self.handler.bf_sign(op_handle))
    }

    fn big_float_sqrt(
        &mut self,
        destination_handle: i32,
        op_handle: i32,
    ) -> Result<(), VMHooksError> {
        self.handler.bf_sqrt(destination_handle, op_handle)
    }

    fn big_float_pow(
        &mut self,
        destination_handle: i32,
        op_handle: i32,
        exponent: i32,
    ) -> Result<(), VMHooksError> {
        self.handler.bf_pow(destination_handle, op_handle, exponent)
    }

    fn big_float_floor(
        &mut self,
        dest_big_int_handle: i32,
        op_handle: i32,
    ) -> Result<(), VMHooksError> {
        self.handler.bf_floor(dest_big_int_handle, op_handle)
    }

    fn big_float_ceil(
        &mut self,
        dest_big_int_handle: i32,
        op_handle: i32,
    ) -> Result<(), VMHooksError> {
        self.handler.bf_ceil(dest_big_int_handle, op_handle)
    }

    fn big_float_truncate(
        &mut self,
        dest_big_int_handle: i32,
        op_handle: i32,
    ) -> Result<(), VMHooksError> {
        self.handler.bf_trunc(dest_big_int_handle, op_handle)
    }

    fn big_float_set_int64(
        &mut self,
        destination_handle: i32,
        value: i64,
    ) -> Result<(), VMHooksError> {
        self.handler.bf_set_i64(destination_handle, value)
    }

    fn big_float_is_int(&mut self, op_handle: i32) -> Result<i32, VMHooksError> {
        bool_to_i32(self.handler.bf_is_bi(op_handle))
    }

    fn big_float_set_big_int(
        &mut self,
        destination_handle: i32,
        big_int_handle: i32,
    ) -> Result<(), VMHooksError> {
        self.handler.bf_set_bi(destination_handle, big_int_handle)
    }

    fn big_float_get_const_pi(&mut self, destination_handle: i32) -> Result<(), VMHooksError> {
        self.handler.bf_get_const_pi(destination_handle)
    }

    fn big_float_get_const_e(&mut self, destination_handle: i32) -> Result<(), VMHooksError> {
        self.handler.bf_get_const_e(destination_handle)
    }

    fn big_int_get_unsigned_argument(
        &mut self,
        id: i32,
        destination_handle: i32,
    ) -> Result<(), VMHooksError> {
        self.handler
            .load_argument_big_int_unsigned(id, destination_handle)
    }

    fn big_int_get_signed_argument(
        &mut self,
        id: i32,
        destination_handle: i32,
    ) -> Result<(), VMHooksError> {
        self.handler
            .load_argument_big_int_signed(id, destination_handle)
    }

    fn big_int_storage_store_unsigned(
        &mut self,
        key_offset: MemPtr,
        key_length: MemLength,
        source_handle: i32,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: big_int_storage_store_unsigned")
    }

    fn big_int_storage_load_unsigned(
        &mut self,
        key_offset: MemPtr,
        key_length: MemLength,
        destination_handle: i32,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: big_int_storage_load_unsigned")
    }

    fn big_int_get_call_value(&mut self, destination_handle: i32) -> Result<(), VMHooksError> {
        self.handler.load_egld_value(destination_handle)
    }

    fn big_int_get_esdt_call_value(&mut self, destination: i32) -> Result<(), VMHooksError> {
        panic!("Unavailable: big_int_get_esdt_call_value");
    }

    fn big_int_get_esdt_call_value_by_index(
        &mut self,
        destination_handle: i32,
        index: i32,
    ) -> Result<(), VMHooksError> {
        panic!("Unavailable: big_int_get_esdt_call_value_by_index");
    }

    fn big_int_get_external_balance(
        &mut self,
        address_offset: MemPtr,
        result: i32,
    ) -> Result<(), VMHooksError> {
        unsafe {
            let address_bytes = self.handler.memory_load(address_offset, 32);
            self.handler.load_balance(&address_bytes, result)
        }
    }

    fn big_int_get_esdt_external_balance(
        &mut self,
        address_offset: MemPtr,
        token_id_offset: MemPtr,
        token_id_len: MemLength,
        nonce: i64,
        result_handle: i32,
    ) -> Result<(), VMHooksError> {
        unsafe {
            let address_bytes = self.handler.memory_load(address_offset, 32);
            let token_id_bytes = self.handler.memory_load(token_id_offset, token_id_len);
            self.handler.big_int_get_esdt_external_balance(
                &address_bytes,
                &token_id_bytes,
                nonce as u64,
                result_handle,
            )
        }
    }

    fn big_int_new(&mut self, small_value: i64) -> Result<i32, VMHooksError> {
        self.handler.bi_new(small_value)
    }

    fn big_int_unsigned_byte_length(&mut self, reference_handle: i32) -> Result<i32, VMHooksError> {
        panic!("Unavailable: big_int_unsigned_byte_length")
    }

    fn big_int_signed_byte_length(&mut self, reference_handle: i32) -> Result<i32, VMHooksError> {
        panic!("Unavailable: big_int_signed_byte_length")
    }

    fn big_int_get_unsigned_bytes(
        &mut self,
        reference_handle: i32,
        byte_offset: MemPtr,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: big_int_get_unsigned_bytes")
    }

    fn big_int_get_signed_bytes(
        &mut self,
        reference_handle: i32,
        byte_offset: MemPtr,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: big_int_get_signed_bytes")
    }

    fn big_int_set_unsigned_bytes(
        &mut self,
        destination_handle: i32,
        byte_offset: MemPtr,
        byte_length: MemLength,
    ) -> Result<(), VMHooksError> {
        unsafe {
            let bytes = self.handler.memory_load(byte_offset, byte_length);
            self.handler
                .bi_set_unsigned_bytes(destination_handle, &bytes)
        }
    }

    fn big_int_set_signed_bytes(
        &mut self,
        destination_handle: i32,
        byte_offset: MemPtr,
        byte_length: MemLength,
    ) -> Result<(), VMHooksError> {
        unsafe {
            let bytes = self.handler.memory_load(byte_offset, byte_length);
            self.handler.bi_set_signed_bytes(destination_handle, &bytes)
        }
    }

    fn big_int_is_int64(&mut self, destination_handle: i32) -> Result<i32, VMHooksError> {
        self.handler.bi_is_int64(destination_handle)
    }

    fn big_int_get_int64(&mut self, destination_handle: i32) -> Result<i64, VMHooksError> {
        self.handler.bi_get_int64(destination_handle)
    }

    fn big_int_set_int64(
        &mut self,
        destination_handle: i32,
        value: i64,
    ) -> Result<(), VMHooksError> {
        self.handler.bi_set_int64(destination_handle, value)
    }

    fn big_int_add(
        &mut self,
        destination_handle: i32,
        op1_handle: i32,
        op2_handle: i32,
    ) -> Result<(), VMHooksError> {
        self.handler
            .bi_add(destination_handle, op1_handle, op2_handle)
    }

    fn big_int_sub(
        &mut self,
        destination_handle: i32,
        op1_handle: i32,
        op2_handle: i32,
    ) -> Result<(), VMHooksError> {
        self.handler
            .bi_sub(destination_handle, op1_handle, op2_handle)
    }

    fn big_int_mul(
        &mut self,
        destination_handle: i32,
        op1_handle: i32,
        op2_handle: i32,
    ) -> Result<(), VMHooksError> {
        self.handler
            .bi_mul(destination_handle, op1_handle, op2_handle)
    }

    fn big_int_tdiv(
        &mut self,
        destination_handle: i32,
        op1_handle: i32,
        op2_handle: i32,
    ) -> Result<(), VMHooksError> {
        self.handler
            .bi_t_div(destination_handle, op1_handle, op2_handle)
    }

    fn big_int_tmod(
        &mut self,
        destination_handle: i32,
        op1_handle: i32,
        op2_handle: i32,
    ) -> Result<(), VMHooksError> {
        self.handler
            .bi_t_mod(destination_handle, op1_handle, op2_handle)
    }

    fn big_int_ediv(
        &mut self,
        destination_handle: i32,
        op1_handle: i32,
        op2_handle: i32,
    ) -> Result<(), VMHooksError> {
        panic!("Not supported: big_int_ediv");
    }

    fn big_int_emod(
        &mut self,
        destination_handle: i32,
        op1_handle: i32,
        op2_handle: i32,
    ) -> Result<(), VMHooksError> {
        panic!("Not supported: big_int_emod");
    }

    fn big_int_sqrt(
        &mut self,
        destination_handle: i32,
        op_handle: i32,
    ) -> Result<(), VMHooksError> {
        self.handler.bi_sqrt(destination_handle, op_handle)
    }

    fn big_int_pow(
        &mut self,
        destination_handle: i32,
        op1_handle: i32,
        op2_handle: i32,
    ) -> Result<(), VMHooksError> {
        self.handler
            .bi_pow(destination_handle, op1_handle, op2_handle)
    }

    fn big_int_log2(&mut self, op_handle: i32) -> Result<i32, VMHooksError> {
        Ok(self.handler.bi_log2(op_handle))
    }

    fn big_int_abs(&mut self, destination_handle: i32, op_handle: i32) -> Result<(), VMHooksError> {
        self.handler.bi_abs(destination_handle, op_handle)
    }

    fn big_int_neg(&mut self, destination_handle: i32, op_handle: i32) -> Result<(), VMHooksError> {
        self.handler.bi_neg(destination_handle, op_handle)
    }

    fn big_int_sign(&mut self, op_handle: i32) -> Result<i32, VMHooksError> {
        Ok(self.handler.bi_sign(op_handle))
    }

    fn big_int_cmp(&mut self, op1_handle: i32, op2_handle: i32) -> Result<i32, VMHooksError> {
        Ok(self.handler.bi_cmp(op1_handle, op2_handle))
    }

    fn big_int_not(&mut self, destination_handle: i32, op_handle: i32) -> Result<(), VMHooksError> {
        panic!("Unavailable: big_int_not");
    }

    fn big_int_and(
        &mut self,
        destination_handle: i32,
        op1_handle: i32,
        op2_handle: i32,
    ) -> Result<(), VMHooksError> {
        self.handler
            .bi_and(destination_handle, op1_handle, op2_handle)
    }

    fn big_int_or(
        &mut self,
        destination_handle: i32,
        op1_handle: i32,
        op2_handle: i32,
    ) -> Result<(), VMHooksError> {
        self.handler
            .bi_or(destination_handle, op1_handle, op2_handle)
    }

    fn big_int_xor(
        &mut self,
        destination_handle: i32,
        op1_handle: i32,
        op2_handle: i32,
    ) -> Result<(), VMHooksError> {
        self.handler
            .bi_xor(destination_handle, op1_handle, op2_handle)
    }

    fn big_int_shr(
        &mut self,
        destination_handle: i32,
        op_handle: i32,
        bits: i32,
    ) -> Result<(), VMHooksError> {
        self.handler
            .bi_shr(destination_handle, op_handle, bits as usize)
    }

    fn big_int_shl(
        &mut self,
        destination_handle: i32,
        op_handle: i32,
        bits: i32,
    ) -> Result<(), VMHooksError> {
        self.handler
            .bi_shl(destination_handle, op_handle, bits as usize)
    }

    fn big_int_finish_unsigned(&mut self, reference_handle: i32) -> Result<(), VMHooksError> {
        self.handler.finish_big_uint_raw(reference_handle)
    }

    fn big_int_finish_signed(&mut self, reference_handle: i32) -> Result<(), VMHooksError> {
        self.handler.finish_big_int_raw(reference_handle)
    }

    fn big_int_to_string(
        &mut self,
        big_int_handle: i32,
        destination_handle: i32,
    ) -> Result<(), VMHooksError> {
        self.handler
            .bi_to_string(big_int_handle, destination_handle)
    }

    fn mbuffer_new(&mut self) -> Result<i32, VMHooksError> {
        Ok(self.handler.mb_new_empty())
    }

    fn mbuffer_new_from_bytes(
        &mut self,
        data_offset: MemPtr,
        data_length: MemLength,
    ) -> Result<i32, VMHooksError> {
        unsafe {
            let bytes = self.handler.memory_load(data_offset, data_length);
            Ok(self.handler.mb_new_from_bytes(&bytes))
        }
    }

    fn mbuffer_get_length(&mut self, m_buffer_handle: i32) -> Result<i32, VMHooksError> {
        Ok(self.handler.mb_len(m_buffer_handle) as i32)
    }

    fn mbuffer_get_bytes(
        &mut self,
        m_buffer_handle: i32,
        result_offset: MemPtr,
    ) -> Result<i32, VMHooksError> {
        let bytes = self.handler.mb_get_bytes(m_buffer_handle);
        unsafe {
            self.handler.memory_store(result_offset, &bytes);
        }
        Ok(bytes.len() as i32)
    }

    fn mbuffer_get_byte_slice(
        &mut self,
        source_handle: i32,
        starting_position: i32,
        slice_length: i32,
        result_offset: MemPtr,
    ) -> Result<i32, VMHooksError> {
        if let Ok(bytes) = self.handler.mb_get_slice(
            source_handle,
            starting_position as usize,
            slice_length as usize,
        ) {
            assert_eq!(bytes.len(), slice_length as usize);
            unsafe {
                self.handler.memory_store(result_offset, &bytes);
            }
            Ok(RESULT_OK)
        } else {
            Ok(RESULT_ERROR)
        }
    }

    fn mbuffer_copy_byte_slice(
        &mut self,
        source_handle: i32,
        starting_position: i32,
        slice_length: i32,
        destination_handle: i32,
    ) -> Result<i32, VMHooksError> {
        Ok(self.handler.mb_copy_slice(
            source_handle,
            starting_position as usize,
            slice_length as usize,
            destination_handle,
        ))
    }

    fn mbuffer_eq(
        &mut self,
        m_buffer_handle1: i32,
        m_buffer_handle2: i32,
    ) -> Result<i32, VMHooksError> {
        Ok(self.handler.mb_eq(m_buffer_handle1, m_buffer_handle2))
    }

    fn mbuffer_set_bytes(
        &mut self,
        m_buffer_handle: i32,
        data_offset: MemPtr,
        data_length: MemLength,
    ) -> Result<i32, VMHooksError> {
        unsafe {
            let bytes = self.handler.memory_load(data_offset, data_length);
            self.handler.mb_set(m_buffer_handle, &bytes);
        }
        Ok(RESULT_OK)
    }

    fn mbuffer_set_byte_slice(
        &mut self,
        m_buffer_handle: i32,
        starting_position: i32,
        data_length: MemLength,
        data_offset: MemPtr,
    ) -> Result<i32, VMHooksError> {
        unsafe {
            let bytes = self.handler.memory_load(data_offset, data_length);
            Ok(self
                .handler
                .mb_set_slice(m_buffer_handle, starting_position as usize, &bytes))
        }
    }

    fn mbuffer_append(
        &mut self,
        accumulator_handle: i32,
        data_handle: i32,
    ) -> Result<i32, VMHooksError> {
        self.handler.mb_append(accumulator_handle, data_handle);
        Ok(RESULT_OK)
    }

    fn mbuffer_append_bytes(
        &mut self,
        accumulator_handle: i32,
        data_offset: MemPtr,
        data_length: MemLength,
    ) -> Result<i32, VMHooksError> {
        unsafe {
            let bytes = self.handler.memory_load(data_offset, data_length);
            self.handler.mb_append_bytes(accumulator_handle, &bytes);
        }
        Ok(RESULT_OK)
    }

    fn mbuffer_to_big_int_unsigned(
        &mut self,
        m_buffer_handle: i32,
        big_int_handle: i32,
    ) -> Result<i32, VMHooksError> {
        match self
            .handler
            .mb_to_big_int_unsigned(m_buffer_handle, big_int_handle)
        {
            Ok(_) => Ok(RESULT_OK),
            Err(e) => Err(e),
        }
    }

    fn mbuffer_to_big_int_signed(
        &mut self,
        m_buffer_handle: i32,
        big_int_handle: i32,
    ) -> Result<i32, VMHooksError> {
        match self
            .handler
            .mb_to_big_int_signed(m_buffer_handle, big_int_handle)
        {
            Ok(_) => Ok(RESULT_OK),
            Err(e) => Err(e),
        }
    }

    fn mbuffer_from_big_int_unsigned(
        &mut self,
        m_buffer_handle: i32,
        big_int_handle: i32,
    ) -> Result<i32, VMHooksError> {
        match self
            .handler
            .mb_from_big_int_unsigned(m_buffer_handle, big_int_handle)
        {
            Ok(_) => Ok(RESULT_OK),
            Err(e) => Err(e),
        }
    }

    fn mbuffer_from_big_int_signed(
        &mut self,
        m_buffer_handle: i32,
        big_int_handle: i32,
    ) -> Result<i32, VMHooksError> {
        match self
            .handler
            .mb_from_big_int_signed(m_buffer_handle, big_int_handle)
        {
            Ok(_) => Ok(RESULT_OK),
            Err(e) => Err(e),
        }
    }

    fn mbuffer_to_big_float(
        &mut self,
        m_buffer_handle: i32,
        big_float_handle: i32,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: mbuffer_to_big_float")
    }

    fn mbuffer_from_big_float(
        &mut self,
        m_buffer_handle: i32,
        big_float_handle: i32,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: mbuffer_from_big_float")
    }

    fn mbuffer_storage_store(
        &mut self,
        key_handle: i32,
        source_handle: i32,
    ) -> Result<i32, VMHooksError> {
        self.handler
            .storage_store_managed_buffer_raw(key_handle, source_handle);
        Ok(RESULT_OK)
    }

    fn mbuffer_storage_load(
        &mut self,
        key_handle: i32,
        destination_handle: i32,
    ) -> Result<i32, VMHooksError> {
        self.handler
            .storage_load_managed_buffer_raw(key_handle, destination_handle);
        Ok(RESULT_OK)
    }

    fn mbuffer_storage_load_from_address(
        &mut self,
        address_handle: i32,
        key_handle: i32,
        destination_handle: i32,
    ) -> Result<(), VMHooksError> {
        self.handler
            .storage_load_from_address(address_handle, key_handle, destination_handle)
    }

    fn mbuffer_get_argument(
        &mut self,
        id: i32,
        destination_handle: i32,
    ) -> Result<i32, VMHooksError> {
        match self
            .handler
            .load_argument_managed_buffer(id, destination_handle)
        {
            Ok(_) => Ok(RESULT_OK),
            Err(e) => Err(e),
        }
    }

    fn mbuffer_finish(&mut self, source_handle: i32) -> Result<i32, VMHooksError> {
        match self.handler.finish_managed_buffer_raw(source_handle) {
            Ok(_) => Ok(RESULT_OK),
            Err(e) => Err(e),
        }
    }

    fn mbuffer_set_random(
        &mut self,
        destination_handle: i32,
        length: i32,
    ) -> Result<i32, VMHooksError> {
        match self
            .handler
            .mb_set_random(destination_handle, length as usize)
        {
            Ok(_) => Ok(RESULT_OK),
            Err(e) => Err(e),
        }
    }

    fn managed_map_new(&mut self) -> Result<i32, VMHooksError> {
        Ok(self.handler.mm_new())
    }

    fn managed_map_put(
        &mut self,
        map_handle: i32,
        key_handle: i32,
        value_handle: i32,
    ) -> Result<i32, VMHooksError> {
        self.handler.mm_put(map_handle, key_handle, value_handle);
        Ok(RESULT_OK)
    }

    fn managed_map_get(
        &mut self,
        map_handle: i32,
        key_handle: i32,
        out_value_handle: i32,
    ) -> Result<i32, VMHooksError> {
        self.handler
            .mm_get(map_handle, key_handle, out_value_handle);
        Ok(RESULT_OK)
    }

    fn managed_map_remove(
        &mut self,
        map_handle: i32,
        key_handle: i32,
        out_value_handle: i32,
    ) -> Result<i32, VMHooksError> {
        self.handler
            .mm_remove(map_handle, key_handle, out_value_handle);
        Ok(RESULT_OK)
    }

    fn managed_map_contains(
        &mut self,
        map_handle: i32,
        key_handle: i32,
    ) -> Result<i32, VMHooksError> {
        bool_to_i32(self.handler.mm_contains(map_handle, key_handle))
    }

    fn small_int_get_unsigned_argument(&mut self, id: i32) -> Result<i64, VMHooksError> {
        Ok(self.handler.get_argument_u64(id) as i64)
    }

    fn small_int_get_signed_argument(&mut self, id: i32) -> Result<i64, VMHooksError> {
        Ok(self.handler.get_argument_i64(id))
    }

    fn small_int_finish_unsigned(&mut self, value: i64) -> Result<(), VMHooksError> {
        self.handler.finish_u64(value as u64)
    }

    fn small_int_finish_signed(&mut self, value: i64) -> Result<(), VMHooksError> {
        self.handler.finish_i64(value)
    }

    fn small_int_storage_store_unsigned(
        &mut self,
        key_offset: MemPtr,
        key_length: MemLength,
        value: i64,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: small_int_storage_store_unsigned")
    }

    fn small_int_storage_store_signed(
        &mut self,
        key_offset: MemPtr,
        key_length: MemLength,
        value: i64,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: small_int_storage_store_signed")
    }

    fn small_int_storage_load_unsigned(
        &mut self,
        key_offset: MemPtr,
        key_length: MemLength,
    ) -> Result<i64, VMHooksError> {
        panic!("Unavailable: small_int_storage_load_unsigned")
    }

    fn small_int_storage_load_signed(
        &mut self,
        key_offset: MemPtr,
        key_length: MemLength,
    ) -> Result<i64, VMHooksError> {
        panic!("Unavailable: small_int_storage_load_signed")
    }

    fn int64get_argument(&mut self, id: i32) -> Result<i64, VMHooksError> {
        panic!("Unavailable: int64get_argument")
    }

    fn int64finish(&mut self, value: i64) -> Result<(), VMHooksError> {
        panic!("Unavailable: int64finish");
    }

    fn int64storage_store(
        &mut self,
        key_offset: MemPtr,
        key_length: MemLength,
        value: i64,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: int64storage_store")
    }

    fn int64storage_load(
        &mut self,
        key_offset: MemPtr,
        key_length: MemLength,
    ) -> Result<i64, VMHooksError> {
        panic!("Unavailable: int64storage_load")
    }

    fn sha256(
        &mut self,
        data_offset: MemPtr,
        length: MemLength,
        result_offset: MemPtr,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: sha256")
    }

    fn managed_sha256(
        &mut self,
        input_handle: i32,
        output_handle: i32,
    ) -> Result<i32, VMHooksError> {
        self.handler.sha256_managed(output_handle, input_handle);
        Ok(RESULT_OK)
    }

    fn keccak256(
        &mut self,
        data_offset: MemPtr,
        length: MemLength,
        result_offset: MemPtr,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: keccak256")
    }

    fn managed_keccak256(
        &mut self,
        input_handle: i32,
        output_handle: i32,
    ) -> Result<i32, VMHooksError> {
        self.handler.keccak256_managed(output_handle, input_handle);
        Ok(RESULT_OK)
    }

    fn ripemd160(
        &mut self,
        data_offset: MemPtr,
        length: MemLength,
        result_offset: MemPtr,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: ripemd160")
    }

    fn managed_ripemd160(
        &mut self,
        input_handle: i32,
        output_handle: i32,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: managed_ripemd160")
    }

    fn verify_bls(
        &mut self,
        key_offset: MemPtr,
        message_offset: MemPtr,
        message_length: MemLength,
        sig_offset: MemPtr,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: verify_bls")
    }

    fn managed_verify_bls(
        &mut self,
        key_handle: i32,
        message_handle: i32,
        sig_handle: i32,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: managed_verify_bls")
    }

    fn verify_ed25519(
        &mut self,
        key_offset: MemPtr,
        message_offset: MemPtr,
        message_length: MemLength,
        sig_offset: MemPtr,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: verify_ed25519")
    }

    fn managed_verify_ed25519(
        &mut self,
        key_handle: i32,
        message_handle: i32,
        sig_handle: i32,
    ) -> Result<i32, VMHooksError> {
        self.handler
            .verify_ed25519_managed(key_handle, message_handle, sig_handle);
        Ok(RESULT_OK)
    }

    fn verify_custom_secp256k1(
        &mut self,
        key_offset: MemPtr,
        key_length: MemLength,
        message_offset: MemPtr,
        message_length: MemLength,
        sig_offset: MemPtr,
        hash_type: i32,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: verify_custom_secp256k1")
    }

    fn managed_verify_custom_secp256k1(
        &mut self,
        key_handle: i32,
        message_handle: i32,
        sig_handle: i32,
        hash_type: i32,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: managed_verify_custom_secp256k1")
    }

    fn verify_secp256k1(
        &mut self,
        key_offset: MemPtr,
        key_length: MemLength,
        message_offset: MemPtr,
        message_length: MemLength,
        sig_offset: MemPtr,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: verify_secp256k1")
    }

    fn managed_verify_secp256k1(
        &mut self,
        key_handle: i32,
        message_handle: i32,
        sig_handle: i32,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: managed_verify_secp256k1")
    }

    fn encode_secp256k1_der_signature(
        &mut self,
        r_offset: MemPtr,
        r_length: MemLength,
        s_offset: MemPtr,
        s_length: MemLength,
        sig_offset: MemPtr,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: encode_secp256k1_der_signature")
    }

    fn managed_encode_secp256k1_der_signature(
        &mut self,
        r_handle: i32,
        s_handle: i32,
        sig_handle: i32,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: managed_encode_secp256k1_der_signature")
    }

    fn add_ec(
        &mut self,
        x_result_handle: i32,
        y_result_handle: i32,
        ec_handle: i32,
        fst_point_xhandle: i32,
        fst_point_yhandle: i32,
        snd_point_xhandle: i32,
        snd_point_yhandle: i32,
    ) -> Result<(), VMHooksError> {
        panic!("Unavailable: add_ec");
    }

    fn double_ec(
        &mut self,
        x_result_handle: i32,
        y_result_handle: i32,
        ec_handle: i32,
        point_xhandle: i32,
        point_yhandle: i32,
    ) -> Result<(), VMHooksError> {
        panic!("Unavailable: double_ec");
    }

    fn is_on_curve_ec(
        &mut self,
        ec_handle: i32,
        point_xhandle: i32,
        point_yhandle: i32,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: is_on_curve_ec")
    }

    fn scalar_base_mult_ec(
        &mut self,
        x_result_handle: i32,
        y_result_handle: i32,
        ec_handle: i32,
        data_offset: MemPtr,
        length: MemLength,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: scalar_base_mult_ec")
    }

    fn managed_scalar_base_mult_ec(
        &mut self,
        x_result_handle: i32,
        y_result_handle: i32,
        ec_handle: i32,
        data_handle: i32,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: managed_scalar_base_mult_ec")
    }

    fn scalar_mult_ec(
        &mut self,
        x_result_handle: i32,
        y_result_handle: i32,
        ec_handle: i32,
        point_xhandle: i32,
        point_yhandle: i32,
        data_offset: MemPtr,
        length: MemLength,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: scalar_mult_ec")
    }

    fn managed_scalar_mult_ec(
        &mut self,
        x_result_handle: i32,
        y_result_handle: i32,
        ec_handle: i32,
        point_xhandle: i32,
        point_yhandle: i32,
        data_handle: i32,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: managed_scalar_mult_ec")
    }

    fn marshal_ec(
        &mut self,
        x_pair_handle: i32,
        y_pair_handle: i32,
        ec_handle: i32,
        result_offset: MemPtr,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: marshal_ec")
    }

    fn managed_marshal_ec(
        &mut self,
        x_pair_handle: i32,
        y_pair_handle: i32,
        ec_handle: i32,
        result_handle: i32,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: managed_marshal_ec")
    }

    fn marshal_compressed_ec(
        &mut self,
        x_pair_handle: i32,
        y_pair_handle: i32,
        ec_handle: i32,
        result_offset: MemPtr,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: marshal_compressed_ec")
    }

    fn managed_marshal_compressed_ec(
        &mut self,
        x_pair_handle: i32,
        y_pair_handle: i32,
        ec_handle: i32,
        result_handle: i32,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: managed_marshal_compressed_ec")
    }

    fn unmarshal_ec(
        &mut self,
        x_result_handle: i32,
        y_result_handle: i32,
        ec_handle: i32,
        data_offset: MemPtr,
        length: MemLength,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: unmarshal_ec")
    }

    fn managed_unmarshal_ec(
        &mut self,
        x_result_handle: i32,
        y_result_handle: i32,
        ec_handle: i32,
        data_handle: i32,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: managed_unmarshal_ec")
    }

    fn unmarshal_compressed_ec(
        &mut self,
        x_result_handle: i32,
        y_result_handle: i32,
        ec_handle: i32,
        data_offset: MemPtr,
        length: MemLength,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: unmarshal_compressed_ec")
    }

    fn managed_unmarshal_compressed_ec(
        &mut self,
        x_result_handle: i32,
        y_result_handle: i32,
        ec_handle: i32,
        data_handle: i32,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: managed_unmarshal_compressed_ec")
    }

    fn generate_key_ec(
        &mut self,
        x_pub_key_handle: i32,
        y_pub_key_handle: i32,
        ec_handle: i32,
        result_offset: MemPtr,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: generate_key_ec")
    }

    fn managed_generate_key_ec(
        &mut self,
        x_pub_key_handle: i32,
        y_pub_key_handle: i32,
        ec_handle: i32,
        result_handle: i32,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: managed_generate_key_ec")
    }

    fn create_ec(
        &mut self,
        data_offset: MemPtr,
        data_length: MemLength,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: create_ec")
    }

    fn managed_create_ec(&mut self, data_handle: i32) -> Result<i32, VMHooksError> {
        panic!("Unavailable: managed_create_ec")
    }

    fn get_curve_length_ec(&mut self, ec_handle: i32) -> Result<i32, VMHooksError> {
        panic!("Unavailable: get_curve_length_ec")
    }

    fn get_priv_key_byte_length_ec(&mut self, ec_handle: i32) -> Result<i32, VMHooksError> {
        panic!("Unavailable: get_priv_key_byte_length_ec")
    }

    fn elliptic_curve_get_values(
        &mut self,
        ec_handle: i32,
        field_order_handle: i32,
        base_point_order_handle: i32,
        eq_constant_handle: i32,
        x_base_point_handle: i32,
        y_base_point_handle: i32,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: elliptic_curve_get_values")
    }

    fn is_reserved_function_name(&mut self, name_handle: i32) -> Result<i32, VMHooksError> {
        panic!("Unavailable: is_reserved_function_name")
    }

    fn managed_get_original_caller_addr(
        &mut self,
        destination_handle: i32,
    ) -> Result<(), VMHooksError> {
        panic!("Unavailable: managed_get_original_caller_addr")
    }

    fn managed_get_relayer_addr(&mut self, destination_handle: i32) -> Result<(), VMHooksError> {
        panic!("Unavailable: managed_get_relayer_addr")
    }

    fn managed_multi_transfer_esdt_nft_execute_by_user(
        &mut self,
        user_handle: i32,
        dst_handle: i32,
        token_transfers_handle: i32,
        gas_limit: i64,
        function_handle: i32,
        arguments_handle: i32,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: managed_multi_transfer_esdt_nft_execute_by_user")
    }

    fn managed_verify_secp256r1(
        &mut self,
        key_handle: i32,
        message_handle: i32,
        sig_handle: i32,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: managed_verify_secp256r1")
    }
    fn managed_verify_blssignature_share(
        &mut self,
        key_handle: i32,
        message_handle: i32,
        sig_handle: i32,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: managed_verify_blssignature_share")
    }
    fn managed_verify_blsaggregated_signature(
        &mut self,
        key_handle: i32,
        message_handle: i32,
        sig_handle: i32,
    ) -> Result<i32, VMHooksError> {
        panic!("Unavailable: managed_verify_blsaggregated_signature")
    }
}
