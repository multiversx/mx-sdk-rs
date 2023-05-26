use std::{
    cell::{Ref, RefCell, RefMut},
    fmt::Debug,
};

use crate::tx_mock::TxManagedTypes;

use super::{VMHooksBigInt, VMHooksError};

pub trait ManagedTypesSource: Debug {
    fn m_types_borrow(&self) -> Ref<TxManagedTypes>;

    fn m_types_borrow_mut(&self) -> RefMut<TxManagedTypes>;
}

pub trait VMHooksManagedTypes: VMHooksBigInt + Debug {}

#[derive(Debug, Default)]
pub struct TxManagedTypesCell(RefCell<TxManagedTypes>);

impl ManagedTypesSource for TxManagedTypesCell {
    fn m_types_borrow(&self) -> Ref<TxManagedTypes> {
        self.0.borrow()
    }

    fn m_types_borrow_mut(&self) -> RefMut<TxManagedTypes> {
        self.0.borrow_mut()
    }
}

impl VMHooksError for TxManagedTypesCell {}
impl VMHooksBigInt for TxManagedTypesCell {}
impl VMHooksManagedTypes for TxManagedTypesCell {}
