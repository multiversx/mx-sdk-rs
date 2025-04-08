use std::rc::Rc;

use multiversx_chain_vm_executor::{InstanceState, VMHooks, VMHooksBuilder};

use crate::host::context::TxContextRef;

use super::{TxContextVMHooksHandler, VMHooksDispatcher};

pub struct TxContextVMHooksBuilder {
    tx_context_ref: TxContextRef,
}

impl TxContextVMHooksBuilder {
    pub fn new(tx_context_ref: TxContextRef) -> Self {
        TxContextVMHooksBuilder { tx_context_ref }
    }
}

impl VMHooksBuilder for TxContextVMHooksBuilder {
    fn create_vm_hooks(&self, instance_state_ref: Rc<dyn InstanceState>) -> Box<dyn VMHooks> {
        let handler = TxContextVMHooksHandler::new(self.tx_context_ref.clone(), instance_state_ref);
        Box::new(VMHooksDispatcher::new(Box::new(handler)))
    }
}
