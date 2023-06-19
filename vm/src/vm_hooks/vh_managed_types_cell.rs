use std::cell::{Ref, RefCell, RefMut};

use multiversx_sc::types::Address;

use crate::{
    tx_mock::{TxInput, TxLog, TxManagedTypes, TxResult},
    world_mock::{AccountData, BlockInfo},
};

use super::{
    VMHooksBigInt, VMHooksBlockchain, VMHooksCallValue, VMHooksCrypto, VMHooksEndpointArgument,
    VMHooksEndpointFinish, VMHooksError, VMHooksErrorManaged, VMHooksHandler, VMHooksHandlerSource,
    VMHooksLog, VMHooksManagedBuffer, VMHooksManagedTypes, VMHooksStorageRead, VMHooksStorageWrite,
};

/// A simple wrapper around a managed type container RefCell.
///
/// Implements `VMHooksManagedTypes` and thus can be used as a basis of a minimal static API.
#[derive(Debug, Default)]
pub struct TxManagedTypesCell(RefCell<TxManagedTypes>);

impl VMHooksHandlerSource for TxManagedTypesCell {
    fn m_types_borrow(&self) -> Ref<TxManagedTypes> {
        self.0.borrow()
    }

    fn m_types_borrow_mut(&self) -> RefMut<TxManagedTypes> {
        self.0.borrow_mut()
    }

    fn input_ref(&self) -> &TxInput {
        panic!("cannot access tx inputs in the StaticApi")
    }

    fn result_borrow_mut(&self) -> RefMut<TxResult> {
        panic!("cannot access tx results in the StaticApi")
    }

    fn push_tx_log(&self, _tx_log: TxLog) {
        panic!("cannot log events in the StaticApi")
    }

    fn storage_read_any_address(&self, _address: &Address, _key: &[u8]) -> Vec<u8> {
        panic!("cannot access the storage in the StaticApi")
    }

    fn storage_write(&self, _key: &[u8], _value: &[u8]) {
        panic!("cannot access the storage in the StaticApi")
    }

    fn get_previous_block_info(&self) -> &BlockInfo {
        panic!("cannot access the block info in the StaticApi")
    }

    fn get_current_block_info(&self) -> &BlockInfo {
        panic!("cannot access the block info in the StaticApi")
    }

    fn account_data(&self, _address: &Address) -> AccountData {
        panic!("cannot access account data in the StaticApi")
    }
}

impl VMHooksBigInt for TxManagedTypesCell {}
impl VMHooksManagedBuffer for TxManagedTypesCell {}
impl VMHooksManagedTypes for TxManagedTypesCell {}

impl VMHooksCallValue for TxManagedTypesCell {}
impl VMHooksEndpointArgument for TxManagedTypesCell {}
impl VMHooksEndpointFinish for TxManagedTypesCell {}
impl VMHooksError for TxManagedTypesCell {}
impl VMHooksErrorManaged for TxManagedTypesCell {}
impl VMHooksStorageRead for TxManagedTypesCell {}
impl VMHooksStorageWrite for TxManagedTypesCell {}
impl VMHooksCrypto for TxManagedTypesCell {}
impl VMHooksBlockchain for TxManagedTypesCell {}
impl VMHooksLog for TxManagedTypesCell {}

impl VMHooksHandler for TxManagedTypesCell {}
