use multiversx_chain_vm::{
    executor::{VMHooks, VMHooksEarlyExit},
    host::context::TxContextRef,
    host::vm_hooks::{TxVMHooksContext, VMHooksDispatcher},
};
use multiversx_sc::{chain_core::types::ReturnCode, err_msg};

use crate::executor::debug::{
    ContractDebugInstance, ContractDebugInstanceState, ContractDebugStack, StaticVarData,
};

use super::{DebugHandle, VMHooksApi, VMHooksApiBackend};

#[derive(Clone)]
pub struct DebugApiBackend;

impl VMHooksApiBackend for DebugApiBackend {
    type HandleType = DebugHandle;

    fn with_vm_hooks<R, F>(f: F) -> R
    where
        F: FnOnce(&mut dyn VMHooks) -> Result<R, VMHooksEarlyExit>,
    {
        let instance = ContractDebugStack::static_peek();
        let tx_context_ref = instance.tx_context_ref.clone();
        let vh_context = TxVMHooksContext::new(tx_context_ref, ContractDebugInstanceState);
        let mut dispatcher = VMHooksDispatcher::new(vh_context);
        f(&mut dispatcher).unwrap_or_else(|err| ContractDebugInstanceState::early_exit_panic(err))
    }

    fn with_vm_hooks_ctx_1<R, F>(handle: Self::HandleType, f: F) -> R
    where
        F: FnOnce(&mut dyn VMHooks) -> Result<R, VMHooksEarlyExit>,
    {
        let tx_context_ref = TxContextRef(handle.context.clone());
        let vh_context = TxVMHooksContext::new(tx_context_ref, ContractDebugInstanceState);
        let mut dispatcher = VMHooksDispatcher::new(vh_context);
        f(&mut dispatcher).unwrap_or_else(|err| ContractDebugInstanceState::early_exit_panic(err))
    }

    fn with_vm_hooks_ctx_2<R, F>(handle1: Self::HandleType, handle2: Self::HandleType, f: F) -> R
    where
        F: FnOnce(&mut dyn VMHooks) -> Result<R, VMHooksEarlyExit>,
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
        F: FnOnce(&mut dyn VMHooks) -> Result<R, VMHooksEarlyExit>,
    {
        assert_handles_on_same_context(&handle1, &handle2);
        assert_handles_on_same_context(&handle1, &handle3);
        Self::with_vm_hooks_ctx_1(handle1, f)
    }

    fn assert_live_handle(handle: &Self::HandleType) {
        if !handle.is_on_current_context() {
            ContractDebugInstanceState::early_exit_panic(
                VMHooksEarlyExit::new(ReturnCode::DebugApiError.as_u64())
                    .with_const_message(err_msg::DEBUG_API_ERR_HANDLE_STALE),
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

fn assert_handles_on_same_context(handle1: &DebugHandle, handle2: &DebugHandle) {
    if !handle1.is_on_same_context(handle2) {
        ContractDebugInstanceState::early_exit_panic(
            VMHooksEarlyExit::new(ReturnCode::DebugApiError.as_u64())
                .with_const_message(err_msg::DEBUG_API_ERR_HANDLE_CONTEXT_MISMATCH),
        );
    }
}
