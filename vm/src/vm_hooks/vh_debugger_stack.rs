use std::{
    cell::{Ref, RefMut},
    rc::Rc,
};

use multiversx_sc::types::Address;

use crate::{
    tx_mock::{TxContext, TxInput, TxLog, TxManagedTypes, TxResult},
    world_mock::{check_reserved_key, AccountData, BlockInfo},
};

use super::{
    VMHooksBigInt, VMHooksBlockchain, VMHooksCallValue, VMHooksCrypto, VMHooksEndpointArgument,
    VMHooksEndpointFinish, VMHooksError, VMHooksErrorManaged, VMHooksHandler, VMHooksHandlerSource,
    VMHooksLog, VMHooksManagedBuffer, VMHooksManagedTypes, VMHooksStorageRead, VMHooksStorageWrite,
};

/// A simple wrapper around a managed type container RefCell.
///
/// Implements `VMHooksManagedTypes` and thus can be used as a basis of a minimal static API.
#[derive(Debug)]
pub struct TxContextWrapper(Rc<TxContext>);

impl TxContextWrapper {
    pub fn new(tx_context_rc: Rc<TxContext>) -> Self {
        TxContextWrapper(tx_context_rc)
    }
}

impl VMHooksHandlerSource for TxContextWrapper {
    fn m_types_borrow(&self) -> Ref<TxManagedTypes> {
        self.0.m_types_borrow()
    }

    fn m_types_borrow_mut(&self) -> RefMut<TxManagedTypes> {
        self.0.m_types_borrow_mut()
    }

    fn input_ref(&self) -> &TxInput {
        self.0.input_ref()
    }

    fn result_borrow_mut(&self) -> RefMut<TxResult> {
        self.0.result_borrow_mut()
    }

    fn storage_read_any_address(&self, address: &Address, key: &[u8]) -> Vec<u8> {
        self.0.with_account_mut(address, |account| {
            account.storage.get(key).cloned().unwrap_or_default()
        })
    }

    fn storage_write(&self, key: &[u8], value: &[u8]) {
        check_reserved_key(key);

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

    fn account_data(&self, address: &Address) -> AccountData {
        self.0.with_account(address, |account| account.clone())
    }

    fn push_tx_log(&self, tx_log: TxLog) {
        self.0.result_borrow_mut().result_logs.push(tx_log);
    }
}

impl VMHooksBigInt for TxContextWrapper {}
impl VMHooksManagedBuffer for TxContextWrapper {}
impl VMHooksManagedTypes for TxContextWrapper {}

impl VMHooksCallValue for TxContextWrapper {}
impl VMHooksEndpointArgument for TxContextWrapper {}
impl VMHooksEndpointFinish for TxContextWrapper {}
impl VMHooksError for TxContextWrapper {}
impl VMHooksErrorManaged for TxContextWrapper {}
impl VMHooksStorageRead for TxContextWrapper {}
impl VMHooksStorageWrite for TxContextWrapper {}
impl VMHooksCrypto for TxContextWrapper {}
impl VMHooksBlockchain for TxContextWrapper {}
impl VMHooksLog for TxContextWrapper {}

impl VMHooksHandler for TxContextWrapper {}
