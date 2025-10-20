use std::{fmt::Debug, sync::MutexGuard};

use multiversx_chain_vm_executor::{MemLength, MemPtr, VMHooksEarlyExit};

use crate::{
    blockchain::state::{AccountData, BlockConfig},
    host::context::{
        BackTransfers, ManagedTypeContainer, TxFunctionName, TxInput, TxLog, TxResult,
    },
    schedule::GasSchedule,
    types::{VMAddress, VMCodeMetadata, H256},
};

/// Abstracts away the borrowing of a managed types structure.
pub trait VMHooksContext: Debug {
    /// Loads a slice of memory from the instance.
    ///
    /// ## Safety
    ///
    /// The offset and the length must point to valid instance memory.
    unsafe fn memory_load(&self, offset: MemPtr, length: MemLength) -> Vec<u8>;

    /// Writes to instance memory.
    ///
    /// ## Safety
    ///
    /// The offset and the length must point to valid instance memory.
    unsafe fn memory_store(&self, mem_ptr: MemPtr, data: &[u8]);

    fn m_types_lock(&self) -> MutexGuard<'_, ManagedTypeContainer>;

    fn gas_schedule(&self) -> &GasSchedule;

    fn use_gas(&mut self, gas: u64) -> Result<(), VMHooksEarlyExit>;

    fn input_ref(&self) -> &TxInput;

    fn current_address(&self) -> &VMAddress {
        &self.input_ref().to
    }

    fn tx_hash(&self) -> H256 {
        self.input_ref().tx_hash.clone()
    }

    /// Random number generator, based on the blockchain randomness source.
    fn random_next_bytes(&self, length: usize) -> Vec<u8>;

    fn result_lock(&self) -> MutexGuard<'_, TxResult>;

    fn push_tx_log(&self, tx_log: TxLog) {
        self.result_lock().result_logs.push(tx_log);
    }

    fn storage_read(&self, key: &[u8]) -> Vec<u8> {
        self.storage_read_any_address(self.current_address(), key)
    }

    fn storage_read_any_address(&self, address: &VMAddress, key: &[u8]) -> Vec<u8>;

    fn storage_write(&mut self, key: &[u8], value: &[u8]) -> Result<(), VMHooksEarlyExit>;

    fn get_block_config(&self) -> &BlockConfig;

    fn back_transfers_lock(&self) -> MutexGuard<'_, BackTransfers>;

    /// For ownership reasons, needs to return a clone.
    ///
    /// Can be optimized, but is not a priority right now.
    fn account_data(&self, address: &VMAddress) -> Option<AccountData>;

    fn account_code(&self, address: &VMAddress) -> Vec<u8>;

    fn perform_async_call(
        &mut self,
        to: VMAddress,
        egld_value: num_bigint::BigUint,
        func_name: TxFunctionName,
        args: Vec<Vec<u8>>,
    ) -> Result<(), VMHooksEarlyExit>;

    fn perform_execute_on_dest_context(
        &mut self,
        to: VMAddress,
        egld_value: num_bigint::BigUint,
        func_name: TxFunctionName,
        args: Vec<Vec<u8>>,
    ) -> Result<TxResult, VMHooksEarlyExit>;

    fn perform_execute_on_dest_context_readonly(
        &mut self,
        to: VMAddress,
        func_name: TxFunctionName,
        arguments: Vec<Vec<u8>>,
    ) -> Result<Vec<Vec<u8>>, VMHooksEarlyExit>;

    fn perform_deploy(
        &mut self,
        egld_value: num_bigint::BigUint,
        contract_code: Vec<u8>,
        code_metadata: VMCodeMetadata,
        args: Vec<Vec<u8>>,
    ) -> Result<(VMAddress, Vec<Vec<u8>>), VMHooksEarlyExit>;

    fn perform_transfer_execute(
        &mut self,
        to: VMAddress,
        egld_value: num_bigint::BigUint,
        func_name: TxFunctionName,
        arguments: Vec<Vec<u8>>,
    ) -> Result<(), VMHooksEarlyExit>;
}
