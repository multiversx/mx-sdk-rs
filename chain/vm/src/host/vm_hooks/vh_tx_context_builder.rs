use multiversx_chain_vm_executor::{InstanceState, VMHooks, VMHooksBuilder};

use crate::host::context::TxContextRef;

use super::{TxContextVMHooksHandler2, VMHooksDispatcher};

pub struct TxContextVMHooksBuilder {
    tx_context_ref: TxContextRef,
}

impl TxContextVMHooksBuilder {
    pub fn new(tx_context_ref: TxContextRef) -> Self {
        TxContextVMHooksBuilder { tx_context_ref }
    }
}

// impl Debug for TxContextVMHooksBuilder {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         f.debug_struct("TxContextVMHooksBuilder").finish()
//     }
// }

impl VMHooksBuilder for TxContextVMHooksBuilder {
    fn create_vm_hooks<'a, 'b, 'c>(
        &'a self,
        instance_state_ref: Box<dyn InstanceState + 'b>,
    ) -> Box<dyn VMHooks + 'c> {
        let handler = TxContextVMHooksHandler2 {
            tx_context_ref: self.tx_context_ref.clone(),
            instance_ref: instance_state_ref,
        };
        Box::new(VMHooksDispatcher::new(Box::new(handler)))
    }
}
