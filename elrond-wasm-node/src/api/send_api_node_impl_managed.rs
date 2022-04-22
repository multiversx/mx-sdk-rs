use crate::VmApiImpl;
use elrond_wasm::{
    api::{
        const_handles, BlockchainApi, BlockchainApiImpl, Handle, ManagedTypeApi, SendApiImpl,
        StaticVarApiImpl,
    },
    types::{
        BigUint, CodeMetadata, EsdtTokenPayment, ManagedAddress, ManagedArgBuffer, ManagedBuffer,
        ManagedType, ManagedVec, TokenIdentifier,
    },
};

#[allow(unused)]
extern "C" {
    // managed buffer API
    fn mBufferNew() -> i32;
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
    fn managedExecuteOnDestContextByCaller(
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
    ) -> i32;

    /// Allows us to filter results from nested sync call
    fn getNumReturnData() -> i32;
    fn managedGetReturnData(resultID: i32, resultHandle: i32);

    /// Clears results propagated from nested sync calls
    fn cleanReturnData();
    fn deleteFromReturnData(resultID: i32);
}

unsafe fn code_metadata_to_buffer_handle(code_metadata: CodeMetadata) -> Handle {
    let code_metadata_bytes = code_metadata.to_byte_array();
    mBufferNewFromBytes(
        code_metadata_bytes.as_ptr(),
        code_metadata_bytes.len() as i32,
    )
}

impl SendApiImpl for VmApiImpl {
    fn direct_egld<M, D>(&self, to: &ManagedAddress<M>, amount: &BigUint<M>, data: D)
    where
        M: ManagedTypeApi,
        D: Into<ManagedBuffer<M>>,
    {
        let data_buffer = data.into();
        unsafe {
            let empty_arguments_handle = mBufferNew();

            let _ = managedTransferValueExecute(
                to.get_raw_handle(),
                amount.get_raw_handle(),
                0,
                data_buffer.get_raw_handle(),
                empty_arguments_handle,
            );
        }
    }

    fn direct_egld_execute<M: ManagedTypeApi>(
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

    fn direct_esdt_execute<M: ManagedTypeApi>(
        &self,
        to: &ManagedAddress<M>,
        token: &TokenIdentifier<M>,
        amount: &BigUint<M>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<M>,
        arg_buffer: &ManagedArgBuffer<M>,
    ) -> Result<(), &'static [u8]> {
        self.direct_esdt_nft_execute(to, token, 0, amount, gas_limit, endpoint_name, arg_buffer)
    }

    fn direct_esdt_nft_execute<M: ManagedTypeApi>(
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
        self.direct_multi_esdt_transfer_execute(to, &payments, gas_limit, endpoint_name, arg_buffer)
    }

    fn direct_multi_esdt_transfer_execute<M: ManagedTypeApi>(
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

    fn create_async_call_raw<M: ManagedTypeApi>(
        &self,
        to: &ManagedAddress<M>,
        amount: &BigUint<M>,
        endpoint_name: &ManagedBuffer<M>,
        success: &'static [u8],
        error: &'static [u8],
        gas: u64,
        extra_gas_for_callback: u64,
        arg_buffer: &ManagedArgBuffer<M>,
    ) {
        unsafe {
            let _ = managedCreateAsyncCall(
                to.get_raw_handle(),
                amount.get_raw_handle(),
                endpoint_name.get_raw_handle(),
                arg_buffer.get_raw_handle(),
                success.as_ptr(),
                success.len() as i32,
                error.as_ptr(),
                error.len() as i32,
                gas as i64,
                extra_gas_for_callback as i64,
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

    fn execute_on_dest_context_by_caller_raw<M: ManagedTypeApi>(
        &self,
        gas: u64,
        to: &ManagedAddress<M>,
        amount: &BigUint<M>,
        endpoint_name: &ManagedBuffer<M>,
        arg_buffer: &ManagedArgBuffer<M>,
    ) -> ManagedVec<M, ManagedBuffer<M>> {
        unsafe {
            let result_handle = self.next_handle();

            let _ = managedExecuteOnDestContextByCaller(
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

    fn call_local_esdt_built_in_function<M: ManagedTypeApi>(
        &self,
        gas: u64,
        function_name: &ManagedBuffer<M>,
        arg_buffer: &ManagedArgBuffer<M>,
    ) -> ManagedVec<M, ManagedBuffer<M>> {
        // account-level built-in function, so the destination address is the contract itself
        let own_address_handle = const_handles::MBUF_TEMPORARY_1;
        VmApiImpl::blockchain_api_impl().load_sc_address_managed(own_address_handle);

        let results = self.execute_on_dest_context_raw(
            gas,
            &ManagedAddress::from_raw_handle(own_address_handle),
            &BigUint::zero(),
            function_name,
            arg_buffer,
        );

        self.clean_return_data();

        results
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
