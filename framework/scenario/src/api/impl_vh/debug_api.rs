use multiversx_chain_vm::{
    executor::VMHooks,
    tx_mock::{StaticVarData, StaticVarStack, TxContextStack},
    vm_hooks::{TxContextWrapper, VMHooksDispatcher},
};

use super::{VMHooksApi, VMHooksApiBackend};

#[derive(Clone)]
pub struct DebugApiBackend;

impl VMHooksApiBackend for DebugApiBackend {
    fn with_vm_hooks<R, F>(f: F) -> R
    where
        F: FnOnce(&dyn VMHooks) -> R,
    {
        let top_context = TxContextStack::static_peek();
        let wrapper = TxContextWrapper::new(top_context);
        let dispatcher = VMHooksDispatcher::new(Box::new(wrapper));
        f(&dispatcher)
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

impl std::fmt::Debug for DebuggerApi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("DebugApi")
    }
}
