use std::fmt::Debug;
use std::sync::MutexGuard;

use multiversx_chain_core::types::ReturnCode;
use multiversx_chain_vm_executor::{InstanceState, MemLength, MemPtr, VMHooksEarlyExit};
use num_bigint::BigUint;
use num_traits::Zero;

use crate::host::runtime::RuntimeInstanceCallLambdaDefault;
use crate::schedule::GasSchedule;
use crate::{
    blockchain::{
        reserved::STORAGE_RESERVED_PREFIX,
        state::{AccountData, BlockInfo},
    },
    host::context::{
        async_call_tx_input, AsyncCallTxData, BackTransfers, BlockchainUpdate, CallType,
        ManagedTypeContainer, TxCache, TxContextRef, TxFunctionName, TxInput, TxResult,
    },
    host::execution,
    types::{VMAddress, VMCodeMetadata},
    vm_err_msg,
};

use super::vh_early_exit::{early_exit_async_call, early_exit_vm_error};
use super::VMHooksContext;

pub struct TxVMHooksContext<S: InstanceState> {
    tx_context_ref: TxContextRef,
    pub(crate) instance_state_ref: S,
}

impl<S: InstanceState> TxVMHooksContext<S> {
    pub fn new(tx_context_ref: TxContextRef, instance_state_ref: S) -> Self {
        TxVMHooksContext {
            tx_context_ref,
            instance_state_ref,
        }
    }
}

impl<S: InstanceState> Debug for TxVMHooksContext<S> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TxContextVMHooksHandler").finish()
    }
}

impl<S: InstanceState> VMHooksContext for TxVMHooksContext<S> {
    unsafe fn memory_load(&self, offset: MemPtr, length: MemLength) -> Vec<u8> {
        self.instance_state_ref
            .memory_load_owned(offset, length)
            .expect("error loading memory from wasmer instance")
    }

    unsafe fn memory_store(&self, mem_ptr: MemPtr, data: &[u8]) {
        self.instance_state_ref
            .memory_store(mem_ptr, data)
            .expect("error writing to wasmer instance memory");
    }

    fn m_types_lock(&self) -> MutexGuard<ManagedTypeContainer> {
        self.tx_context_ref.m_types_lock()
    }

    fn gas_schedule(&self) -> &GasSchedule {
        &self.tx_context_ref.0.runtime_ref.vm_ref.gas_schedule
    }

    fn use_gas(&mut self, gas: u64) -> Result<(), VMHooksEarlyExit> {
        let gas_limit = self.input_ref().gas_limit;
        let state_ref = &mut self.instance_state_ref;
        let prev_gas_used = state_ref
            .get_points_used()
            .expect("error fetching points used from instance state");

        let next_gas_used = prev_gas_used + gas;

        // println!("use gas {gas}: {prev_gas_used} -> {next_gas_used}");

        if next_gas_used > gas_limit {
            Err(VMHooksEarlyExit::new(ReturnCode::OutOfGas.as_u64()))
        } else {
            state_ref
                .set_points_used(next_gas_used)
                .expect("error setting points used in instance");
            Ok(())
        }
    }

    fn input_ref(&self) -> &TxInput {
        self.tx_context_ref.input_ref()
    }

    fn random_next_bytes(&self, length: usize) -> Vec<u8> {
        self.tx_context_ref.rng_lock().next_bytes(length)
    }

    fn result_lock(&self) -> MutexGuard<TxResult> {
        self.tx_context_ref.result_lock()
    }

    fn storage_read_any_address(&self, address: &VMAddress, key: &[u8]) -> Vec<u8> {
        self.tx_context_ref.with_account_mut(address, |account| {
            account.storage.get(key).cloned().unwrap_or_default()
        })
    }

    fn storage_write(&mut self, key: &[u8], value: &[u8]) -> Result<(), VMHooksEarlyExit> {
        self.check_reserved_key(key)?;
        self.check_not_readonly()?;

        self.tx_context_ref.with_contract_account_mut(|account| {
            account.storage.insert(key.to_vec(), value.to_vec());
        });
        Ok(())
    }

    fn get_previous_block_info(&self) -> &BlockInfo {
        &self.tx_context_ref.blockchain_ref().previous_block_info
    }

    fn get_current_block_info(&self) -> &BlockInfo {
        &self.tx_context_ref.blockchain_ref().current_block_info
    }

    fn get_epoch_start_block_info(&self) -> &BlockInfo {
        &self.tx_context_ref.blockchain_ref().epoch_start_block_info
    }

    fn back_transfers_lock(&self) -> MutexGuard<BackTransfers> {
        self.tx_context_ref.back_transfers_lock()
    }

