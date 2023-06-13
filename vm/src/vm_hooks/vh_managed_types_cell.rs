use std::cell::{Ref, RefCell, RefMut};

use crate::tx_mock::{TxInput, TxManagedTypes, TxResult};

use super::{
    VMHooksBigInt, VMHooksCallValue, VMHooksEndpointArgument, VMHooksEndpointFinish, VMHooksError,
    VMHooksErrorManaged, VMHooksHandler, VMHooksHandlerSource, VMHooksManagedBuffer,
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

    fn input_ref(&self) -> &TxInput {
        panic!("cannot access tx inputs in the StaticApi")
    }

    fn result_borrow_mut(&self) -> RefMut<TxResult> {
        panic!("cannot access tx results in the StaticApi")
    }
}

impl VMHooksBigInt for TxManagedTypesCell {}
impl VMHooksManagedBuffer for TxManagedTypesCell {}
impl VMHooksManagedTypes for TxManagedTypesCell {}
impl VMHooksHandler for TxManagedTypesCell {}

impl VMHooksCallValue for TxManagedTypesCell {}
impl VMHooksEndpointArgument for TxManagedTypesCell {}
impl VMHooksEndpointFinish for TxManagedTypesCell {}
impl VMHooksError for TxManagedTypesCell {}
impl VMHooksErrorManaged for TxManagedTypesCell {}
