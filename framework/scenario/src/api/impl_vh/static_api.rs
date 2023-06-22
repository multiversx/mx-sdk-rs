use std::ops::Deref;

use multiversx_chain_vm::{
    executor::VMHooks,
    tx_mock::StaticVarData,
    vm_hooks::{TxManagedTypesCell, VMHooksDispatcher, VMHooksHandler},
};
use multiversx_sc::api::RawHandle;

use super::{VMHooksApi, VMHooksApiBackend};

fn new_vh_dispatcher_managed_types_cell() -> Box<dyn VMHooks> {
    let vh_handler: Box<dyn VMHooksHandler> = Box::<TxManagedTypesCell>::default();
    Box::new(VMHooksDispatcher::new(vh_handler))
}

thread_local! {
    static MANAGED_TYPES_CELL: Box<dyn VMHooks> = new_vh_dispatcher_managed_types_cell();

    static STATIC_VAR_DATA_CELL: StaticVarData = StaticVarData::default();
}

#[derive(Clone)]
pub struct StaticApiBackend;

impl VMHooksApiBackend for StaticApiBackend {
    type HandleType = RawHandle;

    fn with_vm_hooks<R, F>(f: F) -> R
    where
        F: FnOnce(&dyn VMHooks) -> R,
    {
        MANAGED_TYPES_CELL.with(|vh| f(vh.deref()))
    }

    fn with_static_data<R, F>(f: F) -> R
    where
        F: FnOnce(&StaticVarData) -> R,
    {
        STATIC_VAR_DATA_CELL.with(|data| f(data))
    }
}

pub type StaticApi = VMHooksApi<StaticApiBackend>;

impl std::fmt::Debug for StaticApi {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("StaticApi").finish()
    }
}
