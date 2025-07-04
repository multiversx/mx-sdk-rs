use multiversx_chain_vm_executor::{InstanceState, VMHooksEarlyExit, VMHooksSetEarlyExit};

use super::{TxVMHooksContext, VMHooksDispatcher};

/// Allows external instance state types to define `set_early_exit`, and thus be usable in VM hooks adapters.
pub trait InstanceStateSetEarlyExit: InstanceState {
    fn set_early_exit(&self, early_exit: VMHooksEarlyExit);
}

impl<S> VMHooksSetEarlyExit for VMHooksDispatcher<TxVMHooksContext<S>>
where
    S: InstanceStateSetEarlyExit,
{
    fn set_early_exit(&self, early_exit: VMHooksEarlyExit) {
        self.handler
            .context
            .instance_state_ref
            .set_early_exit(early_exit);
    }
}
