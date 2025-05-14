use std::{fmt::Debug, sync::MutexGuard};

use multiversx_chain_vm_executor::{MemLength, MemPtr, VMHooksEarlyExit};

use crate::{
    blockchain::state::{AccountData, BlockInfo},
    host::context::{
        BackTransfers, ManagedTypeContainer, TxFunctionName, TxInput, TxLog, TxResult,
    },
    schedule::GasSchedule,
    types::{VMAddress, VMCodeMetadata, H256},
};

/// Abstracts away the borrowing of a managed types structure.
pub trait VMHooksHandlerSource: Debug {
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

    fn m_types_lock(&self) -> MutexGuard<ManagedTypeContainer>;

    fn gas_schedule(&self) -> &GasSchedule;

    fn use_gas(&mut self, gas: u64) -> Result<(), VMHooksEarlyExit>;

    /// Shortcut for consuming gas for data copies, based on copied data length.
    fn use_gas_for_data_copy(&mut self, num_bytes_copied: usize) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(
            num_bytes_copied as u64 * self.gas_schedule().base_operation_cost.data_copy_per_byte,
        )
    }

    fn input_ref(&self) -> &TxInput;

    fn current_address(&self) -> &VMAddress {
        &self.input_ref().to
    }

    fn tx_hash(&self) -> H256 {
        self.input_ref().tx_hash.clone()
    }

    /// Random number generator, based on the blockchain randomness source.
    fn random_next_bytes(&self, length: usize) -> Vec<u8>;

    fn result_lock(&self) -> MutexGuard<TxResult>;

    fn push_tx_log(&self, tx_log: TxLog) {
        self.result_lock().result_logs.push(tx_log);
    }

    fn storage_read(&self, key: &[u8]) -> Vec<u8> {
        self.storage_read_any_address(self.current_address(), key)
    }

    fn storage_read_any_address(&self, address: &VMAddress, key: &[u8]) -> Vec<u8>;

    fn storage_write(&mut self, key: &[u8], value: &[u8]) -> Result<(), VMHooksEarlyExit>;

    fn get_previous_block_info(&self) -> &BlockInfo;

    fn get_current_block_info(&self) -> &BlockInfo;

    fn back_transfers_lock(&self) -> MutexGuard<BackTransfers>;

    /// For ownership reasons, needs to return a clone.
    ///
    /// Can be optimized, but is not a priority right now.
    fn account_data(&self, address: &VMAddress) -> Option<AccountData>;

    /// For ownership reasons, needs to return a clone.
    ///
    /// Can be optimized, but is not a priority right now.
    fn current_account_data(&self) -> AccountData {
        self.account_data(&self.input_ref().to)
            .expect("missing current account")
    }

    fn account_code(&self, address: &VMAddress) -> Vec<u8>;

    /// Utility function used in set_vec_of_esdt_transfers (present in multiple interfaces)
    /// Will probably be moved in future commits.
    fn calculate_set_vec_of_bytes_gas_cost(&self, len: usize) -> Result<u64, VMHooksEarlyExit> {
        let len_u64 = len as u64;
        let total_gas = len_u64 * self.gas_schedule().managed_buffer_api_cost.m_buffer_new
            + self
                .gas_schedule()
                .managed_buffer_api_cost
                .m_buffer_set_bytes;

        Ok(total_gas)
    }

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
    ) -> Result<Vec<Vec<u8>>, VMHooksEarlyExit>;

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
