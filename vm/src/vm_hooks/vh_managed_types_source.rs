use std::{
    cell::{Ref, RefMut},
    fmt::Debug,
};

use crate::tx_mock::TxManagedTypes;

pub trait ManagedTypesSource: Debug {
    fn m_types_borrow(&self) -> Ref<TxManagedTypes>;

    fn m_types_borrow_mut(&self) -> RefMut<TxManagedTypes>;
}
