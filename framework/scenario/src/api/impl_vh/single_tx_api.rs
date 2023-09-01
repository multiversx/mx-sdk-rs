use std::sync::Mutex;

use multiversx_chain_vm::{
    executor::VMHooks,
    types::VMAddress,
    vm_hooks::{SingleTxApiData, SingleTxApiVMHooksHandler, VMHooksDispatcher},
    world_mock::AccountData,
};
use multiversx_sc::api::RawHandle;

use crate::debug_executor::StaticVarData;

use super::{VMHooksApi, VMHooksApiBackend};

thread_local! {
    static SINGLE_TX_API_VH_CELL: Mutex<SingleTxApiVMHooksHandler> = Mutex::default();

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
            let handler = cell.lock().unwrap().clone();
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
            let _ = std::mem::take(&mut *cell.lock().unwrap());
        })
    }

    pub fn with_global<F, R>(f: F) -> R
    where
        F: FnOnce(&mut SingleTxApiData) -> R,
    {
        SINGLE_TX_API_VH_CELL.with(|cell| {
            let mut handler = cell.lock().unwrap();
            handler.with_mut_data(f)
        })
    }

    pub fn with_global_default_account<F, R>(f: F) -> R
    where
        F: FnOnce(&mut AccountData) -> R,
    {
        Self::with_global(|data| data.with_account_mut(&VMAddress::zero(), f))
    }
}

impl std::fmt::Debug for SingleTxApi {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SingleTxApi").finish()
    }
}
