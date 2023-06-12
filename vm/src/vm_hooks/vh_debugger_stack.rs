use std::{
    cell::{Ref, RefMut},
    rc::Rc,
};

use crate::tx_mock::{TxContext, TxInput, TxManagedTypes, TxResult};

use super::{
    VMHooksBigInt, VMHooksCallValue, VMHooksEndpointArgument, VMHooksEndpointFinish, VMHooksError,
    VMHooksHandler, VMHooksHandlerSource, VMHooksManagedBuffer, VMHooksManagedTypes,
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
}

impl VMHooksBigInt for TxContextWrapper {}
impl VMHooksManagedBuffer for TxContextWrapper {}
impl VMHooksManagedTypes for TxContextWrapper {}
impl VMHooksHandler for TxContextWrapper {}

impl VMHooksCallValue for TxContextWrapper {}
impl VMHooksEndpointArgument for TxContextWrapper {}
impl VMHooksEndpointFinish for TxContextWrapper {}
impl VMHooksError for TxContextWrapper {}
