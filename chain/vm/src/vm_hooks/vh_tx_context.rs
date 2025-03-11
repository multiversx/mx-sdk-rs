use std::rc::Weak;
use std::sync::MutexGuard;
use std::{fmt::Debug, ops::Deref};

use multiversx_chain_core::types::ReturnCode;
use multiversx_chain_vm_executor::{BreakpointValue, Instance, MemLength, MemPtr};
use num_bigint::BigUint;
use num_traits::Zero;

use crate::tx_execution::execute_builtin_function_or_default;
use crate::{
    tx_execution::instance_call,
    tx_mock::{
        async_call_tx_input, AsyncCallTxData, BackTransfers, BlockchainUpdate, CallType, TxCache,
        TxContextRef, TxFunctionName, TxInput, TxManagedTypes, TxPanic, TxResult,
    },
    types::{VMAddress, VMCodeMetadata},
    vm_err_msg,
    vm_hooks::{
        VMHooksBigFloat, VMHooksBigInt, VMHooksBlockchain, VMHooksCallValue, VMHooksCrypto,
        VMHooksEndpointArgument, VMHooksEndpointFinish, VMHooksError, VMHooksErrorManaged,
        VMHooksHandler, VMHooksHandlerSource, VMHooksLog, VMHooksManagedBuffer, VMHooksManagedMap,
        VMHooksManagedTypes, VMHooksSend, VMHooksStorageRead, VMHooksStorageWrite,
    },
    world_mock::{reserved::STORAGE_RESERVED_PREFIX, AccountData, BlockInfo},
};

pub struct TxContextVMHooksHandler {
    tx_context_ref: TxContextRef,
    instance_ref: Weak<Box<dyn Instance>>,
}

impl TxContextVMHooksHandler {
    pub fn new(tx_context_ref: TxContextRef, instance_ref: Weak<Box<dyn Instance>>) -> Self {
        TxContextVMHooksHandler {
            tx_context_ref,
            instance_ref,
        }
    }

    fn instance_box_ref(&self) -> &dyn Instance {
        assert!(
            self.instance_ref.strong_count() > 0,
            "instance reference dropped"
        );
        unsafe {
            let box_ref = self
                .instance_ref
                .as_ptr()
                .as_ref()
                .expect("null instance pointer");
            box_ref.deref()
        }
    }
}

impl Debug for TxContextVMHooksHandler {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TxContextVMHooksHandler").finish()
    }
}

impl VMHooksHandlerSource for TxContextVMHooksHandler {
    unsafe fn memory_load(&self, offset: MemPtr, length: MemLength) -> &[u8] {
        self.instance_box_ref()
            .memory_load(offset, length)
            .expect("error loading memory from wasmer instance")
    }

    unsafe fn memory_store(&self, mem_ptr: MemPtr, data: &[u8]) {
        self.instance_box_ref()
            .memory_store(mem_ptr, data)
            .expect("error writing to wasmer instance memory");
    }

    fn m_types_lock(&self) -> MutexGuard<TxManagedTypes> {
        self.tx_context_ref.m_types_lock()
    }

