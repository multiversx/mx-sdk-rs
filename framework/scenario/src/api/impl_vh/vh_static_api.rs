use std::sync::{Mutex, MutexGuard};

use multiversx_chain_vm_executor::{MemLength, MemPtr, VMHooksError};

use multiversx_chain_vm::{
    blockchain::state::{AccountData, BlockInfo},
    chain_core::types::ReturnCode,
    host::{
        context::{BackTransfers, ManagedTypeContainer, TxFunctionName, TxInput, TxLog, TxResult},
        vm_hooks::{
            VMHooksBigFloat, VMHooksBigInt, VMHooksBlockchain, VMHooksCallValue, VMHooksCrypto,
            VMHooksEndpointArgument, VMHooksEndpointFinish, VMHooksErrorManaged, VMHooksHandler,
            VMHooksHandlerSource, VMHooksLog, VMHooksManagedBuffer, VMHooksManagedMap,
            VMHooksManagedTypes, VMHooksSend, VMHooksSignalError, VMHooksStorageRead,
            VMHooksStorageWrite,
        },
    },
    schedule::GasSchedule,
    types::{VMAddress, VMCodeMetadata},
};

use crate::executor::debug::ContractDebugInstanceState;

const ZERO_GAS_SCHEDULE: GasSchedule = GasSchedule::zeroed();

/// A simple wrapper around a managed type container Mutex.
///
/// Implements `VMHooksManagedTypes` and thus can be used as a basis of a minimal static API.
#[derive(Debug, Default)]
pub struct StaticApiVMHooksHandler(Mutex<ManagedTypeContainer>);

impl StaticApiVMHooksHandler {
    pub const CURRENT_ADDRESS_PLACEHOLDER: VMAddress =
        VMAddress::new(*b"STATIC_API_CURRENT_ADDRESS______");
}

impl VMHooksHandlerSource for StaticApiVMHooksHandler {
    unsafe fn memory_load(&self, offset: MemPtr, length: MemLength) -> Vec<u8> {
        let slice = unsafe { ContractDebugInstanceState::main_memory_load(offset, length) };
        slice.to_vec()
    }

    unsafe fn memory_store(&self, offset: MemPtr, data: &[u8]) {
        unsafe {
            ContractDebugInstanceState::main_memory_store(offset, data);
        }
    }

    fn m_types_lock(&self) -> MutexGuard<ManagedTypeContainer> {
        self.0.lock().unwrap()
    }

    fn halt_with_error(&mut self, status: ReturnCode, message: &str) -> Result<(), VMHooksError> {
        panic!("VM error occured, status: {status}, message: {message}")
    }

    fn halt_with_error_legacy(&mut self, status: ReturnCode, message: &str) {
        panic!("VM error occured, status: {status}, message: {message}")
    }

    fn gas_schedule(&self) -> &GasSchedule {
        &ZERO_GAS_SCHEDULE
    }

    fn use_gas(&mut self, _gas: u64) {}

    fn input_ref(&self) -> &TxInput {
        panic!("cannot access tx inputs in the StaticApi")
    }

    fn current_address(&self) -> &VMAddress {
        &Self::CURRENT_ADDRESS_PLACEHOLDER
    }

    fn random_next_bytes(&self, _length: usize) -> Vec<u8> {
        panic!("cannot access the random bytes generator in the StaticApi")
    }

    fn result_lock(&self) -> MutexGuard<TxResult> {
        panic!("cannot access tx results in the StaticApi")
    }

    fn push_tx_log(&self, _tx_log: TxLog) {
        panic!("cannot log events in the StaticApi")
    }

    fn storage_read_any_address(&self, _address: &VMAddress, _key: &[u8]) -> Vec<u8> {
        panic!("cannot access the storage in the StaticApi")
    }

    fn storage_write(&mut self, _key: &[u8], _value: &[u8]) {
        panic!("cannot access the storage in the StaticApi")
    }

    fn get_previous_block_info(&self) -> &BlockInfo {
        panic!("cannot access the block info in the StaticApi")
    }

    fn get_current_block_info(&self) -> &BlockInfo {
        panic!("cannot access the block info in the StaticApi")
    }

    fn back_transfers_lock(&self) -> MutexGuard<BackTransfers> {
        panic!("cannot access the back transfers in the StaticApi")
    }

    fn account_data(&self, _address: &VMAddress) -> Option<AccountData> {
        panic!("cannot access account data in the StaticApi")
    }

    fn account_code(&self, _address: &VMAddress) -> Vec<u8> {
        panic!("cannot access account data in the StaticApi")
    }

    fn perform_async_call(
        &mut self,
        _to: VMAddress,
        _egld_value: num_bigint::BigUint,
        _func_name: TxFunctionName,
        _args: Vec<Vec<u8>>,
    ) -> ! {
        panic!("cannot launch contract calls in the StaticApi")
    }

    fn perform_execute_on_dest_context(
        &mut self,
        _to: VMAddress,
        _egld_value: num_bigint::BigUint,
        _func_name: TxFunctionName,
        _args: Vec<Vec<u8>>,
    ) -> Vec<Vec<u8>> {
        panic!("cannot launch contract calls in the StaticApi")
    }

    fn perform_execute_on_dest_context_readonly(
        &mut self,
        _to: VMAddress,
        _func_name: TxFunctionName,
        _arguments: Vec<Vec<u8>>,
    ) -> Vec<Vec<u8>> {
        panic!("cannot launch contract calls in the StaticApi")
    }

    fn perform_deploy(
        &mut self,
        _egld_value: num_bigint::BigUint,
        _contract_code: Vec<u8>,
        _code_metadata: VMCodeMetadata,
        _args: Vec<Vec<u8>>,
    ) -> (VMAddress, Vec<Vec<u8>>) {
        panic!("cannot launch contract calls in the StaticApi")
    }

    fn perform_transfer_execute(
        &mut self,
        _to: VMAddress,
        _egld_value: num_bigint::BigUint,
        _func_name: TxFunctionName,
        _arguments: Vec<Vec<u8>>,
    ) {
        panic!("cannot launch contract calls in the StaticApi")
    }
}

impl VMHooksBigInt for StaticApiVMHooksHandler {}
impl VMHooksManagedBuffer for StaticApiVMHooksHandler {}
impl VMHooksManagedMap for StaticApiVMHooksHandler {}
impl VMHooksBigFloat for StaticApiVMHooksHandler {}
impl VMHooksManagedTypes for StaticApiVMHooksHandler {}

impl VMHooksCallValue for StaticApiVMHooksHandler {}
impl VMHooksEndpointArgument for StaticApiVMHooksHandler {}
impl VMHooksEndpointFinish for StaticApiVMHooksHandler {}
impl VMHooksSignalError for StaticApiVMHooksHandler {}
impl VMHooksErrorManaged for StaticApiVMHooksHandler {}
impl VMHooksStorageRead for StaticApiVMHooksHandler {}
impl VMHooksStorageWrite for StaticApiVMHooksHandler {}
impl VMHooksCrypto for StaticApiVMHooksHandler {}
impl VMHooksBlockchain for StaticApiVMHooksHandler {}
impl VMHooksLog for StaticApiVMHooksHandler {}
impl VMHooksSend for StaticApiVMHooksHandler {}

impl VMHooksHandler for StaticApiVMHooksHandler {}
