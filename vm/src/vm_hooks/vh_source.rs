use std::{
    cell::{Ref, RefMut},
    fmt::Debug,
};

use multiversx_sc::types::{Address, CodeMetadata, H256};

use crate::{
    tx_mock::{TxFunctionName, TxInput, TxLog, TxManagedTypes, TxResult},
    world_mock::{AccountData, BlockInfo},
};

/// Abstracts away the borrowing of a managed types structure.
pub trait VMHooksHandlerSource: Debug {
    fn m_types_borrow(&self) -> Ref<TxManagedTypes>;

    fn m_types_borrow_mut(&self) -> RefMut<TxManagedTypes>;

    fn input_ref(&self) -> &TxInput;

    fn tx_hash(&self) -> H256 {
        self.input_ref().tx_hash.clone()
    }

    fn result_borrow_mut(&self) -> RefMut<TxResult>;

    fn push_tx_log(&self, tx_log: TxLog) {
        self.result_borrow_mut().result_logs.push(tx_log);
    }

    fn storage_read(&self, key: &[u8]) -> Vec<u8> {
        self.storage_read_any_address(&self.input_ref().to, key)
    }

    fn storage_read_any_address(&self, address: &Address, key: &[u8]) -> Vec<u8>;

    fn storage_write(&self, key: &[u8], value: &[u8]);

    fn get_previous_block_info(&self) -> &BlockInfo;

    fn get_current_block_info(&self) -> &BlockInfo;

    /// For ownership reasons, needs to return a clone.
    ///
    /// Can be optimized, but is not a priority right now.
    fn account_data(&self, address: &Address) -> AccountData;

    /// For ownership reasons, needs to return a clone.
    ///
    /// Can be optimized, but is not a priority right now.
    fn current_account_data(&self) -> AccountData {
        self.account_data(&self.input_ref().to)
    }

    fn account_code(&self, address: &Address) -> Vec<u8>;

    fn perform_async_call(
        &self,
        to: Address,
        egld_value: num_bigint::BigUint,
        func_name: TxFunctionName,
        args: Vec<Vec<u8>>,
    ) -> !;

    fn perform_execute_on_dest_context(
        &self,
        to: Address,
        egld_value: num_bigint::BigUint,
        func_name: TxFunctionName,
        args: Vec<Vec<u8>>,
    ) -> Vec<Vec<u8>>;

    fn perform_deploy(
        &self,
        egld_value: num_bigint::BigUint,
        contract_code: Vec<u8>,
        code_metadata: CodeMetadata,
        args: Vec<Vec<u8>>,
    ) -> (Address, Vec<Vec<u8>>);

    fn perform_transfer_execute(
        &self,
        to: Address,
        egld_value: num_bigint::BigUint,
        func_name: TxFunctionName,
        arguments: Vec<Vec<u8>>,
    );
}