    fn halt_with_error(&self, status: ReturnCode, message: &str) {
        *self.tx_context_ref.result_lock() =
            TxResult::from_panic_obj(&TxPanic::new(status, message));
        let breakpoint = match status {
            ReturnCode::UserError => BreakpointValue::SignalError,
            _ => BreakpointValue::ExecutionFailed,
        };
        let _ = self.instance_box_ref().set_breakpoint_value(breakpoint);
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

    fn storage_write(&self, key: &[u8], value: &[u8]) {
        self.check_reserved_key(key);
        self.check_not_readonly();

        self.tx_context_ref.with_contract_account_mut(|account| {
            account.storage.insert(key.to_vec(), value.to_vec());
        });
    }

    fn get_previous_block_info(&self) -> &BlockInfo {
        &self.tx_context_ref.blockchain_ref().previous_block_info
    }

    fn get_current_block_info(&self) -> &BlockInfo {
        &self.tx_context_ref.blockchain_ref().current_block_info
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
        &self,
        to: VMAddress,
        egld_value: num_bigint::BigUint,
        func_name: TxFunctionName,
        arguments: Vec<Vec<u8>>,
    ) -> ! {
        let async_call_data = self.create_async_call_data(to, egld_value, func_name, arguments);
        // the cell is no longer needed, since we end in a panic
        let mut tx_result = self.result_lock();
        tx_result.all_calls.push(async_call_data.clone());
        tx_result.pending_calls.async_call = Some(async_call_data);
        drop(tx_result); // this avoid to poison the mutex
        std::panic::panic_any(BreakpointValue::AsyncCall);
    }

    fn perform_execute_on_dest_context(
        &self,
        to: VMAddress,
        egld_value: num_bigint::BigUint,
        func_name: TxFunctionName,
        arguments: Vec<Vec<u8>>,
    ) -> Vec<Vec<u8>> {
        let async_call_data = self.create_async_call_data(to, egld_value, func_name, arguments);
        let tx_input = async_call_tx_input(&async_call_data, CallType::ExecuteOnDestContext);
        let tx_cache = TxCache::new(self.tx_context_ref.blockchain_cache_arc());
        let (tx_result, blockchain_updates) = execute_builtin_function_or_default(
            tx_input,
            tx_cache,
            &self.tx_context_ref.runtime_ref,
            instance_call,
        );

        if tx_result.result_status.is_success() {
            self.sync_call_post_processing(tx_result, blockchain_updates)
        } else {
            // also kill current execution
            self.halt_with_error(tx_result.result_status, &tx_result.result_message);
            Vec::new()
        }
    }

    fn perform_execute_on_dest_context_readonly(
        &self,
        to: VMAddress,
        func_name: TxFunctionName,
        arguments: Vec<Vec<u8>>,
    ) -> Vec<Vec<u8>> {
        let async_call_data =
            self.create_async_call_data(to, BigUint::zero(), func_name, arguments);
        let mut tx_input = async_call_tx_input(&async_call_data, CallType::ExecuteOnDestContext);
        tx_input.readonly = true;
        let tx_cache = TxCache::new(self.tx_context_ref.blockchain_cache_arc());
        let (tx_result, blockchain_updates) = execute_builtin_function_or_default(
            tx_input,
            tx_cache,
            &self.tx_context_ref.runtime_ref,
            instance_call,
        );

        if tx_result.result_status.is_success() {
            self.sync_call_post_processing(tx_result, blockchain_updates)
        } else {
            // also kill current execution
            self.halt_with_error(tx_result.result_status, &tx_result.result_message);
            Vec::new()
        }
    }

    fn perform_deploy(
        &self,
        egld_value: num_bigint::BigUint,
        contract_code: Vec<u8>,
        code_metadata: VMCodeMetadata,
        args: Vec<Vec<u8>>,
    ) -> (VMAddress, Vec<Vec<u8>>) {
        let contract_address = self.current_address();
        let tx_hash = self.tx_hash();
        let tx_input = TxInput {
            from: contract_address.clone(),
            to: VMAddress::zero(),
            egld_value,
            esdt_values: Vec::new(),
            func_name: TxFunctionName::EMPTY,
            args,
            gas_limit: 1000,
            gas_price: 0,
            tx_hash,
            ..Default::default()
        };

        let tx_cache = TxCache::new(self.tx_context_ref.blockchain_cache_arc());
        tx_cache.increase_acount_nonce(contract_address);
        let (tx_result, new_address, blockchain_updates) =
            self.tx_context_ref.runtime_ref.deploy_contract(
                tx_input,
                contract_code,
                code_metadata,
                tx_cache,
                instance_call,
            );

        match tx_result.result_status {
            ReturnCode::Success => (
                new_address,
                self.sync_call_post_processing(tx_result, blockchain_updates),
            ),
            ReturnCode::ExecutionFailed => {
                // TODO: not sure it's the right condition, it catches insufficient funds
                self.vm_error(&tx_result.result_message);
                (VMAddress::zero(), Vec::new())
            },
            _ => {
                self.vm_error(vm_err_msg::ERROR_SIGNALLED_BY_SMARTCONTRACT);

                (VMAddress::zero(), Vec::new())
            },
        }
    }

    fn perform_transfer_execute(
        &self,
        to: VMAddress,
        egld_value: num_bigint::BigUint,
        func_name: TxFunctionName,
        arguments: Vec<Vec<u8>>,
    ) {
        let async_call_data = self.create_async_call_data(to, egld_value, func_name, arguments);
        let mut tx_input = async_call_tx_input(&async_call_data, CallType::TransferExecute);
        if self.is_back_transfer(&tx_input) {
            tx_input.call_type = CallType::BackTransfer;
        }

        let tx_cache = TxCache::new(self.tx_context_ref.blockchain_cache_arc());
        let (tx_result, blockchain_updates) = execute_builtin_function_or_default(
            tx_input,
            tx_cache,
            &self.tx_context_ref.runtime_ref,
            instance_call,
        );

        match tx_result.result_status {
            ReturnCode::Success => {
                self.tx_context_ref
                    .result_lock()
                    .all_calls
                    .push(async_call_data);

                let _ = self.sync_call_post_processing(tx_result, blockchain_updates);
            },
            ReturnCode::ExecutionFailed => self.vm_error(&tx_result.result_message), // TODO: not sure it's the right condition, it catches insufficient funds
            _ => self.vm_error(vm_err_msg::ERROR_SIGNALLED_BY_SMARTCONTRACT),
        }
    }
}

impl TxContextVMHooksHandler {
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

    fn check_reserved_key(&self, key: &[u8]) {
        if key.starts_with(STORAGE_RESERVED_PREFIX) {
            self.vm_error(vm_err_msg::WRITE_RESERVED);
        }
    }

    /// TODO: only checked on storage writes, needs more checks for calls, transfers, etc.
    fn check_not_readonly(&self) {
        if self.tx_context_ref.input_ref().readonly {
            self.vm_error(vm_err_msg::WRITE_READONLY);
        }
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

impl VMHooksBigInt for TxContextVMHooksHandler {}
impl VMHooksManagedBuffer for TxContextVMHooksHandler {}
impl VMHooksManagedMap for TxContextVMHooksHandler {}
impl VMHooksBigFloat for TxContextVMHooksHandler {}
impl VMHooksManagedTypes for TxContextVMHooksHandler {}

impl VMHooksCallValue for TxContextVMHooksHandler {}
impl VMHooksEndpointArgument for TxContextVMHooksHandler {}
impl VMHooksEndpointFinish for TxContextVMHooksHandler {}
impl VMHooksError for TxContextVMHooksHandler {}
impl VMHooksErrorManaged for TxContextVMHooksHandler {}
impl VMHooksStorageRead for TxContextVMHooksHandler {}
impl VMHooksStorageWrite for TxContextVMHooksHandler {}
impl VMHooksCrypto for TxContextVMHooksHandler {}
impl VMHooksBlockchain for TxContextVMHooksHandler {}
impl VMHooksLog for TxContextVMHooksHandler {}
impl VMHooksSend for TxContextVMHooksHandler {}

impl VMHooksHandler for TxContextVMHooksHandler {}
