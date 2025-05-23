use std::sync::{Arc, Mutex};

use multiversx_chain_vm_executor::{
    Executor, OpcodeCost, VMHooksEarlyExit, VMHooksLegacy, VMHooksLegacyAdapter,
    VMHooksSetEarlyExit,
};
use multiversx_chain_vm_executor_wasmer::new_traits::{
    WasmerProdExecutor, WasmerProdInstanceState, WasmerProdRuntimeRef,
};

use crate::host::{
    runtime::RuntimeWeakRef,
    vm_hooks::{TxVMHooksContext, VMHooksDispatcher},
};

pub fn new_prod_executor(runtime_ref: RuntimeWeakRef) -> Box<dyn Executor + Send + Sync> {
    Box::new(WasmerProdExecutor::new(Box::new(runtime_ref)))
}

impl VMHooksSetEarlyExit for VMHooksDispatcher<TxVMHooksContext<WasmerProdInstanceState>> {
    fn set_early_exit(&self, early_exit: VMHooksEarlyExit) {
        self.handler
            .context
            .instance_state_ref
            .set_early_exit(early_exit);
    }
}

impl WasmerProdRuntimeRef for RuntimeWeakRef {
    fn vm_hooks(&self, instance_state: WasmerProdInstanceState) -> Box<dyn VMHooksLegacy> {
        let runtime = self.upgrade();
        let tx_context_ref = runtime.get_executor_context();
        let vh_handler = TxVMHooksContext::new(tx_context_ref, instance_state);
        Box::new(VMHooksLegacyAdapter::new(VMHooksDispatcher::new(
            vh_handler,
        )))
    }

    fn opcode_cost(&self) -> std::sync::Arc<std::sync::Mutex<OpcodeCost>> {
        let runtime = self.upgrade();
        Arc::new(Mutex::new(
            runtime.vm_ref.gas_schedule.wasm_opcode_cost.clone(),
        ))
    }
}
