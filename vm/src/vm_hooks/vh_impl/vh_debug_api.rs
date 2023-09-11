use std::sync::{Arc, MutexGuard};

use multiversx_chain_vm_executor::BreakpointValue;

use crate::{
    tx_execution::execute_current_tx_context_input,
    tx_mock::{
        async_call_tx_input, AsyncCallTxData, BlockchainUpdate, TxCache, TxContext, TxFunctionName,
        TxInput, TxManagedTypes, TxPanic, TxResult,
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

/// A simple wrapper around a managed type container RefCell.
///
/// Implements `VMHooksManagedTypes` and thus can be used as a basis of a minimal static API.
#[derive(Debug)]
pub struct DebugApiVMHooksHandler(Arc<TxContext>);

impl DebugApiVMHooksHandler {
    pub fn new(tx_context_arc: Arc<TxContext>) -> Self {
        DebugApiVMHooksHandler(tx_context_arc)
    }
}

impl VMHooksHandlerSource for DebugApiVMHooksHandler {
    fn m_types_lock(&self) -> MutexGuard<TxManagedTypes> {
        self.0.m_types_lock()
    }

    fn halt_with_error(&self, status: u64, message: &str) -> ! {
        *self.0.result_lock() = TxResult::from_panic_obj(&TxPanic::new(status, message));
        let breakpoint = match status {
            4 => BreakpointValue::SignalError,
            _ => BreakpointValue::ExecutionFailed,
        };
        std::panic::panic_any(breakpoint);
    }

    fn input_ref(&self) -> &TxInput {
        self.0.input_ref()
    }

    fn random_next_bytes(&self, length: usize) -> Vec<u8> {
        self.0.rng_lock().next_bytes(length)
    }

    fn result_lock(&self) -> MutexGuard<TxResult> {
        self.0.result_lock()
    }

    fn storage_read_any_address(&self, address: &VMAddress, key: &[u8]) -> Vec<u8> {
        self.0.with_account_mut(address, |account| {
            account.storage.get(key).cloned().unwrap_or_default()
        })
    }

    fn storage_write(&self, key: &[u8], value: &[u8]) {
        self.check_reserved_key(key);

        self.0.with_contract_account_mut(|account| {
            account.storage.insert(key.to_vec(), value.to_vec());
        });
    }

    fn get_previous_block_info(&self) -> &BlockInfo {
        &self.0.blockchain_ref().previous_block_info
    }

    fn get_current_block_info(&self) -> &BlockInfo {
        &self.0.blockchain_ref().current_block_info
    }

    fn account_data(&self, address: &VMAddress) -> AccountData {
        self.0.with_account(address, |account| account.clone())
    }

    fn account_code(&self, address: &VMAddress) -> Vec<u8> {
        self.0
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
        let tx_input = async_call_tx_input(&async_call_data);
        let tx_cache = TxCache::new(self.0.blockchain_cache_arc());
        let (tx_result, blockchain_updates) = self.0.vm_ref.execute_builtin_function_or_default(
            tx_input,
            tx_cache,
            execute_current_tx_context_input,
        );

        if tx_result.result_status == 0 {
            self.sync_call_post_processing(tx_result, blockchain_updates)
        } else {
            // also kill current execution
            self.halt_with_error(tx_result.result_status, &tx_result.result_message)
        }
    }

    fn perform_deploy(
        &self,
        egld_value: num_bigint::BigUint,
        contract_code: Vec<u8>,
        _code_metadata: VMCodeMetadata,
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

        let tx_cache = TxCache::new(self.0.blockchain_cache_arc());
        tx_cache.increase_acount_nonce(contract_address);
        let (tx_result, new_address, blockchain_updates) = self.0.vm_ref.deploy_contract(
            tx_input,
            contract_code,
            tx_cache,
            execute_current_tx_context_input,
        );

        match tx_result.result_status {
            0 => (
                new_address,
                self.sync_call_post_processing(tx_result, blockchain_updates),
            ),
            10 => self.vm_error(&tx_result.result_message), // TODO: not sure it's the right condition, it catches insufficient funds
            _ => self.vm_error(vm_err_msg::ERROR_SIGNALLED_BY_SMARTCONTRACT),
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
        let tx_input = async_call_tx_input(&async_call_data);
        let tx_cache = TxCache::new(self.0.blockchain_cache_arc());
        let (tx_result, blockchain_updates) = self.0.vm_ref.execute_builtin_function_or_default(
            tx_input,
            tx_cache,
            execute_current_tx_context_input,
        );

        match tx_result.result_status {
            0 => {
                self.0.result_lock().all_calls.push(async_call_data);

                let _ = self.sync_call_post_processing(tx_result, blockchain_updates);
            },
            10 => self.vm_error(&tx_result.result_message), // TODO: not sure it's the right condition, it catches insufficient funds
            _ => self.vm_error(vm_err_msg::ERROR_SIGNALLED_BY_SMARTCONTRACT),
        }
    }
}

impl DebugApiVMHooksHandler {
    fn create_async_call_data(
        &self,
        to: VMAddress,
        egld_value: num_bigint::BigUint,
        func_name: TxFunctionName,
        arguments: Vec<Vec<u8>>,
    ) -> AsyncCallTxData {
        let contract_address = &self.0.input_ref().to;
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
        self.0.blockchain_cache().commit_updates(blockchain_updates);

        self.0.result_lock().merge_after_sync_call(&tx_result);

        tx_result.result_values
    }

    fn check_reserved_key(&self, key: &[u8]) {
        if key.starts_with(STORAGE_RESERVED_PREFIX) {
            self.vm_error("cannot write to storage under reserved key");
        }
    }
}

impl VMHooksBigInt for DebugApiVMHooksHandler {}
impl VMHooksManagedBuffer for DebugApiVMHooksHandler {}
impl VMHooksManagedMap for DebugApiVMHooksHandler {}
impl VMHooksBigFloat for DebugApiVMHooksHandler {}
impl VMHooksManagedTypes for DebugApiVMHooksHandler {}

impl VMHooksCallValue for DebugApiVMHooksHandler {}
impl VMHooksEndpointArgument for DebugApiVMHooksHandler {}
impl VMHooksEndpointFinish for DebugApiVMHooksHandler {}
impl VMHooksError for DebugApiVMHooksHandler {}
impl VMHooksErrorManaged for DebugApiVMHooksHandler {}
impl VMHooksStorageRead for DebugApiVMHooksHandler {}
impl VMHooksStorageWrite for DebugApiVMHooksHandler {}
impl VMHooksCrypto for DebugApiVMHooksHandler {}
impl VMHooksBlockchain for DebugApiVMHooksHandler {}
impl VMHooksLog for DebugApiVMHooksHandler {}
impl VMHooksSend for DebugApiVMHooksHandler {}

impl VMHooksHandler for DebugApiVMHooksHandler {}
