use std::sync::{Arc, Mutex};

use multiversx_chain_vm_executor::{
    Executor, InstanceState, OpcodeCost, VMHooksEarlyExit, VMHooksLegacy, VMHooksLegacyAdapter,
};
use multiversx_chain_vm_executor_wasmer::new_traits::{
    WasmerProdExecutor, WasmerProdInstanceState, WasmerProdRuntimeRef,
};

use multiversx_chain_vm::host::{
    runtime::RuntimeWeakRef,
    vm_hooks::{InstanceStateSetEarlyExit, TxVMHooksContext, VMHooksDispatcher},
};

/// Creates a new Wasmer 2.2 executor for a given runtime reference.
pub fn new_prod_executor(runtime_ref: RuntimeWeakRef) -> Box<dyn Executor + Send + Sync> {
    Box::new(WasmerProdExecutor::new(Box::new(RuntimeRefAdapter(
        runtime_ref,
    ))))
}

/// Wrapper around RuntimeRefAdapter, which allows us to implement `RuntimeRefAdapter`,
/// while avoiding the orphan rule.
pub struct RuntimeRefAdapter(RuntimeWeakRef);

impl WasmerProdRuntimeRef for RuntimeRefAdapter {
    fn vm_hooks(&self, instance_state: WasmerProdInstanceState) -> Box<dyn VMHooksLegacy> {
        let runtime = self.0.upgrade();
        let tx_context_ref = runtime.get_executor_context();
        let instance_state_adapter = WasmerProdInstanceStateAdapter(instance_state);
        let vh_handler = TxVMHooksContext::new(tx_context_ref, instance_state_adapter);
        Box::new(VMHooksLegacyAdapter::new(VMHooksDispatcher::new(
            vh_handler,
        )))
    }

    fn opcode_cost(&self) -> std::sync::Arc<std::sync::Mutex<OpcodeCost>> {
        let runtime = self.0.upgrade();
        Arc::new(Mutex::new(
            runtime.vm_ref.gas_schedule.wasm_opcode_cost.clone(),
        ))
    }
}

/// Wrapper around WasmerProdInstanceState, which allows us to implement `InstanceStateSetEarlyExit`,
/// while avoiding the orphan rule.
pub struct WasmerProdInstanceStateAdapter(WasmerProdInstanceState);

impl InstanceState for WasmerProdInstanceStateAdapter {
    fn get_points_used(&mut self) -> Result<u64, multiversx_chain_vm_executor::ExecutorError> {
        self.0.get_points_used()
    }

    fn set_points_used(
        &mut self,
        points: u64,
    ) -> Result<(), multiversx_chain_vm_executor::ExecutorError> {
        self.0.set_points_used(points)
    }

    fn memory_load_to_slice(
        &self,
        mem_ptr: multiversx_chain_vm_executor::MemPtr,
        dest: &mut [u8],
    ) -> Result<(), multiversx_chain_vm_executor::ExecutorError> {
        self.0.memory_load_to_slice(mem_ptr, dest)
    }

    fn memory_load_owned(
        &self,
        mem_ptr: multiversx_chain_vm_executor::MemPtr,
        mem_length: multiversx_chain_vm_executor::MemLength,
    ) -> Result<Vec<u8>, multiversx_chain_vm_executor::ExecutorError> {
        self.0.memory_load_owned(mem_ptr, mem_length)
    }

    fn memory_store(
        &self,
        mem_ptr: multiversx_chain_vm_executor::MemPtr,
        data: &[u8],
    ) -> Result<(), multiversx_chain_vm_executor::ExecutorError> {
        self.0.memory_store(mem_ptr, data)
    }
}

impl InstanceStateSetEarlyExit for WasmerProdInstanceStateAdapter {
    fn set_early_exit(&self, early_exit: VMHooksEarlyExit) {
        self.0.set_early_exit(early_exit);
    }
}
