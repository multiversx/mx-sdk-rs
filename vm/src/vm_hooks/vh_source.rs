use std::{
    cell::{Ref, RefMut},
    fmt::Debug,
};

use crate::tx_mock::{TxInput, TxManagedTypes, TxResult};

/// Abstracts away the borrowing of a managed types structure.
pub trait VMHooksHandlerSource: Debug {
    fn m_types_borrow(&self) -> Ref<TxManagedTypes>;

    fn m_types_borrow_mut(&self) -> RefMut<TxManagedTypes>;

    fn input_ref(&self) -> &TxInput;

    fn result_borrow_mut(&self) -> RefMut<TxResult>;
}
