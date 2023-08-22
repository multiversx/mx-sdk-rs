use std::sync::Arc;

use multiversx_chain_vm::{
    executor::{BreakpointValue, VMHooks},
    tx_mock::{TxContext, TxContextRef, TxContextStack, TxPanic},
    vm_hooks::{DebugApiVMHooksHandler, VMHooksDispatcher},
};
use multiversx_sc::err_msg;

use crate::debug_executor::{StaticVarData, StaticVarStack};

use super::{DebugHandle, VMHooksApi, VMHooksApiBackend};

#[derive(Clone)]
pub struct DebugApiBackend;

impl VMHooksApiBackend for DebugApiBackend {
    type HandleType = DebugHandle;

    fn with_vm_hooks<R, F>(f: F) -> R
    where
        F: FnOnce(&dyn VMHooks) -> R,
    {
        let top_context = TxContextStack::static_peek();
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
                err_msg::DEBUG_API_ERR_STATUS,
                err_msg::DEBUG_API_ERR_HANDLE_STALE,
            );
        }
    }
    fn with_static_data<R, F>(f: F) -> R
    where
        F: FnOnce(&StaticVarData) -> R,
    {
        let top_context = StaticVarStack::static_peek();
        f(&top_context)
    }
}

pub type DebugApi = VMHooksApi<DebugApiBackend>;

impl DebugApi {
    pub fn dummy() {
        let tx_context = TxContext::dummy();
        let tx_context_arc = Arc::new(tx_context);
        // TODO: WARNING: this does not clean up after itself, must fix!!!
        TxContextStack::static_push(tx_context_arc);
        StaticVarStack::static_push();
    }
}

impl std::fmt::Debug for DebugApi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("DebugApi")
    }
}

fn debugger_panic(status: u64, message: &str) {
    TxContextRef::new_from_static().replace_tx_result_with_error(TxPanic::new(status, message));
    std::panic::panic_any(BreakpointValue::SignalError);
}

fn assert_handles_on_same_context(handle1: &DebugHandle, handle2: &DebugHandle) {
    if !handle1.is_on_same_context(handle2) {
        debugger_panic(
            err_msg::DEBUG_API_ERR_STATUS,
            err_msg::DEBUG_API_ERR_HANDLE_CONTEXT_MISMATCH,
        );
    }
}
