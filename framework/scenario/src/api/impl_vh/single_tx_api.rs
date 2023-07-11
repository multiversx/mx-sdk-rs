use std::cell::RefCell;

use multiversx_chain_vm::{
    executor::VMHooks,
    vm_hooks::{SingleTxApiData, SingleTxApiVMHooksHandler, VMHooksDispatcher},
};
use multiversx_sc::api::RawHandle;

use crate::debug_executor::StaticVarData;

use super::{VMHooksApi, VMHooksApiBackend};

thread_local! {
    static SINGLE_TX_API_VH_CELL: RefCell<SingleTxApiVMHooksHandler> = RefCell::default();

    static SINGLE_TX_API_STATIC_CELL: StaticVarData = StaticVarData::default();
}

#[derive(Clone)]
pub struct SingleTxApiBackend;

impl VMHooksApiBackend for SingleTxApiBackend {
    type HandleType = RawHandle;

    fn with_vm_hooks<R, F>(f: F) -> R
    where
        F: FnOnce(&dyn VMHooks) -> R,
    {
        SINGLE_TX_API_VH_CELL.with(|cell| {
            let handler = cell.borrow().clone();
            let dispatcher = VMHooksDispatcher::new(Box::new(handler));
            f(&dispatcher)
        })
    }

    fn with_static_data<R, F>(f: F) -> R
    where
        F: FnOnce(&StaticVarData) -> R,
    {
        SINGLE_TX_API_STATIC_CELL.with(|data| f(data))
    }
}

/// Similar to the `StaticApi`, but offers allows calls to storage, input, and even creating results.
pub type SingleTxApi = VMHooksApi<SingleTxApiBackend>;

impl SingleTxApi {
    pub fn clear_global() {
        SINGLE_TX_API_VH_CELL.with(|cell| {
            let _ = cell.take();
        })
    }

    pub fn with_global<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut SingleTxApiData) -> R,
    {
        SINGLE_TX_API_VH_CELL.with(|cell| {
            let mut handler = cell.borrow_mut();
            handler.with_mut_data(f)
        })
    }
}

impl std::fmt::Debug for SingleTxApi {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SingleTxApi").finish()
    }
}
