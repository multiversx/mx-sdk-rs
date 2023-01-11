use crate::api::{managed_types::big_int_api_node::unsafe_buffer_load_be_pad_right, VmApiImpl};
use alloc::vec::Vec;
use multiversx_sc::{
    api::{const_handles, ManagedTypeApi, SendApi, SendApiImpl, StaticVarApiImpl},
    types::{
        heap::{Address, ArgBuffer, BoxedBytes},
        managed_vec_from_slice_of_boxed_bytes, BigUint, CodeMetadata, EsdtTokenPayment,
        ManagedAddress, ManagedArgBuffer, ManagedBuffer, ManagedType, ManagedVec, TokenIdentifier,
    },
    HexCallDataSerializer,
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

    fn upgradeFromSourceContract(
        scAddressOffset: *const u8,
        gas: i64,
        valueOffset: *const u8,
        sourceContractAddressOffset: *const u8,
        codeMetadataOffset: *const u8,
        numArguments: i32,
        argumentsLengthOffset: *const u8,
        dataOffset: *const u8,
    );

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

    fn executeReadOnly(
        gas: i64,
        addressOffset: *const u8,
        functionOffset: *const u8,
        functionLength: i32,
        numArguments: i32,
        argumentsLengthOffset: *const u8,
        dataOffset: *const u8,
    ) -> i32;

    // managed buffer API
    fn mBufferNewFromBytes(byte_ptr: *const u8, byte_len: i32) -> i32;

    fn managedMultiTransferESDTNFTExecute(
        dstHandle: i32,
        tokenTransfersHandle: i32,
        gasLimit: i64,
        functionHandle: i32,
        argumentsHandle: i32,
    ) -> i32;
    fn managedTransferValueExecute(
        dstHandle: i32,
        valueHandle: i32,
        gasLimit: i64,
        functionHandle: i32,
        argumentsHandle: i32,
    ) -> i32;
    fn managedExecuteOnDestContext(
        gas: i64,
        addressHandle: i32,
        valueHandle: i32,
        functionHandle: i32,
        argumentsHandle: i32,
        resultHandle: i32,
    ) -> i32;
    fn managedExecuteOnSameContext(
        gas: i64,
        addressHandle: i32,
        valueHandle: i32,
        functionHandle: i32,
        argumentsHandle: i32,
        resultHandle: i32,
    ) -> i32;
    fn managedExecuteReadOnly(
        gas: i64,
        addressHandle: i32,
        functionHandle: i32,
        argumentsHandle: i32,
        resultHandle: i32,
    ) -> i32;
    fn managedCreateContract(
        gas: i64,
        valueHandle: i32,
        codeHandle: i32,
        codeMetadataHandle: i32,
        argumentsHandle: i32,
        resultAddressHandle: i32,
        resultHandle: i32,
    ) -> i32;
    fn managedDeployFromSourceContract(
        gas: i64,
        valueHandle: i32,
        addressHandle: i32,
        codeMetadataHandle: i32,
        argumentsHandle: i32,
        resultAddressHandle: i32,
        resultHandle: i32,
    ) -> i32;
    fn managedUpgradeContract(
        dstHandle: i32,
        gas: i64,
        valueHandle: i32,
        codeHandle: i32,
        codeMetadataHandle: i32,
        argumentsHandle: i32,
        resultHandle: i32,
    );
    fn managedUpgradeFromSourceContract(
        dstHandle: i32,
        gas: i64,
        valueHandle: i32,
        addressHandle: i32,
        codeMetadataHandle: i32,
        argumentsHandle: i32,
        resultHandle: i32,
    );
    fn managedAsyncCall(
        dstHandle: i32,
        valueHandle: i32,
        functionHandle: i32,
        argumentsHandle: i32,
    ) -> !;

    fn managedCreateAsyncCall(
        dstHandle: i32,
        valueHandle: i32,
        functionHandle: i32,
        argumentsHandle: i32,
        successOffset: *const u8,
        successLength: i32,
        errorOffset: *const u8,
        errorLength: i32,
        gas: i64,
        extraGasForCallback: i64,
        callbackClosureHandle: i32,
    ) -> i32;

    fn getNumReturnData() -> i32;
    #[allow(unused)]
    fn managedGetReturnData(resultID: i32, resultHandle: i32);
    fn getReturnDataSize(result_index: i32) -> i32;
    fn getReturnData(result_index: i32, dataOffset: *const u8) -> i32;

    /// Clears results propagated from nested sync calls
    fn cleanReturnData();
    fn deleteFromReturnData(resultID: i32);
}

