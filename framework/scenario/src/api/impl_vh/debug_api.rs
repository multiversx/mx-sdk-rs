use std::{
    ops::DerefMut,
    sync::{Arc, Mutex},
};

use multiversx_chain_vm::{
    executor::{BreakpointValue, VMHooks},
    tx_mock::{TxContext, TxContextRef, TxPanic},
    vm_hooks::{DebugApiVMHooksHandler, VMHooksDispatcher},
};
use multiversx_sc::{chain_core::types::ReturnCode, err_msg};

use crate::debug_executor::StaticVarData;

use super::{DebugHandle, VMHooksApi, VMHooksApiBackend};

thread_local!(
    static CURRENT_TX_CONTEXT: Mutex<Option<TxContextRef>> = Mutex::new(None)
);

thread_local!(
    static STATIC_VAR_DATA: Mutex<Option<Arc<StaticVarData>>> = Mutex::new(None)
);

#[derive(Clone)]
pub struct DebugApiBackend;

impl DebugApiBackend {
    pub fn get_current_tx_context() -> TxContextRef {
        let opt_ref = CURRENT_TX_CONTEXT.with(|cell| {
            let opt = cell
                .lock()
                .expect("error accessing the DebugApiBackend current context mutex contents");
            opt.clone()
        });
        opt_ref.expect("Uninitialized DebugApiBackend (current tx context missing)")
    }

    pub fn replace_current_tx_context(value: Option<TxContextRef>) -> Option<TxContextRef> {
        CURRENT_TX_CONTEXT.with(|cell| {
            let mut opt = cell
                .lock()
                .expect("error replacing the DebugApiBackend current context mutex contents");
            core::mem::replace(opt.deref_mut(), value)
        })
    }

    pub fn get_static_var_data() -> Arc<StaticVarData> {
        let opt_ref = STATIC_VAR_DATA.with(|cell| {
            let opt = cell
                .lock()
                .expect("error accessing the DebugApiBackend current static var contents");
            opt.clone()
        });
        opt_ref.expect("Uninitialized DebugApiBackend (static var data missing)")
    }

    pub fn replace_static_var_data(
        value: Option<Arc<StaticVarData>>,
    ) -> Option<Arc<StaticVarData>> {
        STATIC_VAR_DATA.with(|cell| {
            let mut opt = cell
                .lock()
                .expect("error replacing the DebugApiBackend current static var contents");
            core::mem::replace(opt.deref_mut(), value)
        })
    }
}

impl VMHooksApiBackend for DebugApiBackend {
    type HandleType = DebugHandle;

    fn with_vm_hooks<R, F>(f: F) -> R
    where
        F: FnOnce(&dyn VMHooks) -> R,
    {
        let top_context = Self::get_current_tx_context();
        let wrapper = DebugApiVMHooksHandler::new(top_context);
        let dispatcher = VMHooksDispatcher::new(Box::new(wrapper));
        f(&dispatcher)
    }

    fn with_vm_hooks_ctx_1<R, F>(handle: Self::HandleType, f: F) -> R
    where
        F: FnOnce(&dyn VMHooks) -> R,
    {
        let wrapper = DebugApiVMHooksHandler::new(handle.context);
        let dispatcher = VMHooksDispatcher::new(Box::new(wrapper));
        f(&dispatcher)
    }

    fn with_vm_hooks_ctx_2<R, F>(handle1: Self::HandleType, handle2: Self::HandleType, f: F) -> R
    where
        F: FnOnce(&dyn VMHooks) -> R,
    {
        assert_handles_on_same_context(&handle1, &handle2);
        Self::with_vm_hooks_ctx_1(handle1, f)
    }

    fn with_vm_hooks_ctx_3<R, F>(
        handle1: Self::HandleType,
        handle2: Self::HandleType,
        handle3: Self::HandleType,
        f: F,
    ) -> R
    where
        F: FnOnce(&dyn VMHooks) -> R,
    {
        assert_handles_on_same_context(&handle1, &handle2);
        assert_handles_on_same_context(&handle1, &handle3);
        Self::with_vm_hooks_ctx_1(handle1, f)
    }

    fn assert_live_handle(handle: &Self::HandleType) {
        if !handle.is_on_current_context() {
            debugger_panic(
                ReturnCode::DebugApiError,
                err_msg::DEBUG_API_ERR_HANDLE_STALE,
            );
        }
    }
    fn with_static_data<R, F>(f: F) -> R
    where
        F: FnOnce(&StaticVarData) -> R,
    {
        let static_var_data = Self::get_static_var_data();
        f(&static_var_data)
    }
}

pub type DebugApi = VMHooksApi<DebugApiBackend>;

impl DebugApi {
    /// WARNING: this does not clean up after itself, must fix!!!
    pub fn dummy() {
        let tx_context = TxContext::dummy();
        let tx_context_arc = Arc::new(tx_context);

        DebugApiBackend::replace_current_tx_context(Some(TxContextRef(tx_context_arc)));
        DebugApiBackend::replace_static_var_data(Some(Arc::new(StaticVarData::default())));
    }

    pub fn get_current_tx_context() -> TxContextRef {
        DebugApiBackend::get_current_tx_context()
    }
}

impl std::fmt::Debug for DebugApi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("DebugApi")
    }
}

fn debugger_panic(status: ReturnCode, message: &str) {
    DebugApi::get_current_tx_context().replace_tx_result_with_error(TxPanic::new(status, message));
    std::panic::panic_any(BreakpointValue::SignalError);
}

fn assert_handles_on_same_context(handle1: &DebugHandle, handle2: &DebugHandle) {
    if !handle1.is_on_same_context(handle2) {
        debugger_panic(
            ReturnCode::DebugApiError,
            err_msg::DEBUG_API_ERR_HANDLE_CONTEXT_MISMATCH,
        );
    }
}
