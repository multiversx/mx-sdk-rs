use std::sync::{Mutex, MutexGuard};

use multiversx_chain_vm_executor::{MemLength, MemPtr, VMHooksEarlyExit};

use multiversx_chain_vm::{
    blockchain::state::{AccountData, BlockConfig},
    host::{
        context::{BackTransfers, ManagedTypeContainer, TxFunctionName, TxInput, TxLog, TxResult},
        vm_hooks::VMHooksContext,
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
pub struct StaticApiVMHooksContext(Mutex<ManagedTypeContainer>);

impl StaticApiVMHooksContext {
    pub const CURRENT_ADDRESS_PLACEHOLDER: VMAddress =
        VMAddress::new(*b"STATIC_API_CURRENT_ADDRESS______");
}

impl VMHooksContext for StaticApiVMHooksContext {
    unsafe fn memory_load(&self, offset: MemPtr, length: MemLength) -> Vec<u8> {
        let slice = unsafe { ContractDebugInstanceState::main_memory_load(offset, length) };
        slice.to_vec()
    }

    unsafe fn memory_store(&self, offset: MemPtr, data: &[u8]) {
        unsafe {
            ContractDebugInstanceState::main_memory_store(offset, data);
        }
    }

    fn m_types_lock(&self) -> MutexGuard<'_, ManagedTypeContainer> {
        self.0.lock().unwrap()
    }

    fn gas_schedule(&self) -> &GasSchedule {
        &ZERO_GAS_SCHEDULE
    }

    fn use_gas(&mut self, _gas: u64) -> Result<(), VMHooksEarlyExit> {
        Ok(())
    }

    fn input_ref(&self) -> &TxInput {
        panic!("cannot access tx inputs in the StaticApi")
    }

    fn current_address(&self) -> &VMAddress {
        &Self::CURRENT_ADDRESS_PLACEHOLDER
    }

    fn random_next_bytes(&self, _length: usize) -> Vec<u8> {
        panic!("cannot access the random bytes generator in the StaticApi")
    }

    fn result_lock(&self) -> MutexGuard<'_, TxResult> {
        panic!("cannot access tx results in the StaticApi")
    }

    fn push_tx_log(&self, _tx_log: TxLog) {
        panic!("cannot log events in the StaticApi")
    }

    fn storage_read_any_address(&self, _address: &VMAddress, _key: &[u8]) -> Vec<u8> {
        panic!("cannot access the storage in the StaticApi")
    }

    fn storage_write(&mut self, _key: &[u8], _value: &[u8]) -> Result<(), VMHooksEarlyExit> {
        panic!("cannot access the storage in the StaticApi")
    }

    fn get_block_config(&self) -> &BlockConfig {
        panic!("cannot access the block info in the StaticApi")
    }

    fn back_transfers_lock(&self) -> MutexGuard<'_, BackTransfers> {
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
    ) -> Result<(), VMHooksEarlyExit> {
        panic!("cannot launch contract calls in the StaticApi")
    }

    fn perform_execute_on_dest_context(
        &mut self,
        _to: VMAddress,
        _egld_value: num_bigint::BigUint,
        _func_name: TxFunctionName,
        _args: Vec<Vec<u8>>,
    ) -> Result<TxResult, VMHooksEarlyExit> {
        panic!("cannot launch contract calls in the StaticApi")
    }

    fn perform_execute_on_dest_context_readonly(
        &mut self,
        _to: VMAddress,
        _func_name: TxFunctionName,
        _arguments: Vec<Vec<u8>>,
    ) -> Result<Vec<Vec<u8>>, VMHooksEarlyExit> {
        panic!("cannot launch contract calls in the StaticApi")
    }

    fn perform_deploy(
        &mut self,
        _egld_value: num_bigint::BigUint,
        _contract_code: Vec<u8>,
        _code_metadata: VMCodeMetadata,
        _args: Vec<Vec<u8>>,
    ) -> Result<(VMAddress, Vec<Vec<u8>>), VMHooksEarlyExit> {
        panic!("cannot launch contract calls in the StaticApi")
    }

    fn perform_transfer_execute(
        &mut self,
        _to: VMAddress,
        _egld_value: num_bigint::BigUint,
        _func_name: TxFunctionName,
        _arguments: Vec<Vec<u8>>,
    ) -> Result<(), VMHooksEarlyExit> {
        panic!("cannot launch contract calls in the StaticApi")
    }
}
