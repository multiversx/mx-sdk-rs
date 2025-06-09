use crate::{
    chain_core::builtin_func_names::{
        ESDT_MULTI_TRANSFER_FUNC_NAME, ESDT_NFT_TRANSFER_FUNC_NAME, ESDT_TRANSFER_FUNC_NAME,
        UPGRADE_CONTRACT_FUNC_NAME,
    },
    host::{
        context::{AsyncCallTxData, Promise, TxFunctionName, TxTokenTransfer},
        vm_hooks::{vh_early_exit::early_exit_vm_error, VMHooksContext},
    },
    types::{top_encode_big_uint, top_encode_u64, RawHandle, VMAddress, VMCodeMetadata},
    vm_err_msg,
};
use multiversx_chain_vm_executor::VMHooksEarlyExit;
use num_traits::Zero;

use super::VMHooksHandler;

fn append_endpoint_name_and_args(
    args: &mut Vec<Vec<u8>>,
    endpoint_name: TxFunctionName,
    arg_buffer: Vec<Vec<u8>>,
) {
    if !endpoint_name.is_empty() {
        args.push(endpoint_name.into_bytes());
        args.extend(arg_buffer);
    }
}

impl<C: VMHooksContext> VMHooksHandler<C> {
    fn perform_transfer_execute_esdt(
        &mut self,
        to: VMAddress,
        token: Vec<u8>,
        amount: num_bigint::BigUint,
        _gas_limit: u64,
        func_name: TxFunctionName,
        arguments: Vec<Vec<u8>>,
    ) -> Result<(), VMHooksEarlyExit> {
        let mut args = vec![token, amount.to_bytes_be()];
        append_endpoint_name_and_args(&mut args, func_name, arguments);

        self.context.perform_transfer_execute(
            to,
            num_bigint::BigUint::zero(),
            ESDT_TRANSFER_FUNC_NAME.into(),
            args,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn perform_transfer_execute_nft(
        &mut self,
        to: VMAddress,
        token: Vec<u8>,
        nonce: u64,
        amount: num_bigint::BigUint,
        _gas_limit: u64,
        func_name: TxFunctionName,
        arguments: Vec<Vec<u8>>,
    ) -> Result<(), VMHooksEarlyExit> {
        let contract_address = self.context.current_address().clone();

        let mut args = vec![
            token,
            top_encode_u64(nonce),
            top_encode_big_uint(&amount),
            to.to_vec(),
        ];

        append_endpoint_name_and_args(&mut args, func_name, arguments);

        self.context.perform_transfer_execute(
            contract_address,
            num_bigint::BigUint::zero(),
            ESDT_NFT_TRANSFER_FUNC_NAME.into(),
            args,
        )
    }

    fn perform_transfer_execute_multi(
        &mut self,
        to: VMAddress,
        payments: Vec<TxTokenTransfer>,
        _gas_limit: u64,
        endpoint_name: TxFunctionName,
        arguments: Vec<Vec<u8>>,
    ) -> Result<(), VMHooksEarlyExit> {
        let contract_address = self.context.current_address().clone();

        let mut args = vec![to.to_vec(), top_encode_u64(payments.len() as u64)];

        for payment in payments.into_iter() {
            let token_bytes = payment.token_identifier;
            args.push(token_bytes);
            let nonce_bytes = top_encode_u64(payment.nonce);
            args.push(nonce_bytes);
            let amount_bytes = top_encode_big_uint(&payment.value);
            args.push(amount_bytes);
        }

        append_endpoint_name_and_args(&mut args, endpoint_name, arguments);

        self.context.perform_transfer_execute(
            contract_address,
            num_bigint::BigUint::zero(),
            ESDT_MULTI_TRANSFER_FUNC_NAME.into(),
            args,
        )
    }

    fn perform_upgrade_contract(
        &mut self,
        to: VMAddress,
        egld_value: num_bigint::BigUint,
        contract_code: Vec<u8>,
        code_metadata: VMCodeMetadata,
        args: Vec<Vec<u8>>,
    ) -> Result<(), VMHooksEarlyExit> {
        let mut arguments = vec![contract_code, code_metadata.to_vec()];
        arguments.extend(args);
        self.context.perform_async_call(
            to,
            egld_value,
            UPGRADE_CONTRACT_FUNC_NAME.into(),
            arguments,
        )
    }

    pub fn transfer_value_execute(
        &mut self,
        to_handle: RawHandle,
        amount_handle: RawHandle,
        _gas_limit: u64,
        endpoint_name_handle: RawHandle,
        arg_buffer_handle: RawHandle,
    ) -> Result<(), VMHooksEarlyExit> {
        let recipient = self.context.m_types_lock().mb_to_address(to_handle);
        let egld_value = self.context.m_types_lock().bu_get(amount_handle);
        let endpoint_name = self
            .context
            .m_types_lock()
            .mb_to_function_name(endpoint_name_handle);
        let arg_buffer = self.load_arg_data(arg_buffer_handle)?;

        self.context
            .perform_transfer_execute(recipient, egld_value, endpoint_name, arg_buffer)
    }

    pub fn multi_transfer_esdt_nft_execute(
        &mut self,
        to_handle: RawHandle,
        payments_handle: RawHandle,
        gas_limit: u64,
        endpoint_name_handle: RawHandle,
        arg_buffer_handle: RawHandle,
    ) -> Result<(), VMHooksEarlyExit> {
        let to = self.context.m_types_lock().mb_to_address(to_handle);
        let (payments, num_bytes_copied) = self
            .context
            .m_types_lock()
            .mb_get_vec_of_esdt_payments(payments_handle);
        self.use_gas_for_data_copy(num_bytes_copied)?;
        let endpoint_name = self
            .context
            .m_types_lock()
            .mb_to_function_name(endpoint_name_handle);
        let arg_buffer = self.load_arg_data(arg_buffer_handle)?;

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

    pub fn async_call_raw(
        &mut self,
        to_handle: RawHandle,
        egld_value_handle: RawHandle,
        endpoint_name_handle: RawHandle,
        arg_buffer_handle: RawHandle,
    ) -> Result<(), VMHooksEarlyExit> {
        let to = self.context.m_types_lock().mb_to_address(to_handle);
        let egld_value = self.context.m_types_lock().bu_get(egld_value_handle);
        let endpoint_name = self
            .context
            .m_types_lock()
            .mb_to_function_name(endpoint_name_handle);
        let arg_buffer = self.load_arg_data(arg_buffer_handle)?;

        self.context
            .perform_async_call(to, egld_value, endpoint_name, arg_buffer)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_async_call_raw(
        &mut self,
        to_handle: RawHandle,
        egld_value_handle: RawHandle,
        endpoint_name_handle: RawHandle,
        arg_buffer_handle: RawHandle,
        success_callback: &[u8],
        error_callback: &[u8],
        _gas: u64,
        _extra_gas_for_callback: u64,
        callback_closure_handle: RawHandle,
    ) -> Result<(), VMHooksEarlyExit> {
        let contract_address = self.context.current_address().clone();
        let to = self.context.m_types_lock().mb_to_address(to_handle);
        let egld_value = self.context.m_types_lock().bu_get(egld_value_handle);
        let endpoint_name = self
            .context
            .m_types_lock()
            .mb_to_function_name(endpoint_name_handle);
        if endpoint_name.is_empty() {
            // imitating the behavior of the VM
            // TODO: lift limitation from the VM, then also remove this condition here
            return Err(early_exit_vm_error(vm_err_msg::PROMISES_TOKENIZE_FAILED));
        }
        let arg_buffer = self.load_arg_data(arg_buffer_handle)?;
        let tx_hash = self.context.tx_hash();
        let callback_closure_data = self
            .context
            .m_types_lock()
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

        let mut tx_result = self.context.result_lock();
        tx_result.all_calls.push(promise.call.clone());
        tx_result.pending_calls.promises.push(promise);

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn deploy_contract(
        &mut self,
        _gas: u64,
        egld_value_handle: RawHandle,
        code_handle: RawHandle,
        code_metadata_handle: RawHandle,
        arg_buffer_handle: RawHandle,
        new_address_handle: RawHandle,
        result_handle: RawHandle,
    ) -> Result<(), VMHooksEarlyExit> {
        let egld_value = self.context.m_types_lock().bu_get(egld_value_handle);
        let code = self.context.m_types_lock().mb_get(code_handle).to_vec();
        let code_metadata = self
            .context
            .m_types_lock()
            .mb_to_code_metadata(code_metadata_handle);
        let arg_buffer = self.load_arg_data(arg_buffer_handle)?;

        let (new_address, result) =
            self.context
                .perform_deploy(egld_value, code, code_metadata, arg_buffer)?;

        self.context
            .m_types_lock()
            .mb_set(new_address_handle, new_address.to_vec());
        self.set_return_data(result_handle, result)?;

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn deploy_from_source_contract(
        &mut self,
        _gas: u64,
        egld_value_handle: RawHandle,
        source_contract_address_handle: RawHandle,
        code_metadata_handle: RawHandle,
        arg_buffer_handle: RawHandle,
        new_address_handle: RawHandle,
        result_handle: RawHandle,
    ) -> Result<(), VMHooksEarlyExit> {
        let egld_value = self.context.m_types_lock().bu_get(egld_value_handle);
        let source_contract_address = self
            .context
            .m_types_lock()
            .mb_to_address(source_contract_address_handle);
        let source_contract_code = self.context.account_code(&source_contract_address);
        let code_metadata = self
            .context
            .m_types_lock()
            .mb_to_code_metadata(code_metadata_handle);
        let arg_buffer = self.load_arg_data(arg_buffer_handle)?;

        let (new_address, result) = self.context.perform_deploy(
            egld_value,
            source_contract_code,
            code_metadata,
            arg_buffer,
        )?;

        self.context
            .m_types_lock()
            .mb_set(new_address_handle, new_address.to_vec());

        self.set_return_data(result_handle, result)?;

        Ok(())
    }

    pub fn upgrade_from_source_contract(
        &mut self,
        sc_address_handle: RawHandle,
        _gas: u64,
        egld_value_handle: RawHandle,
        source_contract_address_handle: RawHandle,
        code_metadata_handle: RawHandle,
        arg_buffer_handle: RawHandle,
    ) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().base_ops_api_cost.create_contract)?;

        let to = self.context.m_types_lock().mb_to_address(sc_address_handle);
        let egld_value = self.context.m_types_lock().bu_get(egld_value_handle);
        let source_contract_address = self
            .context
            .m_types_lock()
            .mb_to_address(source_contract_address_handle);
        let source_contract_code = self.context.account_code(&source_contract_address);
        let code_metadata = self
            .context
            .m_types_lock()
            .mb_to_code_metadata(code_metadata_handle);
        let arg_buffer = self.load_arg_data(arg_buffer_handle)?;

        self.perform_upgrade_contract(
            to,
            egld_value,
            source_contract_code,
            code_metadata,
            arg_buffer,
        )
    }

    pub fn upgrade_contract(
        &mut self,
        sc_address_handle: RawHandle,
        _gas: u64,
        egld_value_handle: RawHandle,
        code_handle: RawHandle,
        code_metadata_handle: RawHandle,
        arg_buffer_handle: RawHandle,
    ) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().base_ops_api_cost.create_contract)?;

        let to = self.context.m_types_lock().mb_to_address(sc_address_handle);
        let egld_value = self.context.m_types_lock().bu_get(egld_value_handle);
        let code = self.context.m_types_lock().mb_get(code_handle).to_vec();
        let code_metadata = self
            .context
            .m_types_lock()
            .mb_to_code_metadata(code_metadata_handle);
        let arg_buffer = self.load_arg_data(arg_buffer_handle)?;

        self.perform_upgrade_contract(to, egld_value, code, code_metadata, arg_buffer)
    }

    pub fn execute_on_dest_context_raw(
        &mut self,
        _gas: u64,
        to_handle: RawHandle,
        egld_value_handle: RawHandle,
        endpoint_name_handle: RawHandle,
        arg_buffer_handle: RawHandle,
        result_handle: RawHandle,
    ) -> Result<(), VMHooksEarlyExit> {
        let to = self.context.m_types_lock().mb_to_address(to_handle);
        let egld_value = self.context.m_types_lock().bu_get(egld_value_handle);
        let endpoint_name = self
            .context
            .m_types_lock()
            .mb_to_function_name(endpoint_name_handle);
        let arg_buffer = self.load_arg_data(arg_buffer_handle)?;

        let result = self.context.perform_execute_on_dest_context(
            to,
            egld_value,
            endpoint_name,
            arg_buffer,
        )?;

        self.set_return_data(result_handle, result)?;

        Ok(())
    }

    pub fn execute_on_dest_context_readonly_raw(
        &mut self,
        _gas: u64,
        to_handle: RawHandle,
        endpoint_name_handle: RawHandle,
        arg_buffer_handle: RawHandle,
        result_handle: RawHandle,
    ) -> Result<(), VMHooksEarlyExit> {
        let to = self.context.m_types_lock().mb_to_address(to_handle);
        let endpoint_name = self
            .context
            .m_types_lock()
            .mb_to_function_name(endpoint_name_handle);
        let arg_buffer = self.load_arg_data(arg_buffer_handle)?;

        let result =
            self.context
                .perform_execute_on_dest_context_readonly(to, endpoint_name, arg_buffer)?;

        self.set_return_data(result_handle, result)?;

        Ok(())
    }

    fn load_arg_data(
        &mut self,
        arg_buffer_handle: RawHandle,
    ) -> Result<Vec<Vec<u8>>, VMHooksEarlyExit> {
        let (arg_buffer, num_bytes_copied) = self
            .context
            .m_types_lock()
            .mb_get_vec_of_bytes(arg_buffer_handle);

        self.use_gas_for_data_copy(num_bytes_copied)?;
        Ok(arg_buffer)
    }

    fn set_return_data(
        &mut self,
        result_handle: RawHandle,
        result: Vec<Vec<u8>>,
    ) -> Result<(), VMHooksEarlyExit> {
        let num_bytes_copied = self
            .context
            .m_types_lock()
            .mb_set_vec_of_bytes(result_handle, result);

        self.use_gas_for_data_copy(num_bytes_copied)
    }

    pub fn clean_return_data(&mut self) -> Result<(), VMHooksEarlyExit> {
        let mut tx_result = self.context.result_lock();
        tx_result.result_values.clear();
        Ok(())
    }

    pub fn delete_from_return_data(&mut self, index: usize) -> Result<(), VMHooksEarlyExit> {
        let mut tx_result = self.context.result_lock();
        if index > tx_result.result_values.len() {
            return Ok(());
        }

        let _ = tx_result.result_values.remove(index);
        Ok(())
    }
}
