use multiversx_chain_vm::mem_conv;
use multiversx_sc::api::{const_handles, RawHandle, SendApi, SendApiImpl};

use crate::api::{VMHooksApi, VMHooksApiBackend};

impl<VHB: VMHooksApiBackend> SendApi for VMHooksApi<VHB> {
    type SendApiImpl = Self;

    fn send_api_impl() -> Self::SendApiImpl {
        Self::api_impl()
    }
}

impl<VHB: VMHooksApiBackend> SendApiImpl for VMHooksApi<VHB> {
    fn transfer_value_execute(
        &self,
        to_handle: RawHandle,
        amount_handle: RawHandle,
        gas_limit: u64,
        endpoint_name_handle: RawHandle,
        arg_buffer_handle: RawHandle,
    ) -> Result<(), &'static [u8]> {
        let result = self.with_vm_hooks(|vh| {
            vh.managed_transfer_value_execute(
                to_handle,
                amount_handle,
                gas_limit as i64,
                endpoint_name_handle,
                arg_buffer_handle,
            )
        });
        if result == 0 {
            Ok(())
        } else {
            Err(b"transferValueExecute failed")
        }
    }

    fn multi_transfer_esdt_nft_execute(
        &self,
        to_handle: RawHandle,
        payments_handle: RawHandle,
        gas_limit: u64,
        endpoint_name_handle: RawHandle,
        arg_buffer_handle: RawHandle,
    ) -> Result<(), &'static [u8]> {
        let result = self.with_vm_hooks(|vh| {
            vh.managed_multi_transfer_esdt_nft_execute(
                to_handle,
                payments_handle,
                gas_limit as i64,
                endpoint_name_handle,
                arg_buffer_handle,
            )
        });
        if result == 0 {
            Ok(())
        } else {
            Err(b"multiTransferESDTNFTExecute failed")
        }
    }

    fn async_call_raw(
        &self,
        to_handle: RawHandle,
        egld_value_handle: RawHandle,
        endpoint_name_handle: RawHandle,
        arg_buffer_handle: RawHandle,
    ) -> ! {
        self.with_vm_hooks(|vh| {
            vh.managed_async_call(
                to_handle,
                egld_value_handle,
                endpoint_name_handle,
                arg_buffer_handle,
            )
        });

        // Although not explicit in the VM hooks interface, the method always terminates execution.
        unreachable!()
    }

    fn create_async_call_raw(
        &self,
        to_handle: RawHandle,
        egld_value_handle: RawHandle,
        endpoint_name_handle: RawHandle,
        arg_buffer_handle: RawHandle,
        success_callback: &'static str,
        error_callback: &'static str,
        gas: u64,
        extra_gas_for_callback: u64,
        callback_closure_handle: RawHandle,
    ) {
        self.with_vm_hooks(|vh| {
            mem_conv::with_mem_ptr(
                success_callback.as_bytes(),
                |success_offset, success_length| {
                    mem_conv::with_mem_ptr(
                        error_callback.as_bytes(),
                        |error_offset, error_length| {
                            vh.managed_create_async_call(
                                to_handle,
                                egld_value_handle,
                                endpoint_name_handle,
                                arg_buffer_handle,
                                success_offset,
                                success_length,
                                error_offset,
                                error_length,
                                gas as i64,
                                extra_gas_for_callback as i64,
                                callback_closure_handle,
                            );
                        },
                    )
                },
            )
        });
    }

    fn deploy_contract(
        &self,
        gas: u64,
        egld_value_handle: RawHandle,
        code_handle: RawHandle,
        code_metadata_handle: RawHandle,
        arg_buffer_handle: RawHandle,
        new_address_handle: RawHandle,
        result_handle: RawHandle,
    ) {
        self.with_vm_hooks(|vh| {
            vh.managed_create_contract(
                gas as i64,
                egld_value_handle,
                code_handle,
                code_metadata_handle,
                arg_buffer_handle,
                new_address_handle,
                result_handle,
            )
        });
    }

    fn deploy_from_source_contract(
        &self,
        gas: u64,
        egld_value_handle: RawHandle,
        source_contract_address_handle: RawHandle,
        code_metadata_handle: RawHandle,
        arg_buffer_handle: RawHandle,
        new_address_handle: RawHandle,
        result_handle: RawHandle,
    ) {
        self.with_vm_hooks(|vh| {
            vh.managed_deploy_from_source_contract(
                gas as i64,
                egld_value_handle,
                source_contract_address_handle,
                code_metadata_handle,
                arg_buffer_handle,
                new_address_handle,
                result_handle,
            )
        });
    }

    fn upgrade_from_source_contract(
        &self,
        sc_address_handle: RawHandle,
        gas: u64,
        egld_value_handle: RawHandle,
        source_contract_address_handle: RawHandle,
        code_metadata_handle: RawHandle,
        arg_buffer_handle: RawHandle,
    ) {
        // Note: the result handle is a mistake in the EI.
        // The upgrade contract operation is an async call, so no results can be returned.
        let unused_result_handle = const_handles::MBUF_TEMPORARY_1;

        self.with_vm_hooks(|vh| {
            vh.managed_upgrade_from_source_contract(
                sc_address_handle,
                gas as i64,
                egld_value_handle,
                source_contract_address_handle,
                code_metadata_handle,
                arg_buffer_handle,
                unused_result_handle,
            )
        });
    }

    fn upgrade_contract(
        &self,
        sc_address_handle: RawHandle,
        gas: u64,
        egld_value_handle: RawHandle,
        code_handle: RawHandle,
        code_metadata_handle: RawHandle,
        arg_buffer_handle: RawHandle,
    ) {
        // Note: the result handle is a mistake in the EI.
        // The upgrade contract operation is an async call, so no results can be returned.
        let unused_result_handle = const_handles::MBUF_TEMPORARY_1;

        self.with_vm_hooks(|vh| {
            vh.managed_upgrade_contract(
                sc_address_handle,
                gas as i64,
                egld_value_handle,
                code_handle,
                code_metadata_handle,
                arg_buffer_handle,
                unused_result_handle,
            )
        });
    }

    fn execute_on_dest_context_raw(
        &self,
        gas: u64,
        to_handle: RawHandle,
        egld_value_handle: RawHandle,
        endpoint_name_handle: RawHandle,
        arg_buffer_handle: RawHandle,
        result_handle: RawHandle,
    ) {
        self.with_vm_hooks(|vh| {
            vh.managed_execute_on_dest_context(
                gas as i64,
                to_handle,
                egld_value_handle,
                endpoint_name_handle,
                arg_buffer_handle,
                result_handle,
            )
        });
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
        self.with_vm_hooks(|vh| vh.clean_return_data());
    }

    fn delete_from_return_data(&self, index: usize) {
        self.with_vm_hooks(|vh| vh.delete_from_return_data(index as i32));
    }
}
