use multiversx_chain_vm::{executor::VMHooks, tx_mock::StaticVarData};
use multiversx_sc::api::HandleConstraints;

pub trait VMHooksApiBackend: Clone + 'static {
    /// We use a single handle type for all handles.
    type HandleType: HandleConstraints;

    /// All communication with the VM happens via this method.
    fn with_vm_hooks<R, F>(f: F) -> R
    where
        F: FnOnce(&dyn VMHooks) -> R;

    /// Static data does not belong to the VM, or to the VM hooks. It belongs to the contract only.
    fn with_static_data<R, F>(f: F) -> R
    where
        F: FnOnce(&StaticVarData) -> R;
}