    fn account_data(&self, address: &VMAddress) -> Option<AccountData> {
        self.tx_context_ref
            .with_account_or_else(address, |account| Some(account.clone()), || None)
    }

    fn account_code(&self, address: &VMAddress) -> Vec<u8> {
        self.tx_context_ref
            .blockchain_cache()
            .with_account(address, |account| account.contract_path.clone())
            .unwrap_or_else(|| panic!("Account is not a smart contract, it has no code"))
    }

    fn perform_async_call(
        &mut self,
        to: VMAddress,
        egld_value: num_bigint::BigUint,
        func_name: TxFunctionName,
        arguments: Vec<Vec<u8>>,
    ) -> Result<(), VMHooksEarlyExit> {
        let async_call_data = self.create_async_call_data(to, egld_value, func_name, arguments);
        // the cell is no longer needed, since we end in a panic
        let mut tx_result = self.result_lock();
        tx_result.all_calls.push(async_call_data.clone());
        tx_result.pending_calls.async_call = Some(async_call_data);
        Err(early_exit_async_call())
    }

    fn perform_execute_on_dest_context(
        &mut self,
        to: VMAddress,
        egld_value: num_bigint::BigUint,
        func_name: TxFunctionName,
        arguments: Vec<Vec<u8>>,
    ) -> Result<Vec<Vec<u8>>, VMHooksEarlyExit> {
        let async_call_data = self.create_async_call_data(to, egld_value, func_name, arguments);
        let tx_input = async_call_tx_input(&async_call_data, CallType::ExecuteOnDestContext);
        let tx_cache = TxCache::new(self.tx_context_ref.blockchain_cache_arc());
        let (tx_result, blockchain_updates) = execution::execute_builtin_function_or_default(
            tx_input,
            tx_cache,
            &self.tx_context_ref.runtime_ref,
            RuntimeInstanceCallLambdaDefault,
        );

        if tx_result.result_status.is_success() {
            Ok(self.sync_call_post_processing(tx_result, blockchain_updates))
        } else {
            // also kill current execution
            Err(VMHooksEarlyExit::new(tx_result.result_status.as_u64())
                .with_message(tx_result.result_message.clone()))
        }
    }

    fn perform_execute_on_dest_context_readonly(
        &mut self,
        to: VMAddress,
        func_name: TxFunctionName,
        arguments: Vec<Vec<u8>>,
    ) -> Result<Vec<Vec<u8>>, VMHooksEarlyExit> {
        let async_call_data =
            self.create_async_call_data(to, BigUint::zero(), func_name, arguments);
        let mut tx_input = async_call_tx_input(&async_call_data, CallType::ExecuteOnDestContext);
        tx_input.readonly = true;
        let tx_cache = TxCache::new(self.tx_context_ref.blockchain_cache_arc());
        let (tx_result, blockchain_updates) = execution::execute_builtin_function_or_default(
            tx_input,
            tx_cache,
            &self.tx_context_ref.runtime_ref,
            RuntimeInstanceCallLambdaDefault,
        );

        if tx_result.result_status.is_success() {
            Ok(self.sync_call_post_processing(tx_result, blockchain_updates))
        } else {
            // also kill current execution
            Err(VMHooksEarlyExit::new(tx_result.result_status.as_u64())
                .with_message(tx_result.result_message.clone()))
        }
    }

    fn perform_deploy(
        &mut self,
        egld_value: num_bigint::BigUint,
        contract_code: Vec<u8>,
        code_metadata: VMCodeMetadata,
        args: Vec<Vec<u8>>,
    ) -> Result<(VMAddress, Vec<Vec<u8>>), VMHooksEarlyExit> {
        let contract_address = self.current_address();
        let tx_hash = self.tx_hash();
        let tx_input = TxInput {
            from: contract_address.clone(),
            to: VMAddress::zero(),
            egld_value,
            esdt_values: Vec::new(),
            func_name: TxFunctionName::INIT,
            args,
            gas_limit: 1000,
            gas_price: 0,
            tx_hash,
            ..Default::default()
        };

        let tx_cache = TxCache::new(self.tx_context_ref.blockchain_cache_arc());
        tx_cache.increase_account_nonce(contract_address);
        let (tx_result, new_address, blockchain_updates) = execution::execute_deploy(
            tx_input,
            contract_code,
            code_metadata,
            tx_cache,
            &self.tx_context_ref.runtime_ref,
            RuntimeInstanceCallLambdaDefault,
        );

        match tx_result.result_status {
            ReturnCode::Success => Ok((
                new_address,
                self.sync_call_post_processing(tx_result, blockchain_updates),
            )),
            ReturnCode::ExecutionFailed => {
                // TODO: not sure it's the right condition, it catches insufficient funds
                Err(VMHooksEarlyExit::new(ReturnCode::ExecutionFailed.as_u64())
                    .with_message(tx_result.result_message.clone()))
            }
            _ => Err(VMHooksEarlyExit::new(ReturnCode::ExecutionFailed.as_u64())
                .with_const_message(vm_err_msg::ERROR_SIGNALLED_BY_SMARTCONTRACT)),
        }
    }

