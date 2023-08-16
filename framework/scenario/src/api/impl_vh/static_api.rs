use multiversx_chain_vm::{
    executor::VMHooks,
    vm_hooks::{StaticApiVMHooksHandler, VMHooksDispatcher, VMHooksHandler},
};
use multiversx_sc::{api::RawHandle, types::Address};

use crate::debug_executor::StaticVarData;

use super::{VMHooksApi, VMHooksApiBackend};

fn new_static_api_vh() -> VMHooksDispatcher {
    let vh_handler: Box<dyn VMHooksHandler> = Box::<StaticApiVMHooksHandler>::default();
    VMHooksDispatcher::new(vh_handler)
}

thread_local! {
    static STATIC_API_VH_CELL: VMHooksDispatcher = new_static_api_vh();

    static STATIC_API_STATIC_CELL: StaticVarData = StaticVarData::default();
}

#[derive(Clone)]
pub struct StaticApiBackend;

impl VMHooksApiBackend for StaticApiBackend {
    type HandleType = RawHandle;

    fn with_vm_hooks<R, F>(f: F) -> R
    where
        F: FnOnce(&dyn VMHooks) -> R,
    {
        STATIC_API_VH_CELL.with(|vh| f(vh))
    }

    fn with_static_data<R, F>(f: F) -> R
    where
        F: FnOnce(&StaticVarData) -> R,
    {
        STATIC_API_STATIC_CELL.with(|data| f(data))
    }
}

pub type StaticApi = VMHooksApi<StaticApiBackend>;

impl StaticApi {
    /// The static API does not allow interrogating the Tx input,
    /// but does offer a placeholder for the current ("SC") address, to help with contract calls.
    ///
    /// This placeholder then needs to be converted to something useful.
    pub fn is_current_address_placeholder(address: &Address) -> bool {
        address.as_array() == StaticApiVMHooksHandler::CURRENT_ADDRESS_PLACEHOLDER.as_array()
    }
}

impl std::fmt::Debug for StaticApi {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("StaticApi").finish()
    }
}
