use std::{fmt::Debug, sync::MutexGuard};

use crate::{
    tx_mock::{TxFunctionName, TxInput, TxLog, TxManagedTypes, TxResult},
    types::{VMAddress, VMCodeMetadata, H256},
    world_mock::{AccountData, BlockInfo},
};

/// Abstracts away the borrowing of a managed types structure.
pub trait VMHooksHandlerSource: Debug {
    fn m_types_lock(&self) -> MutexGuard<TxManagedTypes>;

    fn halt_with_error(&self, status: u64, message: &str) -> !;

    fn vm_error(&self, message: &str) -> ! {
        self.halt_with_error(10, message)
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

    fn storage_write(&self, key: &[u8], value: &[u8]);

    fn get_previous_block_info(&self) -> &BlockInfo;

    fn get_current_block_info(&self) -> &BlockInfo;

    /// For ownership reasons, needs to return a clone.
    ///
    /// Can be optimized, but is not a priority right now.
    fn account_data(&self, address: &VMAddress) -> AccountData;

    /// For ownership reasons, needs to return a clone.
    ///
    /// Can be optimized, but is not a priority right now.
    fn current_account_data(&self) -> AccountData {
        self.account_data(&self.input_ref().to)
    }

    fn account_code(&self, address: &VMAddress) -> Vec<u8>;

    fn perform_async_call(
        &self,
        to: VMAddress,
        egld_value: num_bigint::BigUint,
        func_name: TxFunctionName,
        args: Vec<Vec<u8>>,
    ) -> !;

    fn perform_execute_on_dest_context(
        &self,
        to: VMAddress,
        egld_value: num_bigint::BigUint,
        func_name: TxFunctionName,
        args: Vec<Vec<u8>>,
    ) -> Vec<Vec<u8>>;

    fn perform_deploy(
        &self,
        egld_value: num_bigint::BigUint,
        contract_code: Vec<u8>,
        code_metadata: VMCodeMetadata,
        args: Vec<Vec<u8>>,
    ) -> (VMAddress, Vec<Vec<u8>>);

    fn perform_transfer_execute(
        &self,
        to: VMAddress,
        egld_value: num_bigint::BigUint,
        func_name: TxFunctionName,
        arguments: Vec<Vec<u8>>,
    );
}
