use multiversx_chain_vm_executor::{Executor, OpcodeCost, VMHooks};
use multiversx_chain_vm_executor_wasmer_experimental::{
    ExperimentalExecutor, ExperimentalExecutorRuntimeRef, ExperimentalInstanceState,
    ExperimentalVMHooksBuilder,
};
use std::sync::Arc;

use crate::host::{
    context::TxContextRef,
    runtime::RuntimeWeakRef,
    vm_hooks::{TxVMHooksContext, VMHooksDispatcher},
};

pub fn new_experimental_executor(runtime_ref: RuntimeWeakRef) -> Box<dyn Executor + Send + Sync> {
    Box::new(ExperimentalExecutor::new(Box::new(runtime_ref)))
}

impl ExperimentalExecutorRuntimeRef for RuntimeWeakRef {
    fn vm_hooks_builder(&self) -> Box<dyn ExperimentalVMHooksBuilder> {
        let runtime = self.upgrade();
        let tx_context_ref = runtime.get_executor_context();
        Box::new(ExperimentalTxContextVMHooksBuilder::new(tx_context_ref))
    }

    fn opcode_cost(&self) -> Arc<OpcodeCost> {
        let runtime = self.upgrade();
        Arc::new(runtime.vm_ref.gas_schedule.wasm_opcode_cost.clone())
    }
}

/// Combines the VM's `TxContextRef` with the `ExperimentalInstanceState` from the executor,
/// to create the `VMHooks.
pub struct ExperimentalTxContextVMHooksBuilder {
    tx_context_ref: TxContextRef,
}

impl ExperimentalTxContextVMHooksBuilder {
    pub fn new(tx_context_ref: TxContextRef) -> Self {
        ExperimentalTxContextVMHooksBuilder { tx_context_ref }
    }
}

impl ExperimentalVMHooksBuilder for ExperimentalTxContextVMHooksBuilder {
    fn create_vm_hooks<'b, 'h>(
        &'b self,
        instance_state_ref: &'h mut ExperimentalInstanceState,
    ) -> Box<dyn VMHooks + 'h> {
        let vh_context = TxVMHooksContext::new(self.tx_context_ref.clone(), instance_state_ref);
        Box::new(VMHooksDispatcher::new(vh_context))
    }
}