unsafe fn code_metadata_to_buffer_handle(code_metadata: CodeMetadata) -> i32 {
    let code_metadata_bytes = code_metadata.to_byte_array();
    mBufferNewFromBytes(
        code_metadata_bytes.as_ptr(),
        code_metadata_bytes.len() as i32,
    )
}

impl SendApi for VmApiImpl {
    type SendApiImpl = VmApiImpl;

    #[inline]
    fn send_api_impl() -> Self::SendApiImpl {
        VmApiImpl {}
    }
}

impl SendApiImpl for VmApiImpl {
    fn transfer_value_legacy<M>(&self, to: &Address, amount: &BigUint<M>, data: &BoxedBytes)
    where
        M: ManagedTypeApi,
    {
        unsafe {
            let amount_bytes32_ptr = unsafe_buffer_load_be_pad_right(amount.get_raw_handle(), 32);
            let _ = transferValue(
                to.as_ptr(),
                amount_bytes32_ptr,
                data.as_ptr(),
                data.len() as i32,
            );
        }
    }

    fn transfer_value_execute<M: ManagedTypeApi>(
        &self,
        to: &ManagedAddress<M>,
        amount: &BigUint<M>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<M>,
        arg_buffer: &ManagedArgBuffer<M>,
    ) -> Result<(), &'static [u8]> {
        unsafe {
            let result = managedTransferValueExecute(
                to.get_raw_handle(),
                amount.get_raw_handle(),
                gas_limit as i64,
                endpoint_name.get_raw_handle(),
                arg_buffer.get_raw_handle(),
            );
            if result == 0 {
                Ok(())
            } else {
                Err(b"transferValueExecute failed")
            }
        }
    }

    fn transfer_value_execute_legacy<M: ManagedTypeApi>(
        &self,
        to: &Address,
        amount: &BigUint<M>,
        gas_limit: u64,
        endpoint_name: &BoxedBytes,
        arg_buffer: &ArgBuffer,
    ) -> Result<(), &'static [u8]> {
        unsafe {
            let amount_bytes32_ptr = unsafe_buffer_load_be_pad_right(amount.get_raw_handle(), 32);
            let result = transferValueExecute(
                to.as_ptr(),
                amount_bytes32_ptr,
                gas_limit as i64,
                endpoint_name.as_ptr(),
                endpoint_name.len() as i32,
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

    fn transfer_esdt_execute<M: ManagedTypeApi>(
        &self,
        to: &ManagedAddress<M>,
        token: &TokenIdentifier<M>,
        amount: &BigUint<M>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<M>,
        arg_buffer: &ManagedArgBuffer<M>,
    ) -> Result<(), &'static [u8]> {
        self.transfer_esdt_nft_execute(to, token, 0, amount, gas_limit, endpoint_name, arg_buffer)
    }

    fn transfer_esdt_execute_legacy<M: ManagedTypeApi>(
        &self,
        to: &Address,
        token: &TokenIdentifier<M>,
        amount: &BigUint<M>,
        gas_limit: u64,
        endpoint_name: &BoxedBytes,
        arg_buffer: &ArgBuffer,
    ) -> Result<(), &'static [u8]> {
        unsafe {
            let amount_bytes32_ptr = unsafe_buffer_load_be_pad_right(amount.get_raw_handle(), 32);
            let token_bytes = token.to_boxed_bytes();
            let result = transferESDTExecute(
                to.as_ptr(),
                token_bytes.as_ptr(),
                token_bytes.len() as i32,
                amount_bytes32_ptr,
                gas_limit as i64,
                endpoint_name.as_ptr(),
                endpoint_name.len() as i32,
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

    fn transfer_esdt_nft_execute<M: ManagedTypeApi>(
        &self,
        to: &ManagedAddress<M>,
        token: &TokenIdentifier<M>,
        nonce: u64,
        amount: &BigUint<M>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<M>,
        arg_buffer: &ManagedArgBuffer<M>,
    ) -> Result<(), &'static [u8]> {
        let mut payments = ManagedVec::new();
        payments.push(EsdtTokenPayment::new(token.clone(), nonce, amount.clone()));
        self.multi_transfer_esdt_nft_execute(to, &payments, gas_limit, endpoint_name, arg_buffer)
    }

    fn transfer_esdt_nft_execute_legacy<M: ManagedTypeApi>(
        &self,
        to: &Address,
        token: &TokenIdentifier<M>,
        nonce: u64,
        amount: &BigUint<M>,
        gas_limit: u64,
        endpoint_name: &BoxedBytes,
        arg_buffer: &ArgBuffer,
    ) -> Result<(), &'static [u8]> {
        unsafe {
            let amount_bytes32_ptr = unsafe_buffer_load_be_pad_right(amount.get_raw_handle(), 32);
            let token_bytes = token.to_boxed_bytes();
            let result = transferESDTNFTExecute(
                to.as_ptr(),
                token_bytes.as_ptr(),
                token_bytes.len() as i32,
                amount_bytes32_ptr,
                nonce as i64,
                gas_limit as i64,
                endpoint_name.as_ptr(),
                endpoint_name.len() as i32,
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

    fn multi_transfer_esdt_nft_execute<M: ManagedTypeApi>(
        &self,
        to: &ManagedAddress<M>,
        payments: &ManagedVec<M, EsdtTokenPayment<M>>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<M>,
        arg_buffer: &ManagedArgBuffer<M>,
    ) -> Result<(), &'static [u8]> {
        unsafe {
            let result = managedMultiTransferESDTNFTExecute(
                to.get_raw_handle(),
                payments.get_raw_handle(),
                gas_limit as i64,
                endpoint_name.get_raw_handle(),
                arg_buffer.get_raw_handle(),
            );
            if result == 0 {
                Ok(())
            } else {
                Err(b"multiTransferESDTNFTExecute failed")
            }
        }
    }

    fn multi_transfer_esdt_nft_execute_legacy<M: ManagedTypeApi>(
        &self,
        to: &Address,
        payments: &[EsdtTokenPayment<M>],
        gas_limit: u64,
        endpoint_name: &BoxedBytes,
        arg_buffer: &ArgBuffer,
    ) -> Result<(), &'static [u8]> {
        unsafe {
            let nr_transfers = payments.len();
            let mut transfer_arg_lengths = Vec::with_capacity(nr_transfers * 3);
            let mut transfer_args =
                Vec::with_capacity(nr_transfers * AVERAGE_MULTI_TRANSFER_ARG_PAIR_LENGTH);

            for token in payments {
                let token_id_bytes = token.token_identifier.to_boxed_bytes();
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
                to.as_ptr(),
                nr_transfers as i32,
                transfer_arg_lengths.as_ptr() as *const u8,
                transfer_args.as_ptr(),
                gas_limit as i64,
                endpoint_name.as_ptr(),
                endpoint_name.len() as i32,
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

    fn async_call_raw<M: ManagedTypeApi>(
        &self,
        to: &ManagedAddress<M>,
        amount: &BigUint<M>,
        endpoint_name: &ManagedBuffer<M>,
        arg_buffer: &ManagedArgBuffer<M>,
    ) -> ! {
        unsafe {
            managedAsyncCall(
                to.get_raw_handle(),
                amount.get_raw_handle(),
                endpoint_name.get_raw_handle(),
                arg_buffer.get_raw_handle(),
            )
        }
    }

    fn async_call_raw_legacy<M: ManagedTypeApi>(
        &self,
        to: &Address,
        amount: &BigUint<M>,
        endpoint_name: &BoxedBytes,
        arg_buffer: &ArgBuffer,
    ) -> ! {
        unsafe {
            let amount_bytes32_ptr = unsafe_buffer_load_be_pad_right(amount.get_raw_handle(), 32);
            let call_data =
                HexCallDataSerializer::from_arg_buffer(endpoint_name.as_slice(), arg_buffer)
                    .into_vec();
            asyncCall(
                to.as_ptr(),
                amount_bytes32_ptr,
                call_data.as_ptr(),
                call_data.len() as i32,
            )
        }
    }

    fn create_async_call_raw(
        &self,
        to: Self::ManagedBufferHandle,
        amount: Self::BigIntHandle,
        endpoint_name: Self::ManagedBufferHandle,
        arg_buffer: Self::ManagedBufferHandle,
        success_callback: &'static str,
        error_callback: &'static str,
        gas: u64,
        extra_gas_for_callback: u64,
        callback_closure: Self::ManagedBufferHandle,
    ) {
        unsafe {
            let _ = managedCreateAsyncCall(
                to,
                amount,
                endpoint_name,
                arg_buffer,
                success_callback.as_ptr(),
                success_callback.len() as i32,
                error_callback.as_ptr(),
                error_callback.len() as i32,
                gas as i64,
                extra_gas_for_callback as i64,
                callback_closure,
            );
        }
    }

    fn deploy_contract<M: ManagedTypeApi>(
        &self,
        gas: u64,
        amount: &BigUint<M>,
        code: &ManagedBuffer<M>,
        code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<M>,
    ) -> (ManagedAddress<M>, ManagedVec<M, ManagedBuffer<M>>) {
        unsafe {
            let code_metadata_handle = code_metadata_to_buffer_handle(code_metadata);
            let new_address_handle = self.next_handle();
            let result_handle = self.next_handle();
            let _ = managedCreateContract(
                gas as i64,
                amount.get_raw_handle(),
                code.get_raw_handle(),
                code_metadata_handle,
                arg_buffer.get_raw_handle(),
                new_address_handle,
                result_handle,
            );

            let new_managed_address = ManagedAddress::from_raw_handle(new_address_handle);
            let results = ManagedVec::from_raw_handle(result_handle);

            (new_managed_address, results)
        }
    }

    fn deploy_contract_legacy<M: ManagedTypeApi>(
        &self,
        gas: u64,
        amount: &BigUint<M>,
        code: &BoxedBytes,
        code_metadata: CodeMetadata,
        arg_buffer: &ArgBuffer,
    ) -> (ManagedAddress<M>, ManagedVec<M, ManagedBuffer<M>>) {
        let mut new_address = Address::zero();
        unsafe {
            let num_return_data_before = getNumReturnData();

            let amount_bytes32_ptr = unsafe_buffer_load_be_pad_right(amount.get_raw_handle(), 32);
            let code_metadata_bytes = code_metadata.to_byte_array();
            let _ = createContract(
                gas as i64,
                amount_bytes32_ptr,
                code.as_ptr(),
                code_metadata_bytes.as_ptr(),
                code.len() as i32,
                new_address.as_mut_ptr(),
                arg_buffer.num_args() as i32,
                arg_buffer.arg_lengths_bytes_ptr(),
                arg_buffer.arg_data_ptr(),
            );

            let num_return_data_after = getNumReturnData();
            let result_bytes = get_return_data_range(num_return_data_before, num_return_data_after);
            let results = managed_vec_from_slice_of_boxed_bytes(result_bytes.as_slice());

            (ManagedAddress::from(new_address), results)
        }
    }

    fn deploy_from_source_contract<M: ManagedTypeApi>(
        &self,
        gas: u64,
        amount: &BigUint<M>,
        source_contract_address: &ManagedAddress<M>,
        code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<M>,
    ) -> (ManagedAddress<M>, ManagedVec<M, ManagedBuffer<M>>) {
        unsafe {
            let code_metadata_handle = code_metadata_to_buffer_handle(code_metadata);
            let new_address_handle = self.next_handle();
            let result_handle = self.next_handle();
            let _ = managedDeployFromSourceContract(
                gas as i64,
                amount.get_raw_handle(),
                source_contract_address.get_raw_handle(),
                code_metadata_handle,
                arg_buffer.get_raw_handle(),
                new_address_handle,
                result_handle,
            );

            let new_managed_address = ManagedAddress::from_raw_handle(new_address_handle);
            let results = ManagedVec::from_raw_handle(result_handle);

            (new_managed_address, results)
        }
    }

    fn deploy_from_source_contract_legacy<M: ManagedTypeApi>(
        &self,
        gas: u64,
        amount: &BigUint<M>,
        source_contract_address: &Address,
        code_metadata: CodeMetadata,
        arg_buffer: &ArgBuffer,
    ) -> (ManagedAddress<M>, ManagedVec<M, ManagedBuffer<M>>) {
        let mut new_address = Address::zero();
        unsafe {
            let num_return_data_before = getNumReturnData();
            let amount_bytes32_ptr = unsafe_buffer_load_be_pad_right(amount.get_raw_handle(), 32);
            let code_metadata_bytes = code_metadata.to_byte_array();
            let _ = deployFromSourceContract(
                gas as i64,
                amount_bytes32_ptr,
                source_contract_address.as_ptr(),
                code_metadata_bytes.as_ptr(),
                new_address.as_mut_ptr(),
                arg_buffer.num_args() as i32,
                arg_buffer.arg_lengths_bytes_ptr(),
                arg_buffer.arg_data_ptr(),
            );

            let num_return_data_after = getNumReturnData();
            let result_bytes = get_return_data_range(num_return_data_before, num_return_data_after);
            let results = managed_vec_from_slice_of_boxed_bytes(result_bytes.as_slice());

            (ManagedAddress::from(new_address), results)
        }
    }

    fn upgrade_from_source_contract<M: ManagedTypeApi>(
        &self,
        sc_address: &ManagedAddress<M>,
        gas: u64,
        amount: &BigUint<M>,
        source_contract_address: &ManagedAddress<M>,
        code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<M>,
    ) {
        unsafe {
            let code_metadata_handle = code_metadata_to_buffer_handle(code_metadata);
            let unused_result_handle = const_handles::MBUF_TEMPORARY_1;
            managedUpgradeFromSourceContract(
                sc_address.get_raw_handle(),
                gas as i64,
                amount.get_raw_handle(),
                source_contract_address.get_raw_handle(),
                code_metadata_handle,
                arg_buffer.get_raw_handle(),
                unused_result_handle,
            );
        }
    }

    fn upgrade_from_source_contract_legacy<M: ManagedTypeApi>(
        &self,
        sc_address: &Address,
        gas: u64,
        amount: &BigUint<M>,
        source_contract_address: &Address,
        code_metadata: CodeMetadata,
        arg_buffer: &ArgBuffer,
    ) {
        unsafe {
            let amount_bytes32_ptr = unsafe_buffer_load_be_pad_right(amount.get_raw_handle(), 32);
            let code_metadata_bytes = code_metadata.to_byte_array();
            upgradeFromSourceContract(
                sc_address.as_ptr(),
                gas as i64,
                amount_bytes32_ptr,
                source_contract_address.as_ptr(),
                code_metadata_bytes.as_ptr(),
                arg_buffer.num_args() as i32,
                arg_buffer.arg_lengths_bytes_ptr(),
                arg_buffer.arg_data_ptr(),
            );
        }
    }

    fn upgrade_contract<M: ManagedTypeApi>(
        &self,
        sc_address: &ManagedAddress<M>,
        gas: u64,
        amount: &BigUint<M>,
        code: &ManagedBuffer<M>,
        code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<M>,
    ) {
        unsafe {
            let code_metadata_handle = code_metadata_to_buffer_handle(code_metadata);
            let unused_result_handle = const_handles::MBUF_TEMPORARY_1;
            managedUpgradeContract(
                sc_address.get_raw_handle(),
                gas as i64,
                amount.get_raw_handle(),
                code.get_raw_handle(),
                code_metadata_handle,
                arg_buffer.get_raw_handle(),
                unused_result_handle,
            );

            // Note: the result handle is a mistake in the EI.
            // The upgrade contract operation is an async call, so no results can be returned.
        }
    }

    fn upgrade_contract_legacy<M: ManagedTypeApi>(
        &self,
        sc_address: &Address,
        gas: u64,
        amount: &BigUint<M>,
        code: &BoxedBytes,
        code_metadata: CodeMetadata,
        arg_buffer: &ArgBuffer,
    ) {
        unsafe {
            let amount_bytes32_ptr = unsafe_buffer_load_be_pad_right(amount.get_raw_handle(), 32);
            let code_metadata_bytes = code_metadata.to_byte_array();
            upgradeContract(
                sc_address.as_ptr(),
                gas as i64,
                amount_bytes32_ptr,
                code.as_ptr(),
                code_metadata_bytes.as_ptr(),
                code.len() as i32,
                arg_buffer.num_args() as i32,
                arg_buffer.arg_lengths_bytes_ptr(),
                arg_buffer.arg_data_ptr(),
            );
        }
    }

    fn execute_on_dest_context_raw<M: ManagedTypeApi>(
        &self,
        gas: u64,
        to: &ManagedAddress<M>,
        amount: &BigUint<M>,
        endpoint_name: &ManagedBuffer<M>,
        arg_buffer: &ManagedArgBuffer<M>,
    ) -> ManagedVec<M, ManagedBuffer<M>> {
        unsafe {
            let result_handle = self.next_handle();
            let _ = managedExecuteOnDestContext(
                gas as i64,
                to.get_raw_handle(),
                amount.get_raw_handle(),
                endpoint_name.get_raw_handle(),
                arg_buffer.get_raw_handle(),
                result_handle,
            );

            ManagedVec::from_raw_handle(result_handle)
        }
    }

    fn execute_on_dest_context_raw_legacy<M: ManagedTypeApi>(
        &self,
        gas: u64,
        to: &Address,
        amount: &BigUint<M>,
        endpoint_name: &BoxedBytes,
        arg_buffer: &ArgBuffer,
    ) -> ManagedVec<M, ManagedBuffer<M>> {
        unsafe {
            let num_return_data_before = getNumReturnData();
            let amount_bytes32_ptr = unsafe_buffer_load_be_pad_right(amount.get_raw_handle(), 32);
            let _ = executeOnDestContext(
                gas as i64,
                to.as_ptr(),
                amount_bytes32_ptr,
                endpoint_name.as_ptr(),
                endpoint_name.len() as i32,
                arg_buffer.num_args() as i32,
                arg_buffer.arg_lengths_bytes_ptr(),
                arg_buffer.arg_data_ptr(),
            );

            let num_return_data_after = getNumReturnData();
            let result_bytes = get_return_data_range(num_return_data_before, num_return_data_after);
            managed_vec_from_slice_of_boxed_bytes(result_bytes.as_slice())
        }
    }

    fn execute_on_same_context_raw<M: ManagedTypeApi>(
        &self,
        gas: u64,
        to: &ManagedAddress<M>,
        amount: &BigUint<M>,
        endpoint_name: &ManagedBuffer<M>,
        arg_buffer: &ManagedArgBuffer<M>,
    ) -> ManagedVec<M, ManagedBuffer<M>> {
        unsafe {
            let result_handle = self.next_handle();

            let _ = managedExecuteOnSameContext(
                gas as i64,
                to.get_raw_handle(),
                amount.get_raw_handle(),
                endpoint_name.get_raw_handle(),
                arg_buffer.get_raw_handle(),
                result_handle,
            );

            ManagedVec::from_raw_handle(result_handle)
        }
    }

    fn execute_on_same_context_raw_legacy<M: ManagedTypeApi>(
        &self,
        gas: u64,
        to: &Address,
        amount: &BigUint<M>,
        endpoint_name: &BoxedBytes,
        arg_buffer: &ArgBuffer,
    ) -> ManagedVec<M, ManagedBuffer<M>> {
        unsafe {
            let num_return_data_before = getNumReturnData();
            let amount_bytes32_ptr = unsafe_buffer_load_be_pad_right(amount.get_raw_handle(), 32);
            let _ = executeOnSameContext(
                gas as i64,
                to.as_ptr(),
                amount_bytes32_ptr,
                endpoint_name.as_ptr(),
                endpoint_name.len() as i32,
                arg_buffer.num_args() as i32,
                arg_buffer.arg_lengths_bytes_ptr(),
                arg_buffer.arg_data_ptr(),
            );

            let num_return_data_after = getNumReturnData();
            let result_bytes = get_return_data_range(num_return_data_before, num_return_data_after);
            managed_vec_from_slice_of_boxed_bytes(result_bytes.as_slice())
        }
    }

    fn execute_on_dest_context_readonly_raw<M: ManagedTypeApi>(
        &self,
        gas: u64,
        to: &ManagedAddress<M>,
        endpoint_name: &ManagedBuffer<M>,
        arg_buffer: &ManagedArgBuffer<M>,
    ) -> ManagedVec<M, ManagedBuffer<M>> {
        unsafe {
            let result_handle = self.next_handle();

            let _ = managedExecuteReadOnly(
                gas as i64,
                to.get_raw_handle(),
                endpoint_name.get_raw_handle(),
                arg_buffer.get_raw_handle(),
                result_handle,
            );

            ManagedVec::from_raw_handle(result_handle)
        }
    }

    fn execute_on_dest_context_readonly_raw_legacy<M: ManagedTypeApi>(
        &self,
        gas: u64,
        to: &Address,
        endpoint_name: &BoxedBytes,
        arg_buffer: &ArgBuffer,
    ) -> ManagedVec<M, ManagedBuffer<M>> {
        unsafe {
            let num_return_data_before = getNumReturnData();
            let _ = executeReadOnly(
                gas as i64,
                to.as_ptr(),
                endpoint_name.as_ptr(),
                endpoint_name.len() as i32,
                arg_buffer.num_args() as i32,
                arg_buffer.arg_lengths_bytes_ptr(),
                arg_buffer.arg_data_ptr(),
            );

            let num_return_data_after = getNumReturnData();
            let result_bytes = get_return_data_range(num_return_data_before, num_return_data_after);
            managed_vec_from_slice_of_boxed_bytes(result_bytes.as_slice())
        }
    }

    fn clean_return_data(&self) {
        unsafe {
            cleanReturnData();
        }
    }

    fn delete_from_return_data(&self, index: usize) {
        unsafe {
            deleteFromReturnData(index as i32);
        }
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
