use super::*;

use multiversx_chain_vm::tx_execution::RuntimeWeakRef;
use multiversx_chain_vm_executor::{
    CompilationOptions, Executor, ExecutorError, Instance, OpcodeCost,
};
use std::fmt;

pub struct DebugSCExecutor {
    runtime_ref: RuntimeWeakRef,
    contract_map_ref: ContractMapRef,
}

unsafe impl Sync for DebugSCExecutor {} // TODO: temp
unsafe impl Send for DebugSCExecutor {} // TODO: temp

impl fmt::Debug for DebugSCExecutor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DebugSCExecutor").finish()
    }
}

impl DebugSCExecutor {
    pub fn new(runtime_ref: RuntimeWeakRef, contract_map_ref: ContractMapRef) -> Self {
        DebugSCExecutor {
            runtime_ref,
            contract_map_ref,
            // next_tx_context: None,
        }
    }
}

impl Executor for DebugSCExecutor {
    fn set_vm_hooks_ptr(
        &mut self,
        _vm_hooks_ptr: *mut std::ffi::c_void,
    ) -> Result<(), ExecutorError> {
        todo!()
    }

    fn set_opcode_cost(&mut self, _opcode_cost: &OpcodeCost) -> Result<(), ExecutorError> {
        Ok(())
    }

    fn new_instance(
        &self,
        wasm_bytes: &[u8],
        _compilation_options: &CompilationOptions,
    ) -> Result<Box<dyn Instance>, ExecutorError> {
        let contract_container = self.contract_map_ref.lock().get_contract(wasm_bytes);
        let tx_context_ref = self.runtime_ref.upgrade().current_context();
        let instance = DebugSCInstance::new(tx_context_ref, contract_container);
        Ok(Box::new(instance))
    }

    fn new_instance_from_cache(
        &self,
        _cache_bytes: &[u8],
        _compilation_options: &CompilationOptions,
    ) -> Result<Box<dyn Instance>, ExecutorError> {
        panic!("DebugSCExecutor new_instance_from_cache not supported")
    }
}
