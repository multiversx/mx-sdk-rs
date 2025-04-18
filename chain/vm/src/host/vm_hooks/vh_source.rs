use std::{fmt::Debug, sync::MutexGuard};

use multiversx_chain_core::types::ReturnCode;
use multiversx_chain_vm_executor::{MemLength, MemPtr};

use crate::{
    blockchain::state::{AccountData, BlockInfo},
    host::context::{
        BackTransfers, ManagedTypeContainer, TxFunctionName, TxInput, TxLog, TxResult,
    },
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

    fn halt_with_error(&mut self, status: ReturnCode, message: &str);

    fn vm_error(&mut self, message: &str) {
        self.halt_with_error(ReturnCode::ExecutionFailed, message)
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

    fn storage_write(&mut self, key: &[u8], value: &[u8]);

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

    fn perform_async_call(
        &mut self,
        to: VMAddress,
        egld_value: num_bigint::BigUint,
        func_name: TxFunctionName,
        args: Vec<Vec<u8>>,
    ) -> !;

    fn perform_execute_on_dest_context(
        &mut self,
        to: VMAddress,
        egld_value: num_bigint::BigUint,
        func_name: TxFunctionName,
        args: Vec<Vec<u8>>,
    ) -> Vec<Vec<u8>>;

    fn perform_execute_on_dest_context_readonly(
        &mut self,
        to: VMAddress,
        func_name: TxFunctionName,
        arguments: Vec<Vec<u8>>,
    ) -> Vec<Vec<u8>>;

    fn perform_deploy(
        &mut self,
        egld_value: num_bigint::BigUint,
        contract_code: Vec<u8>,
        code_metadata: VMCodeMetadata,
        args: Vec<Vec<u8>>,
    ) -> (VMAddress, Vec<Vec<u8>>);

    fn perform_transfer_execute(
        &mut self,
        to: VMAddress,
        egld_value: num_bigint::BigUint,
        func_name: TxFunctionName,
        arguments: Vec<Vec<u8>>,
    );
}
