use std::{
    cell::{Ref, RefMut},
    fmt::Debug,
};

use crate::tx_mock::TxManagedTypes;

/// Abstracts away the borrowing of a managed types structure.
pub trait ManagedTypesSource: Debug {
    fn m_types_borrow(&self) -> Ref<TxManagedTypes>;

    fn m_types_borrow_mut(&self) -> RefMut<TxManagedTypes>;
}
