use std::{
    cell::{Ref, RefMut},
    fmt::Debug,
};

use multiversx_sc::types::Address;

use crate::{
    tx_mock::{TxInput, TxManagedTypes, TxResult},
    world_mock::{AccountData, BlockInfo},
};

/// Abstracts away the borrowing of a managed types structure.
pub trait VMHooksHandlerSource: Debug {
    fn m_types_borrow(&self) -> Ref<TxManagedTypes>;

    fn m_types_borrow_mut(&self) -> RefMut<TxManagedTypes>;

    fn input_ref(&self) -> &TxInput;

    fn result_borrow_mut(&self) -> RefMut<TxResult>;

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
}
