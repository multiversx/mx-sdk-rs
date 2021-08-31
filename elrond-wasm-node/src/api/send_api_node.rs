use crate::api::managed_types::unsafe_buffer_load_be_pad_right;
use crate::ArwenApiImpl;
use alloc::vec::Vec;
use elrond_wasm::api::{BlockchainApi, SendApi, StorageReadApi, StorageWriteApi};
use elrond_wasm::types::{
    Address, ArgBuffer, BigUint, BoxedBytes, CodeMetadata, ManagedType, TokenIdentifier,
};

// Token ID + nonce + amount, as bytes
const AVERAGE_MULTI_TRANSFER_ARG_PAIR_LENGTH: usize = 15 + 2 + 8;

extern "C" {
    fn transferValue(
        dstOffset: *const u8,
        valueOffset: *const u8,
        dataOffset: *const u8,
        dataLength: i32,
    ) -> i32;

    fn transferValueExecute(
        dstOffset: *const u8,
        valueOffset: *const u8,
        gasLimit: i64,
        functionOffset: *const u8,
        functionLength: i32,
        numArguments: i32,
        argumentsLengthOffset: *const u8,
        dataOffset: *const u8,
    ) -> i32;

    fn transferESDTExecute(
        dstOffset: *const u8,
        tokenIdOffset: *const u8,
        tokenIdLen: i32,
        valueOffset: *const u8,
        gasLimit: i64,
        functionOffset: *const u8,
        functionLength: i32,
        numArguments: i32,
        argumentsLengthOffset: *const u8,
        dataOffset: *const u8,
    ) -> i32;

    fn transferESDTNFTExecute(
        dstOffset: *const u8,
        tokenIdOffset: *const u8,
        tokenIdLen: i32,
        valueOffset: *const u8,
        nonce: i64,
        gasLimit: i64,
        functionOffset: *const u8,
        functionLength: i32,
        numArguments: i32,
        argumentsLengthOffset: *const u8,
        dataOffset: *const u8,
    ) -> i32;

    fn multiTransferESDTNFTExecute(
        dstOffset: *const u8,
        numTokenTransfers: i32,
        tokenTransfersArgsLengthOffset: *const u8,
        tokenTransferDataOffset: *const u8,
        gasLimit: i64,
        functionOffset: *const u8,
        functionLength: i32,
        numArguments: i32,
        argumentsLengthOffset: *const u8,
        dataOffset: *const u8,
    ) -> i32;

    fn asyncCall(
        dstOffset: *const u8,
        valueOffset: *const u8,
        dataOffset: *const u8,
        length: i32,
    ) -> !;

    fn createContract(
        gas: i64,
        valueOffset: *const u8,
        codeOffset: *const u8,
        codeMetadataOffset: *const u8,
        codeLength: i32,
        resultOffset: *const u8,
        numArguments: i32,
        argumentsLengthOffset: *const u8,
        dataOffset: *const u8,
    ) -> i32;

    fn deployFromSourceContract(
        gas: i64,
        valueOffset: *const u8,
        sourceContractAddressOffset: *const u8,
        codeMetadataOffset: *const u8,
        resultAddressOffset: *const u8,
        numArguments: i32,
        argumentsLengthOffset: *const u8,
        dataOffset: *const u8,
    ) -> i32;

    fn upgradeContract(
        scAddressOffset: *const u8,
        gas: i64,
        valueOffset: *const u8,
        codeOffset: *const u8,
        codeMetadataOffset: *const u8,
        codeLength: i32,
        numArguments: i32,
        argumentsLengthOffset: *const u8,
        dataOffset: *const u8,
    );

    fn executeOnDestContext(
        gas: i64,
        addressOffset: *const u8,
        valueOffset: *const u8,
        functionOffset: *const u8,
        functionLength: i32,
        numArguments: i32,
        argumentsLengthOffset: *const u8,
        dataOffset: *const u8,
    ) -> i32;

    fn executeOnDestContextByCaller(
        gas: i64,
        addressOffset: *const u8,
        valueOffset: *const u8,
        functionOffset: *const u8,
        functionLength: i32,
        numArguments: i32,
        argumentsLengthOffset: *const u8,
        dataOffset: *const u8,
    ) -> i32;

    fn executeOnSameContext(
        gas: i64,
        addressOffset: *const u8,
        valueOffset: *const u8,
        functionOffset: *const u8,
        functionLength: i32,
        numArguments: i32,
        argumentsLengthOffset: *const u8,
        dataOffset: *const u8,
    ) -> i32;

    fn getNumReturnData() -> i32;
    fn getReturnDataSize(result_index: i32) -> i32;
    fn getReturnData(result_index: i32, dataOffset: *const u8) -> i32;
}

impl SendApi for ArwenApiImpl {
    type ProxyTypeManager = Self;
    type ProxyStorage = Self;
    type ErrorApi = Self;
    type BlockchainApi = Self;

