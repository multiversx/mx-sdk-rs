use std::rc::Rc;

use multiversx_chain_vm::{
    executor::VMHooks,
    tx_mock::{StaticVarData, StaticVarStack, TxContext, TxContextStack},
    vm_hooks::{TxContextWrapper, VMHooksDispatcher},
};

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
        let wrapper = TxContextWrapper::new(top_context);
        let dispatcher = VMHooksDispatcher::new(Box::new(wrapper));
        f(&dispatcher)
    }

    fn with_vm_hooks_ctx_1<R, F>(handle: Self::HandleType, f: F) -> R
    where
        F: FnOnce(&dyn VMHooks) -> R,
    {
        let wrapper = TxContextWrapper::new(handle.context);
        let dispatcher = VMHooksDispatcher::new(Box::new(wrapper));
        f(&dispatcher)
    }

    fn with_vm_hooks_ctx_2<R, F>(handle1: Self::HandleType, handle2: Self::HandleType, f: F) -> R
    where
        F: FnOnce(&dyn VMHooks) -> R,
    {
        assert!(
            Rc::ptr_eq(&handle1.context, &handle2.context),
            "VMHooksApi misuse: operation called with handles from 2 different contexts"
        );
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
        assert!(
            Rc::ptr_eq(&handle1.context, &handle2.context),
            "VMHooksApi misuse: operation called with handles from 2 different contexts"
        );
        assert!(
            Rc::ptr_eq(&handle1.context, &handle3.context),
            "VMHooksApi misuse: operation called with handles from 2 different contexts"
        );
        Self::with_vm_hooks_ctx_1(handle1, f)
    }

    fn assert_live_handle(handle: &Self::HandleType) {
        handle.assert_current_context()
    }

    fn with_static_data<R, F>(f: F) -> R
    where
        F: FnOnce(&StaticVarData) -> R,
    {
        let top_context = StaticVarStack::static_peek();
        f(&top_context)
    }
}

/// TODO: rename to DebugApi
pub type DebuggerApi = VMHooksApi<DebugApiBackend>;

impl DebuggerApi {
    pub fn dummy() {
        let tx_context = TxContext::dummy();
        let tx_context_rc = Rc::new(tx_context);
        // TODO: WARNING: this does not clean up after itself, must fix!!!
        TxContextStack::static_push(tx_context_rc);
        StaticVarStack::static_push();
    }
}

impl std::fmt::Debug for DebuggerApi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("DebugApi")
    }
}