    fn perform_transfer_execute(
        &mut self,
        to: VMAddress,
        egld_value: num_bigint::BigUint,
        func_name: TxFunctionName,
        arguments: Vec<Vec<u8>>,
    ) -> Result<(), VMHooksEarlyExit> {
        let async_call_data = self.create_async_call_data(to, egld_value, func_name, arguments);
        let mut tx_input = async_call_tx_input(&async_call_data, CallType::TransferExecute);
        if self.is_back_transfer(&tx_input) {
            tx_input.call_type = CallType::BackTransfer;
        }

        let tx_cache = TxCache::new(self.tx_context_ref.blockchain_cache_arc());
        let (tx_result, blockchain_updates) = execution::execute_builtin_function_or_default(
            tx_input,
            tx_cache,
            &self.tx_context_ref.runtime_ref,
            RuntimeInstanceCallLambdaDefault,
        );

        match tx_result.result_status {
            ReturnCode::Success => {
                self.tx_context_ref
                    .result_lock()
                    .all_calls
                    .push(async_call_data);

                let _ = self.sync_call_post_processing(tx_result, blockchain_updates);
                Ok(())
            }
            ReturnCode::ExecutionFailed => {
                // TODO: not sure it's the right condition, it catches insufficient funds
                Err(VMHooksEarlyExit::new(ReturnCode::ExecutionFailed.as_u64())
                    .with_message(tx_result.result_message.clone()))
            }
            _ => Err(VMHooksEarlyExit::new(ReturnCode::ExecutionFailed.as_u64())
                .with_const_message(vm_err_msg::ERROR_SIGNALLED_BY_SMARTCONTRACT)),
        }
    }
}

impl<S: InstanceState> TxVMHooksContext<S> {
    fn create_async_call_data(
        &self,
        to: VMAddress,
        egld_value: num_bigint::BigUint,
        func_name: TxFunctionName,
        arguments: Vec<Vec<u8>>,
    ) -> AsyncCallTxData {
        let contract_address = &self.tx_context_ref.input_ref().to;
        let tx_hash = self.tx_hash();
        AsyncCallTxData {
            from: contract_address.clone(),
            to,
            call_value: egld_value,
            endpoint_name: func_name,
            arguments,
            tx_hash,
        }
    }

    fn sync_call_post_processing(
        &self,
        tx_result: TxResult,
        blockchain_updates: BlockchainUpdate,
    ) -> Vec<Vec<u8>> {
        self.tx_context_ref
            .blockchain_cache()
            .commit_updates(blockchain_updates);

        self.tx_context_ref
            .result_lock()
            .merge_after_sync_call(&tx_result);

        let contract_address = &self.tx_context_ref.input_ref().to;
        let builtin_functions = &self.tx_context_ref.runtime_ref.vm_ref.builtin_functions;
        self.back_transfers_lock()
            .new_from_result(contract_address, &tx_result, builtin_functions);

        tx_result.result_values
    }

    fn check_reserved_key(&mut self, key: &[u8]) -> Result<(), VMHooksEarlyExit> {
        if key.starts_with(STORAGE_RESERVED_PREFIX) {
            return Err(early_exit_vm_error(vm_err_msg::WRITE_RESERVED));
        }
        Ok(())
    }

    /// TODO: only checked on storage writes, needs more checks for calls, transfers, etc.
    fn check_not_readonly(&mut self) -> Result<(), VMHooksEarlyExit> {
        if self.tx_context_ref.input_ref().readonly {
            return Err(early_exit_vm_error(vm_err_msg::WRITE_READONLY));
        }
        Ok(())
    }

    fn is_back_transfer(&self, tx_input: &TxInput) -> bool {
        let caller_address = &self.tx_context_ref.input_ref().from;
        if !caller_address.is_smart_contract_address() {
            return false;
        }

        let builtin_functions = &self.tx_context_ref.runtime_ref.vm_ref.builtin_functions;
        let token_transfers = builtin_functions.extract_token_transfers(tx_input);
        &token_transfers.real_recipient == caller_address
    }
}
