use multiversx_chain_vm::executor::VMHooks;
use multiversx_sc::api::HandleConstraints;

use crate::debug_executor::StaticVarData;

pub trait VMHooksApiBackend: Clone + Send + Sync + 'static {
    /// We use a single handle type for all handles.
    type HandleType: HandleConstraints;

    /// All communication with the VM happens via this method.
    fn with_vm_hooks<R, F>(f: F) -> R
    where
        F: FnOnce(&dyn VMHooks) -> R;

    fn with_vm_hooks_ctx_1<R, F>(_handle: Self::HandleType, f: F) -> R
    where
        F: FnOnce(&dyn VMHooks) -> R,
    {
        Self::with_vm_hooks(f)
    }

    fn with_vm_hooks_ctx_2<R, F>(_handle1: Self::HandleType, _handle2: Self::HandleType, f: F) -> R
    where
        F: FnOnce(&dyn VMHooks) -> R,
    {
        Self::with_vm_hooks(f)
    }

    fn with_vm_hooks_ctx_3<R, F>(
        _handle1: Self::HandleType,
        _handle2: Self::HandleType,
        _handle3: Self::HandleType,
        f: F,
    ) -> R
    where
        F: FnOnce(&dyn VMHooks) -> R,
    {
        Self::with_vm_hooks(f)
    }

    fn assert_live_handle(_handle: &Self::HandleType) {
        // by default, no check
    }

    /// Static data does not belong to the VM, or to the VM hooks. It belongs to the contract only.
    fn with_static_data<R, F>(f: F) -> R
    where
        F: FnOnce(&StaticVarData) -> R;
}
