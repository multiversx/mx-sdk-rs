use std::rc::Rc;

use multiversx_chain_vm::{
    executor::{BreakpointValue, VMHooks},
    host::tx_mock::{TxContextRef, TxPanic},
    host::vm_hooks::{TxContextVMHooksHandler, VMHooksDispatcher},
};
use multiversx_chain_vm_executor::Instance;
use multiversx_sc::{chain_core::types::ReturnCode, err_msg};

use crate::debug_executor::{ContractDebugInstance, ContractDebugStack, StaticVarData};

use super::{DebugHandle, VMHooksApi, VMHooksApiBackend};

#[derive(Clone)]
pub struct DebugApiBackend;

impl VMHooksApiBackend for DebugApiBackend {
    type HandleType = DebugHandle;

    fn with_vm_hooks<R, F>(f: F) -> R
    where
        F: FnOnce(&dyn VMHooks) -> R,
    {
        let instance = ContractDebugStack::static_peek();
        let tx_context_ref = instance.tx_context_ref.clone();
        let instance_rc: Rc<Box<dyn Instance>> = Rc::new(Box::new(instance));
        let handler = TxContextVMHooksHandler::new(tx_context_ref, Rc::downgrade(&instance_rc));
        let dispatcher = VMHooksDispatcher::new(Box::new(handler));
        let result = f(&dispatcher);
        std::mem::drop(instance_rc); // making sure the strong reference survives long enough
        result
    }

    fn with_vm_hooks_ctx_1<R, F>(handle: Self::HandleType, f: F) -> R
    where
        F: FnOnce(&dyn VMHooks) -> R,
    {
        let tx_context_ref = TxContextRef(handle.context.clone());
        let instance = ContractDebugStack::find_by_tx_context(&tx_context_ref);
        let instance_rc: Rc<Box<dyn Instance>> = Rc::new(Box::new(instance));
        let handler = TxContextVMHooksHandler::new(tx_context_ref, Rc::downgrade(&instance_rc));
        let dispatcher = VMHooksDispatcher::new(Box::new(handler));
        let result = f(&dispatcher);
        std::mem::drop(instance_rc); // making sure the strong reference survives long enough
        result
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
        let top_static_vars = ContractDebugStack::static_peek().static_var_ref;
        f(&top_static_vars)
    }
}

pub type DebugApi = VMHooksApi<DebugApiBackend>;

impl DebugApi {
    /// WARNING: this does not clean up after itself, must fix!!!
    pub fn dummy() {
        ContractDebugStack::static_push(ContractDebugInstance::dummy());
    }
}

impl std::fmt::Debug for DebugApi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("DebugApi")
    }
}

fn debugger_panic(status: ReturnCode, message: &str) {
    ContractDebugStack::static_peek()
        .tx_context_ref
        .replace_tx_result_with_error(TxPanic::new(status, message));
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