    #[inline]
    fn type_manager(&self) -> Self::ProxyTypeManager {
        self.clone()
    }

    #[inline]
    fn error_api(&self) -> Self::ErrorApi {
        self.clone()
    }

    #[inline]
    fn blockchain(&self) -> Self::BlockchainApi {
        self.clone()
    }

    fn direct_egld(&self, to: &Address, amount: &BigUint<Self::ProxyTypeManager>, data: &[u8]) {
        unsafe {
            let amount_bytes32_ptr = unsafe_buffer_load_be_pad_right(amount.get_raw_handle(), 32);
            let _ = transferValue(
                to.as_ref().as_ptr(),
                amount_bytes32_ptr,
                data.as_ptr(),
                data.len() as i32,
            );
        }
    }

    fn direct_egld_execute(
        &self,
        to: &Address,
        amount: &BigUint<Self::ProxyTypeManager>,
        gas_limit: u64,
        function: &[u8],
        arg_buffer: &ArgBuffer,
    ) -> Result<(), &'static [u8]> {
        unsafe {
            let amount_bytes32_ptr = unsafe_buffer_load_be_pad_right(amount.get_raw_handle(), 32);
            let result = transferValueExecute(
                to.as_ref().as_ptr(),
                amount_bytes32_ptr,
                gas_limit as i64,
                function.as_ptr(),
                function.len() as i32,
                arg_buffer.num_args() as i32,
                arg_buffer.arg_lengths_bytes_ptr(),
                arg_buffer.arg_data_ptr(),
            );
            if result == 0 {
                Ok(())
            } else {
                Err(b"transferValueExecute failed")
            }
        }
    }

    fn direct_esdt_execute(
        &self,
        to: &Address,
        token: &TokenIdentifier<Self::ProxyTypeManager>,
        amount: &BigUint<Self::ProxyTypeManager>,
        gas_limit: u64,
        function: &[u8],
        arg_buffer: &ArgBuffer,
    ) -> Result<(), &'static [u8]> {
        unsafe {
            let amount_bytes32_ptr = unsafe_buffer_load_be_pad_right(amount.get_raw_handle(), 32);
            let result = transferESDTExecute(
                to.as_ref().as_ptr(),
                token.to_esdt_identifier().as_ptr(),
                token.len() as i32,
                amount_bytes32_ptr,
                gas_limit as i64,
                function.as_ptr(),
                function.len() as i32,
                arg_buffer.num_args() as i32,
                arg_buffer.arg_lengths_bytes_ptr(),
                arg_buffer.arg_data_ptr(),
            );
            if result == 0 {
                Ok(())
            } else {
                Err(b"transferESDTExecute failed")
            }
        }
    }

    fn direct_esdt_nft_execute(
        &self,
        to: &Address,
        token: &TokenIdentifier<Self::ProxyTypeManager>,
        nonce: u64,
        amount: &BigUint<Self::ProxyTypeManager>,
        gas_limit: u64,
        function: &[u8],
        arg_buffer: &ArgBuffer,
    ) -> Result<(), &'static [u8]> {
        unsafe {
            let amount_bytes32_ptr = unsafe_buffer_load_be_pad_right(amount.get_raw_handle(), 32);
            let result = transferESDTNFTExecute(
                to.as_ref().as_ptr(),
                token.to_esdt_identifier().as_ptr(),
                token.len() as i32,
                amount_bytes32_ptr,
                nonce as i64,
                gas_limit as i64,
                function.as_ptr(),
                function.len() as i32,
                arg_buffer.num_args() as i32,
                arg_buffer.arg_lengths_bytes_ptr(),
                arg_buffer.arg_data_ptr(),
            );
            if result == 0 {
                Ok(())
            } else {
                Err(b"transferESDTNFTExecute failed")
            }
        }
    }

    fn direct_multi_esdt_transfer_execute(
        &self,
        to: &Address,
        tokens: &[elrond_wasm::types::EsdtTokenPayment<Self::ProxyTypeManager>],
        gas_limit: u64,
        function: &[u8],
        arg_buffer: &ArgBuffer,
    ) -> Result<(), &'static [u8]> {
        unsafe {
            let nr_transfers = tokens.len();
            let mut transfer_arg_lengths = Vec::with_capacity(nr_transfers * 3);
            let mut transfer_args =
                Vec::with_capacity(nr_transfers * AVERAGE_MULTI_TRANSFER_ARG_PAIR_LENGTH);

            for token in tokens {
                let token_id_bytes = token.token_name.to_esdt_identifier();
                let nonce_bytes = &token.token_nonce.to_be_bytes()[..]; // TODO: Maybe top-encode here instead
                let amount_bytes = &token.amount.to_bytes_be();

                transfer_arg_lengths.push(token_id_bytes.len() as i32);
                transfer_arg_lengths.push(nonce_bytes.len() as i32);
                transfer_arg_lengths.push(amount_bytes.len() as i32);

                transfer_args.extend_from_slice(token_id_bytes.as_slice());
                transfer_args.extend_from_slice(nonce_bytes);
                transfer_args.extend_from_slice(amount_bytes.as_slice());
            }

            let result = multiTransferESDTNFTExecute(
                to.as_ref().as_ptr(),
                nr_transfers as i32,
                transfer_arg_lengths.as_ptr() as *const u8,
                transfer_args.as_ptr(),
                gas_limit as i64,
                function.as_ptr(),
                function.len() as i32,
                arg_buffer.num_args() as i32,
                arg_buffer.arg_lengths_bytes_ptr(),
                arg_buffer.arg_data_ptr(),
            );
            if result == 0 {
                Ok(())
            } else {
                Err(b"multiTransferESDTNFTExecute failed")
            }
        }
    }

    fn async_call_raw(
        &self,
        to: &Address,
        amount: &BigUint<Self::ProxyTypeManager>,
        data: &[u8],
    ) -> ! {
        unsafe {
            let amount_bytes32_ptr = unsafe_buffer_load_be_pad_right(amount.get_raw_handle(), 32);
            asyncCall(
                to.as_ref().as_ptr(),
                amount_bytes32_ptr,
                data.as_ptr(),
                data.len() as i32,
            )
        }
    }

    fn deploy_contract(
        &self,
        gas: u64,
        amount: &BigUint<Self::ProxyTypeManager>,
        code: &BoxedBytes,
        code_metadata: CodeMetadata,
        arg_buffer: &ArgBuffer,
    ) -> Option<Address> {
        let mut new_address = Address::zero();
        unsafe {
            let amount_bytes32_ptr = unsafe_buffer_load_be_pad_right(amount.get_raw_handle(), 32);
            let _ = createContract(
                gas as i64,
                amount_bytes32_ptr,
                code.as_ptr(),
                code_metadata.as_ptr(),
                code.len() as i32,
                new_address.as_mut_ptr(),
                arg_buffer.num_args() as i32,
                arg_buffer.arg_lengths_bytes_ptr(),
                arg_buffer.arg_data_ptr(),
            );
        }
        if new_address.is_zero() {
            None
        } else {
            Some(new_address)
        }
    }

    fn deploy_from_source_contract(
        &self,
        gas: u64,
        amount: &BigUint<Self::ProxyTypeManager>,
        source_contract_address: &Address,
        code_metadata: CodeMetadata,
        arg_buffer: &ArgBuffer,
    ) -> Option<Address> {
        let mut new_address = Address::zero();
        unsafe {
            let amount_bytes32_ptr = unsafe_buffer_load_be_pad_right(amount.get_raw_handle(), 32);
            let _ = deployFromSourceContract(
                gas as i64,
                amount_bytes32_ptr,
                source_contract_address.as_ptr(),
                code_metadata.as_ptr(),
                new_address.as_mut_ptr(),
                arg_buffer.num_args() as i32,
                arg_buffer.arg_lengths_bytes_ptr(),
                arg_buffer.arg_data_ptr(),
            );
        }
        if new_address.is_zero() {
            None
        } else {
            Some(new_address)
        }
    }

    fn upgrade_contract(
        &self,
        sc_address: &Address,
        gas: u64,
        amount: &BigUint<Self::ProxyTypeManager>,
        code: &BoxedBytes,
        code_metadata: CodeMetadata,
        arg_buffer: &ArgBuffer,
    ) {
        unsafe {
            let amount_bytes32_ptr = unsafe_buffer_load_be_pad_right(amount.get_raw_handle(), 32);
            upgradeContract(
                sc_address.as_ref().as_ptr(),
                gas as i64,
                amount_bytes32_ptr,
                code.as_ptr(),
                code_metadata.as_ptr(),
                code.len() as i32,
                arg_buffer.num_args() as i32,
                arg_buffer.arg_lengths_bytes_ptr(),
                arg_buffer.arg_data_ptr(),
            );
        }
    }

    fn execute_on_dest_context_raw(
        &self,
        gas: u64,
        address: &Address,
        amount: &BigUint<Self::ProxyTypeManager>,
        function: &[u8],
        arg_buffer: &ArgBuffer,
    ) -> Vec<BoxedBytes> {
        unsafe {
            let num_return_data_before = getNumReturnData();

            let amount_bytes32_ptr = unsafe_buffer_load_be_pad_right(amount.get_raw_handle(), 32);
            let _ = executeOnDestContext(
                gas as i64,
                address.as_ref().as_ptr(),
                amount_bytes32_ptr,
                function.as_ptr(),
                function.len() as i32,
                arg_buffer.num_args() as i32,
                arg_buffer.arg_lengths_bytes_ptr(),
                arg_buffer.arg_data_ptr(),
            );

            let num_return_data_after = getNumReturnData();
            get_return_data_range(num_return_data_before, num_return_data_after)
        }
    }

    fn execute_on_dest_context_raw_custom_result_range<F>(
        &self,
        gas: u64,
        address: &Address,
        amount: &BigUint<Self::ProxyTypeManager>,
        function: &[u8],
        arg_buffer: &ArgBuffer,
        range_closure: F,
    ) -> Vec<BoxedBytes>
    where
        F: FnOnce(usize, usize) -> (usize, usize),
    {
        unsafe {
            let num_return_data_before = getNumReturnData();

            let amount_bytes32_ptr = unsafe_buffer_load_be_pad_right(amount.get_raw_handle(), 32);
            let _ = executeOnDestContext(
                gas as i64,
                address.as_ref().as_ptr(),
                amount_bytes32_ptr,
                function.as_ptr(),
                function.len() as i32,
                arg_buffer.num_args() as i32,
                arg_buffer.arg_lengths_bytes_ptr(),
                arg_buffer.arg_data_ptr(),
            );

            let num_return_data_after = getNumReturnData();
            let (result_start_index, result_end_index) = range_closure(
                num_return_data_before as usize,
                num_return_data_after as usize,
            );
            get_return_data_range(result_start_index as i32, result_end_index as i32)
        }
    }

    fn execute_on_dest_context_by_caller_raw(
        &self,
        gas: u64,
        address: &Address,
        amount: &BigUint<Self::ProxyTypeManager>,
        function: &[u8],
        arg_buffer: &ArgBuffer,
    ) -> Vec<BoxedBytes> {
        unsafe {
            let num_return_data_before = getNumReturnData();

            let amount_bytes32_ptr = unsafe_buffer_load_be_pad_right(amount.get_raw_handle(), 32);
            let _ = executeOnDestContextByCaller(
                gas as i64,
                address.as_ref().as_ptr(),
                amount_bytes32_ptr,
                function.as_ptr(),
                function.len() as i32,
                arg_buffer.num_args() as i32,
                arg_buffer.arg_lengths_bytes_ptr(),
                arg_buffer.arg_data_ptr(),
            );

            let num_return_data_after = getNumReturnData();
            get_return_data_range(num_return_data_before, num_return_data_after)
        }
    }

    fn execute_on_same_context_raw(
        &self,
        gas: u64,
        address: &Address,
        amount: &BigUint<Self::ProxyTypeManager>,
        function: &[u8],
        arg_buffer: &ArgBuffer,
    ) {
        unsafe {
            let amount_bytes32_ptr = unsafe_buffer_load_be_pad_right(amount.get_raw_handle(), 32);
            let _ = executeOnSameContext(
                gas as i64,
                address.as_ref().as_ptr(),
                amount_bytes32_ptr,
                function.as_ptr(),
                function.len() as i32,
                arg_buffer.num_args() as i32,
                arg_buffer.arg_lengths_bytes_ptr(),
                arg_buffer.arg_data_ptr(),
            );
        }
    }

    fn storage_store_tx_hash_key(&self, data: &[u8]) {
        let tx_hash = self.get_tx_hash();
        self.storage_store_slice_u8(tx_hash.as_bytes(), data);
    }

    fn storage_load_tx_hash_key(&self) -> BoxedBytes {
        let tx_hash = self.get_tx_hash();
        self.storage_load_boxed_bytes(tx_hash.as_bytes())
    }

    fn call_local_esdt_built_in_function(
        &self,
        gas: u64,
        function: &[u8],
        arg_buffer: &ArgBuffer,
    ) -> Vec<BoxedBytes> {
        // account-level built-in function, so the destination address is the contract itself
        let own_address = BlockchainApi::get_sc_address(self);

        self.execute_on_dest_context_raw(
            gas,
            &own_address,
            &BigUint::zero(self.clone()),
            function,
            arg_buffer,
        )
    }
}

/// Retrieves already pushed results, via `finish`.
/// `from_index` is inclusive.
/// `to_index` is exclusive.
unsafe fn get_return_data_range(from_index: i32, to_index: i32) -> Vec<BoxedBytes> {
    let num_results = to_index - from_index;
    let mut result = Vec::with_capacity(num_results as usize);
    if num_results > 0 {
        for index in from_index..to_index {
            result.push(get_return_data(index));
        }
    }
    result
}

/// Retrieves already pushed individual result at given index, via `finish`.
unsafe fn get_return_data(return_index: i32) -> BoxedBytes {
    let len = getReturnDataSize(return_index);
    let mut res = BoxedBytes::allocate(len as usize);
    if len > 0 {
        let _ = getReturnData(return_index, res.as_mut_ptr());
    }
    res
}
