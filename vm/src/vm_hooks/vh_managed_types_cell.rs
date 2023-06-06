use std::cell::{Ref, RefCell, RefMut};

use crate::tx_mock::TxManagedTypes;

use super::{
    VMHooksHandlerSource, VMHooksBigInt, VMHooksError, VMHooksHandler, VMHooksManagedBuffer,
    VMHooksManagedTypes,
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
}

impl VMHooksError for TxManagedTypesCell {}
impl VMHooksBigInt for TxManagedTypesCell {}
impl VMHooksManagedBuffer for TxManagedTypesCell {}
impl VMHooksManagedTypes for TxManagedTypesCell {}
impl VMHooksHandler for TxManagedTypesCell {}
