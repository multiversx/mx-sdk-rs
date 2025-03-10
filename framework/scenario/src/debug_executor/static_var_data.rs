use std::cell::RefCell;

use multiversx_sc::types::LockableStaticBuffer;

use super::TxStaticVars;

#[derive(Debug, Default)]
pub struct StaticVarData {
    pub lockable_static_buffer_cell: RefCell<LockableStaticBuffer>,
    pub static_vars_cell: RefCell<TxStaticVars>,
}
