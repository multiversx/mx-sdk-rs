mod backend_type;
mod blockchain_api_vh;
mod call_value_api_vh;
mod crypto_api_vh;
mod endpoint_arg_api_vh;
mod endpoint_finish_api_vh;
mod error_api_vh;
mod log_api_vh;
mod managed_type_api_vh;
mod print_api_vh;
mod send_api_vh;
mod storage_api_vh;
mod vm_api_vh;

pub use backend_type::*;

use std::ops::Deref;

use multiversx_chain_vm::{
    executor::{MemPtr, VMHooks},
    tx_mock::{StaticVarData, StaticVarStack, TxContextStack},
    vm_hooks::{TxContextWrapper, TxManagedTypesCell, VMHooksDispatcher, VMHooksHandler},
};
use multiversx_sc::api::{HandleTypeInfo, ManagedBufferApiImpl, RawHandle};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VMHooksApi<const BACKEND_TYPE: VMHooksBackendType>;

impl<const BACKEND_TYPE: VMHooksBackendType> HandleTypeInfo for VMHooksApi<BACKEND_TYPE> {
    type ManagedBufferHandle = RawHandle;
    type BigIntHandle = RawHandle;
    type BigFloatHandle = RawHandle;
    type EllipticCurveHandle = RawHandle;
    type ManagedMapHandle = RawHandle;
}

fn new_vh_dispatcher_managed_types_cell() -> Box<dyn VMHooks> {
    let vh_handler: Box<dyn VMHooksHandler> = Box::<TxManagedTypesCell>::default();
    Box::new(VMHooksDispatcher::new(vh_handler))
}

thread_local! {
    static MANAGED_TYPES_CELL: Box<dyn VMHooks> = new_vh_dispatcher_managed_types_cell();

    static STATIC_VAR_DATA_CELL: StaticVarData = StaticVarData::default();
}

impl<const BACKEND_TYPE: VMHooksBackendType> VMHooksApi<BACKEND_TYPE> {
    pub fn api_impl() -> VMHooksApi<BACKEND_TYPE> {
        VMHooksApi {}
    }

    /// All communication with the VM happens via this method.
    pub fn with_vm_hooks<R, F>(&self, f: F) -> R
    where
        F: FnOnce(&dyn VMHooks) -> R,
    {
        match BACKEND_TYPE {
            STATIC_MANAGED_TYPES => MANAGED_TYPES_CELL.with(|vh| f(vh.deref())),
            DEBUGGER_STACK => {
                let top_context = TxContextStack::static_peek();
                let wrapper = TxContextWrapper::new(top_context);
                let dispatcher = VMHooksDispatcher::new(Box::new(wrapper));
                f(&dispatcher)
            },
            _ => panic!("invalid VMHooksBackendType"),
        }
    }

    /// Static data does not belong to the VM, or to the VM hooks. It belongs to the contract only.
    pub fn with_static_data<R, F>(&self, f: F) -> R
    where
        F: FnOnce(&StaticVarData) -> R,
    {
        match BACKEND_TYPE {
            STATIC_MANAGED_TYPES => STATIC_VAR_DATA_CELL.with(|data| f(data)),
            DEBUGGER_STACK => {
                let top_context = StaticVarStack::static_peek();
                f(&top_context)
            },
            _ => panic!("invalid VMHooksBackendType"),
        }
    }

    /// Convenience method for calling VM hooks with a pointer to a temporary buffer in which we load a managed buffer.
    ///
    /// It is used for
    /// - addresses
    /// - token identifiers.
    ///
    /// The buffer is 32 bytes long, enough for both addresses and token identifiers.
    pub(crate) fn with_temp_buffer_ptr<R, F>(&self, handle: RawHandle, f: F) -> R
    where
        F: FnOnce(MemPtr) -> R,
    {
        let mut temp_buffer = [0u8; 32];
        self.mb_load_slice(handle, 0, &mut temp_buffer[..])
            .expect("error extracting address bytes");
        f(temp_buffer.as_ptr() as MemPtr)
    }
}

pub type StaticApi = VMHooksApi<STATIC_MANAGED_TYPES>;

pub type DebuggerApi = VMHooksApi<DEBUGGER_STACK>;
