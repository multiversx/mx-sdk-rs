use multiversx_chain_vm::{executor::VMHooks, host::vm_hooks::VMHooksDispatcher};
use multiversx_sc::{api::RawHandle, types::Address};
use std::sync::Mutex;

use crate::executor::debug::StaticVarData;

use super::{StaticApiVMHooksHandler, VMHooksApi, VMHooksApiBackend};

fn new_static_api_vh() -> VMHooksDispatcher<StaticApiVMHooksHandler> {
    VMHooksDispatcher::new(StaticApiVMHooksHandler::default())
}

thread_local! {
    static STATIC_API_VH_CELL: Mutex<VMHooksDispatcher<StaticApiVMHooksHandler>> = Mutex::new(new_static_api_vh());

    static STATIC_API_STATIC_CELL: Mutex<StaticVarData> = Mutex::new(StaticVarData::default());
}

#[derive(Clone)]
pub struct StaticApiBackend;

impl VMHooksApiBackend for StaticApiBackend {
    type HandleType = RawHandle;

    fn with_vm_hooks<R, F>(f: F) -> R
    where
        F: FnOnce(&mut dyn VMHooks) -> R,
    {
        STATIC_API_VH_CELL.with(|vh_mutex| {
            let mut vh = vh_mutex.lock().unwrap();
            f(&mut *vh)
        })
    }

    fn with_static_data<R, F>(f: F) -> R
    where
        F: FnOnce(&StaticVarData) -> R,
    {
        STATIC_API_STATIC_CELL.with(|data_mutex| {
            let data = data_mutex.lock().unwrap();
            f(&data)
        })
    }
}

pub type StaticApi = VMHooksApi<StaticApiBackend>;

impl StaticApi {
    /// The static API does not allow interrogating the Tx input,
    /// but does offer a placeholder for the current ("SC") address, to help with contract calls.
    ///
    /// This placeholder then needs to be converted to something useful.
    pub fn is_current_address_placeholder(address: &Address) -> bool {
        address == &StaticApiVMHooksHandler::CURRENT_ADDRESS_PLACEHOLDER
    }

    pub fn reset() {
        STATIC_API_VH_CELL.with(|vh_mutex| {
            let mut vh = vh_mutex.lock().unwrap();
            *vh = new_static_api_vh()
        });

        STATIC_API_STATIC_CELL.with(|data_mutex| {
            let mut data = data_mutex.lock().unwrap();
            *data = StaticVarData::default()
        })
    }
}

impl std::fmt::Debug for StaticApi {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("StaticApi").finish()
    }
}
